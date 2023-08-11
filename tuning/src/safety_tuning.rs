use engine::board::board_representation::Color;
use engine::eval::{king_safety_net::HIDDEN_LAYER_SIZE, trace::SAFETY_TRACE_LEN};

use rand::{thread_rng, Rng};

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

#[derive(Debug, Clone)]
pub struct Net {
    pub hidden_weights: [[S; HIDDEN_LAYER_SIZE]; SAFETY_TRACE_LEN],
    pub hidden_biases: [S; HIDDEN_LAYER_SIZE],
    pub output_weights: [S; HIDDEN_LAYER_SIZE],
    pub output_bias: S,
}

impl Net {
    pub fn new() -> Self {
        Self {
            hidden_weights: [[S::new(0.0, 0.0); HIDDEN_LAYER_SIZE]; SAFETY_TRACE_LEN],
            hidden_biases: [S::new(0.0, 0.0); HIDDEN_LAYER_SIZE],
            output_weights: [S::new(0.0, 0.0); HIDDEN_LAYER_SIZE],
            output_bias: S::new(0.0, 0.0),
        }
    }

    pub fn new_randomized() -> Self {
        let mut result = Self::new();

        let mut rng = thread_rng();
        result
            .hidden_weights
            .iter_mut()
            .flatten()
            .for_each(|s| *s = S::new(rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1)));

        result
            .hidden_biases
            .iter_mut()
            .for_each(|s| *s = S::new(rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1)));

        result
            .output_weights
            .iter_mut()
            .for_each(|s| *s = S::new(rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1)));

        result.output_bias = S::new(rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1));

        result
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
        let mut partials = Self::new();
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

    pub fn gradient_update(&mut self, partials: &Self, coeff: S) {
        for (grad, &partial) in self
            .hidden_weights
            .iter_mut()
            .flatten()
            .zip(partials.hidden_weights.iter().flatten())
        {
            *grad += coeff * partial;
        }

        for (grad, &partial) in self
            .hidden_biases
            .iter_mut()
            .zip(partials.hidden_biases.iter())
        {
            *grad += coeff * partial;
        }

        for (grad, &partial) in self
            .output_weights
            .iter_mut()
            .zip(partials.output_weights.iter())
        {
            *grad += coeff * partial;
        }

        self.output_bias += coeff * partials.output_bias;
    }
}

#[cfg(test)]
mod tests {
    use engine::board::board_representation::{Board, Color};

    use crate::{tuning::Entry, tuner_val::S};

    use super::{Net, LayerSums};

    #[test]
    fn sign_works() {
        let board = Board::from_fen("B2r2k1/3p1p2/p4PpB/1p3b2/8/2Nq2PP/PP2R1NK/3R4 b - - 2 23");
        let entry = Entry::new(&board, 0.5);

        let net = Net::new_randomized();

        let (mut pos_sums, mut neg_sums) = (LayerSums::new(), LayerSums::new());

        let mut pos_partials = Net::new();
        net.calculate_color(&mut pos_sums, &entry, Color::White);
        net.update_partials(&mut pos_sums, &mut pos_partials, &entry, Color::White, 1.0);

        let mut neg_partials = Net::new();
        net.calculate_color(&mut neg_sums, &entry, Color::White);
        net.update_partials(&mut neg_sums, &mut neg_partials, &entry, Color::White, -1.0);

        let mut total = S::new(0.0, 0.0);
        for (&pos_partial, &neg_partial) in pos_partials
            .hidden_weights
            .iter()
            .flatten()
            .zip(neg_partials.hidden_weights.iter().flatten())
        {
            assert_eq!(pos_partial, -neg_partial);
            total += pos_partial;
        }

        for (&pos_partial, &neg_partial) in pos_partials
            .hidden_biases
            .iter()
            .zip(neg_partials.hidden_biases.iter())
        {
            assert_eq!(pos_partial, -neg_partial);
            total += pos_partial;
        }

        for (&pos_partial, &neg_partial) in pos_partials
            .output_weights
            .iter()
            .zip(neg_partials.output_weights.iter())
        {
            assert_eq!(pos_partial, -neg_partial);
            total += pos_partial;
        }

        assert_eq!(pos_partials.output_bias, -neg_partials.output_bias);

        println!("{} {}", total.mg(), total.eg());
    }
}