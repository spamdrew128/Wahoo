use engine::board::board_representation::Color;
use engine::eval::{king_safety_net::HIDDEN_LAYER_SIZE, trace::SAFETY_TRACE_LEN};

use rand::{thread_rng, Rng};

use crate::tuner_val::S;

use crate::tuning::Entry;

#[derive(Debug, PartialEq)]
struct LayerSums {
    hidden: [S; HIDDEN_LAYER_SIZE],
    output: S,
}

impl LayerSums {
    fn new(weights: &Net) -> Self {
        Self {
            hidden: weights.hidden_biases,
            output: weights.output_bias,
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

    pub fn calc_and_compute_partials(&self, partials: &mut Self, entry: &Entry) -> S {
        let (mut w_sums, mut b_sums) = (LayerSums::new(self), LayerSums::new(self));

        let mut score = self.calculate_color(&mut w_sums, entry, Color::White);
        self.update_partials(&mut w_sums, partials, entry, Color::White, 1.0);

        score -= self.calculate_color(&mut b_sums, entry, Color::Black);
        self.update_partials(&mut b_sums, partials, entry, Color::Black, -1.0);

        score
    }

    pub fn calc_both_sides(&self, entry: &Entry) -> S {
        let (mut w_sums, mut b_sums) = (LayerSums::new(self), LayerSums::new(self));

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
    use engine::board::board_representation::{Board, Color, START_FEN};

    use crate::{
        tuner_val::S,
        tuning::{Entry, Feature},
    };

    use super::{LayerSums, Net};

    fn net_compare<F>(net1: &Net, net2: &Net, comp: F)
    where
        F: Fn(S, S) -> bool,
    {
        for (&a, &b) in net1
            .hidden_weights
            .iter()
            .flatten()
            .zip(net2.hidden_weights.iter().flatten())
        {
            if !comp(a, b) {
                println!("hidden_weights {:?} {:?}", a, b);
            }
        }

        for (&a, &b) in net1.hidden_biases.iter().zip(net2.hidden_biases.iter()) {
            if !comp(a, b) {
                println!("hidden_biases {:?} {:?}", a, b);
            }
        }

        for (&a, &b) in net1.output_weights.iter().zip(net2.output_weights.iter()) {
            if !comp(a, b) {
                println!("output_weights {:?} {:?}", a, b);
            }
        }

        if !comp(net1.output_bias, net2.output_bias) {
            println!("output_bias {:?} {:?}", net1.output_bias, net2.output_bias);
        }
    }

    #[test]
    fn sign_works() {
        let board = Board::from_fen("B2r2k1/3p1p2/p4PpB/1p3b2/8/2Nq2PP/PP2R1NK/3R4 b - - 2 23");
        let entry = Entry::new(&board, 0.5);

        let net = Net::new_randomized();

        let (mut pos_sums, mut neg_sums) = (LayerSums::new(&net), LayerSums::new(&net));

        let mut pos_partials = Net::new();
        net.calculate_color(&mut pos_sums, &entry, Color::White);
        net.update_partials(&mut pos_sums, &mut pos_partials, &entry, Color::White, 1.0);

        let mut neg_partials = Net::new();
        net.calculate_color(&mut neg_sums, &entry, Color::White);
        net.update_partials(&mut neg_sums, &mut neg_partials, &entry, Color::White, -1.0);

        net_compare(&pos_partials, &neg_partials, |a, b| a == -b);
    }

    #[test]
    fn example_expected_output() {
        let some_board = Board::from_fen(START_FEN);
        let mut entry = Entry::new(&some_board, 0.5);
        let color = Color::White;

        let f1 = Feature::new(1, 0);
        let f2 = Feature::new(2, 13);
        let f3 = Feature::new(1, 15);
        entry.safety_feature_vec[color.as_index()] = vec![f1, f2, f3];

        let mut net = Net::new_randomized();
        net.hidden_weights[f1.index] = [
            S::new(0.2, 0.0),
            S::new(-0.1, 0.0),
            S::new(0.0, 0.0),
            S::new(0.01, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
        ];

        net.hidden_weights[f2.index] = [
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.1, 0.0),
            S::new(0.2, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
        ];

        net.hidden_weights[f3.index] = [
            S::new(0.7, 0.0),
            S::new(-0.2, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
        ];

        net.hidden_biases = [
            S::new(-0.1, 0.0),
            S::new(0.2, 0.0),
            S::new(0.0, 0.0),
            S::new(0.3, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
        ];

        net.output_weights = [
            S::new(0.1, 0.0),
            S::new(-0.1, 0.0),
            S::new(0.2, 0.0),
            S::new(-0.3, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
        ];

        net.output_bias = S::new(0.4, 0.0);

        // OUTPUTS
        let mut sums = LayerSums::new(&net);
        let output = net.calculate_color(&mut sums, &entry, Color::White);

        let expected_output = S::new(0.32077_f64.powi(2), 0.0);
        assert_eq!(expected_output, output);

        // PARTIALS
        let mut expected_partials = Net::new();
        expected_partials.output_bias = S::new(2.0 * 0.32077, 0.0);

        expected_partials.output_weights = [
            expected_partials.output_bias * 0.8_f64.powi(2),
            S::new(0.0, 0.0),
            expected_partials.output_bias * 0.2_f64.powi(2),
            expected_partials.output_bias * 0.71_f64.powi(2),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
        ];

        let activation_partials = [
            expected_partials.output_bias * 0.1,
            expected_partials.output_bias * -0.1,
            expected_partials.output_bias * 0.2,
            expected_partials.output_bias * -0.3,
        ];

        expected_partials.hidden_biases = [
            activation_partials[0] * 2.0 * 0.8,
            activation_partials[1] * 2.0 * 0.0,
            activation_partials[2] * 2.0 * 0.2,
            activation_partials[3] * 2.0 * 0.71,
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
        ];

        expected_partials.hidden_weights[f1.index] = [
            f64::from(f1.value) * expected_partials.hidden_biases[0],
            f64::from(f1.value) * expected_partials.hidden_biases[1],
            f64::from(f1.value) * expected_partials.hidden_biases[2],
            f64::from(f1.value) * expected_partials.hidden_biases[3],
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
        ];

        expected_partials.hidden_weights[f2.index] = [
            f64::from(f2.value) * expected_partials.hidden_biases[0],
            f64::from(f2.value) * expected_partials.hidden_biases[1],
            f64::from(f2.value) * expected_partials.hidden_biases[2],
            f64::from(f2.value) * expected_partials.hidden_biases[3],
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
        ];

        expected_partials.hidden_weights[f3.index] = [
            f64::from(f3.value) * expected_partials.hidden_biases[0],
            f64::from(f3.value) * expected_partials.hidden_biases[1],
            f64::from(f3.value) * expected_partials.hidden_biases[2],
            f64::from(f3.value) * expected_partials.hidden_biases[3],
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
            S::new(0.0, 0.0),
        ];

        let mut partials = Net::new();
        net.update_partials(&mut sums, &mut partials, &entry, Color::White, 1.0);

        net_compare(&partials, &expected_partials, |a, b| {
            let diff = a - b;
            diff.mg().abs() < 0.001 && diff.eg().abs() < 0.001
        });
    }
}
