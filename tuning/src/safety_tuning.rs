use engine::board::board_representation::Color;
use engine::eval::{king_safety_net::HIDDEN_LAYER_SIZE, trace::SAFETY_TRACE_LEN};

use crate::tuner_val::S;

use crate::tuning::Entry;

struct LayerSums {
    hidden: [S; HIDDEN_LAYER_SIZE],
    output: S,
}

impl LayerSums {
    fn new() -> Self {
        Self {
            hidden: [S::new(0.0, 0.0); HIDDEN_LAYER_SIZE],
            output: S::new(0.0, 0.0),
        }
    }
}

#[derive(Clone)]
pub struct Net {
    pub hidden_weights: [[S; HIDDEN_LAYER_SIZE]; SAFETY_TRACE_LEN],
    pub hidden_biases: [S; HIDDEN_LAYER_SIZE],
    pub output_weights: [S; HIDDEN_LAYER_SIZE],
    pub output_bias: S,
}

impl Net {
    pub fn new(init: f64) -> Self {
        Self {
            hidden_weights: [[S::new(init, init); HIDDEN_LAYER_SIZE]; SAFETY_TRACE_LEN],
            hidden_biases: [S::new(init, init); HIDDEN_LAYER_SIZE],
            output_weights: [S::new(init, init); HIDDEN_LAYER_SIZE],
            output_bias: S::new(init, init),
        }
    }

    fn calculate_color(&self, sums: &mut LayerSums, entry: &Entry, color: Color) -> S {
        // calculate the weighted sums in the hidden layer (accumulator)
        for f in &entry.safety_feature_vec[color.as_index()] {
            let weights = self.hidden_weights[f.index];

            for (i, &weight) in weights.iter().enumerate() {
                sums.hidden[i] += weight * f64::from(f.value);
            }
        }

        // calculate the output sum using previous layer activations
        for (i, &weight) in self.output_weights.iter().enumerate() {
            sums.output += weight * sums.hidden[i].activation();
        }

        // return activated output
        sums.output.activation()
    }

    fn update_partials(
        &self,
        sums: &mut LayerSums,
        partials: &mut Net,
        entry: &Entry,
        color: Color,
        sign: f64,
    ) {
        // update output bias partial
        let output_bias_partial = sums.output.activation_prime() * sign;
        partials.output_bias += output_bias_partial;

        // update output weights partials
        for (i, &hidden_sum) in sums.hidden.iter().enumerate() {
            partials.output_weights[i] += output_bias_partial * hidden_sum.activation();
        }

        // find output activation partials
        let mut output_activation_partials = [S::new(0.0, 0.0); HIDDEN_LAYER_SIZE];
        for (i, &weight) in self.output_weights.iter().enumerate() {
            output_activation_partials[i] = output_bias_partial * weight;
        }

        // update hidden bias partials
        let mut hidden_bias_partials = [S::new(0.0, 0.0); HIDDEN_LAYER_SIZE];
        for (i, &output_partial) in output_activation_partials.iter().enumerate() {
            hidden_bias_partials[i] = sums.hidden[i].activation() * output_partial;
            partials.hidden_biases[i] += hidden_bias_partials[i];
        }

        // update hidden weight partials
        for f in &entry.safety_feature_vec[color.as_index()] {
            let weight_partials = &mut partials.hidden_weights[f.index];

            for (i, &bias_partial) in hidden_bias_partials.iter().enumerate() {
                weight_partials[i] += bias_partial * f64::from(f.value);
            }
        }
    }

    pub fn calc_and_compute_partials(&self, entry: &Entry) -> (S, Net) {
        let mut partials = Net::new(0.0);
        let (mut w_sums, mut b_sums) = (LayerSums::new(), LayerSums::new());

        let mut score = self.calculate_color(&mut w_sums, entry, Color::White);
        self.update_partials(&mut w_sums, &mut partials, entry, Color::White, 1.0);

        score -= self.calculate_color(&mut b_sums, entry, Color::Black);
        self.update_partials(&mut b_sums, &mut partials, entry, Color::Black, -1.0);

        (score, partials)
    }

    pub fn calc_both_sides(&self, entry: &Entry) -> S {
        let (mut w_sums, mut b_sums) = (LayerSums::new(), LayerSums::new());

        self.calculate_color(&mut w_sums, entry, Color::White)
            - self.calculate_color(&mut b_sums, entry, Color::Black)
    }
}
