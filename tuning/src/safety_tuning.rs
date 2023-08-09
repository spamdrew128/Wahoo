use engine::board::board_representation::Color;
use engine::eval::{king_safety_net::HIDDEN_LAYER_SIZE, trace::SAFETY_TRACE_LEN};

use crate::tuner_val::S;

use crate::tuning::Entry;

struct NetParams {
    hidden_weights: [[S; HIDDEN_LAYER_SIZE]; SAFETY_TRACE_LEN],
    hidden_biases: [S; HIDDEN_LAYER_SIZE],
    hidden_sums: [S; HIDDEN_LAYER_SIZE],
    output_weights: [S; HIDDEN_LAYER_SIZE],
    output_bias: S,
    output_sum: S,
}

impl NetParams {
    fn new() -> Self {
        Self {
            hidden_weights: [[S::new(0.0, 0.0); HIDDEN_LAYER_SIZE]; SAFETY_TRACE_LEN],
            hidden_biases: [S::new(0.0, 0.0); HIDDEN_LAYER_SIZE],
            hidden_sums: [S::new(0.0, 0.0); HIDDEN_LAYER_SIZE],
            output_weights: [S::new(0.0, 0.0); HIDDEN_LAYER_SIZE],
            output_bias: S::new(0.0, 0.0),
            output_sum: S::new(0.0, 0.0),
        }
    }
}

struct NetPartials {
    hidden_weights: [[S; HIDDEN_LAYER_SIZE]; SAFETY_TRACE_LEN],
    hidden_biases: [S; HIDDEN_LAYER_SIZE],
    output_weights: [S; HIDDEN_LAYER_SIZE],
    output_bias: S,
}

impl NetPartials {
    fn new() -> Self {
        Self {
            hidden_weights: [[S::new(0.0, 0.0); HIDDEN_LAYER_SIZE]; SAFETY_TRACE_LEN],
            hidden_biases: [S::new(0.0, 0.0); HIDDEN_LAYER_SIZE],
            output_weights: [S::new(0.0, 0.0); HIDDEN_LAYER_SIZE],
            output_bias: S::new(0.0, 0.0),
        }
    }
}

pub struct Net {
    params: NetParams,
    partials: NetPartials,
}

impl Net {
    fn new() -> Self {
        Self {
            params: NetParams::new(),
            partials: NetPartials::new(),
        }
    }

    fn reset_sums(&mut self) {
        // we reset the activations to just the biases, so that they are
        // already accounted for when we calculate the sums
        self.params.hidden_sums = self.params.hidden_biases;
        self.params.output_sum = self.params.output_bias;
    }

    fn reset_partials(&mut self) {
        self.partials = NetPartials::new();
    }

    pub fn calculate(&mut self, entry: &Entry, color: Color) -> S {
        self.reset_sums();
        let params = &mut self.params;

        // calculate the weighted sums in the hidden layer (accumulator)
        for f in &entry.safety_feature_vec[color.as_index()] {
            let weights = params.hidden_weights[f.index];

            for (i, &weight) in weights.iter().enumerate() {
                params.hidden_sums[i] += weight * f64::from(f.value);
            }
        }

        // calculate the output sum using previous layer activations
        for (i, &weight) in params.output_weights.iter().enumerate() {
            params.output_sum += weight * params.hidden_sums[i].activation();
        }

        // return activated output
        self.params.output_sum.activation()
    }

    fn update_partials(&mut self, entry: &Entry, color: Color, sign: f64) {
        let params = &mut self.params;
        let partials = &mut self.partials;

        // update output bias partial
        let output_bias_partial = params.output_sum.activation_prime() * sign;
        partials.output_bias += output_bias_partial;

        // update output weights partials
        for (i, &hidden_sum) in params.hidden_sums.iter().enumerate() {
            partials.output_weights[i] += output_bias_partial * hidden_sum.activation();
        }

        // find output activation partials
        let mut output_activation_partials = [S::new(0.0, 0.0); HIDDEN_LAYER_SIZE];
        for (i, &weight) in params.output_weights.iter().enumerate() {
            output_activation_partials[i] = output_bias_partial * weight;
        }

        // update hidden bias partials
        let mut hidden_bias_partials = [S::new(0.0, 0.0); HIDDEN_LAYER_SIZE];
        for (i, &output_partial) in output_activation_partials.iter().enumerate() {
            hidden_bias_partials[i] = params.hidden_sums[i].activation() * output_partial;
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

    pub fn calc_and_update_partials(&mut self, entry: &Entry) -> S {
        self.reset_partials();

        let mut score = self.calculate(entry, Color::White);
        self.update_partials(entry, Color::White, 1.0);

        score -= self.calculate(entry, Color::Black);
        self.update_partials(entry, Color::Black, -1.0);

        score
    }
}
