use engine::board::board_representation::Color;
use engine::eval::{king_safety_net::HIDDEN_LAYER_SIZE, trace::SAFETY_TRACE_LEN};

use crate::tuner_val::S;

use crate::tuning::Entry;

struct NetInfo {
    hidden_weights: [[S; HIDDEN_LAYER_SIZE]; SAFETY_TRACE_LEN],
    hidden_biases: [S; HIDDEN_LAYER_SIZE],
    hidden_sums: [S; HIDDEN_LAYER_SIZE],
    output_weights: [S; HIDDEN_LAYER_SIZE],
    output_bias: S,
    output_sum: S,
}

impl NetInfo {
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

pub struct Net {
    params: NetInfo,
    partials: NetInfo,
}

impl Net {
    fn new() -> Self {
        Self {
            params: NetInfo::new(),
            partials: NetInfo::new(),
        }
    }

    fn reset_sums(&mut self) {
        // we reset the activations to just the biases, so that they are
        // already accounted for when we calculate the sums
        self.params.hidden_sums = self.params.hidden_biases;
        self.params.output_sum = self.params.output_bias;
    }

    fn reset_partials(&mut self) {
        self.partials = NetInfo::new();
    }

    fn calculate(&mut self, entry: &Entry, color: Color) -> S {
        let params = &mut self.params;

        // calculate the weighted sums in the hidden layer (accumulator)
        for f in entry.safety_feature_vec[color.as_index()] {
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

    fn update_partials(&mut self, sign: f64) {

    }
}
