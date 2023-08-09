use engine::board::board_representation::Color;
use engine::eval::{king_safety_net::HIDDEN_LAYER_SIZE, trace::SAFETY_TRACE_LEN};

use crate::tuner_val::S;

use crate::tuning::Entry;

struct NetInfo {
    hidden_weights: [[S; HIDDEN_LAYER_SIZE]; SAFETY_TRACE_LEN],
    hidden_biases: [S; HIDDEN_LAYER_SIZE],
    hidden_activations: [S; HIDDEN_LAYER_SIZE],
    output_weights: [S; HIDDEN_LAYER_SIZE],
    output_bias: S,
    output_activation: S,
}

impl NetInfo {
    fn new() -> Self {
        Self {
            hidden_weights: [[S::new(0.0, 0.0); HIDDEN_LAYER_SIZE]; SAFETY_TRACE_LEN],
            hidden_biases: [S::new(0.0, 0.0); HIDDEN_LAYER_SIZE],
            hidden_activations: [S::new(0.0, 0.0); HIDDEN_LAYER_SIZE],
            output_weights: [S::new(0.0, 0.0); HIDDEN_LAYER_SIZE],
            output_bias: S::new(0.0, 0.0),
            output_activation: S::new(0.0, 0.0),
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

    fn reset_activations(&mut self) {
        self.params.hidden_activations = [S::new(0.0, 0.0); HIDDEN_LAYER_SIZE];
        self.params.output_activation = S::new(0.0, 0.0);
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
                params.hidden_activations[i] += weight * f64::from(f.value);
            }
        }

        // pass each hidden layer sum through the activation function
        params.hidden_activations.iter_mut().for_each(|s| {
            *s = s.activation();
        });

        // calculate the output sum using previous layer activations
        for (i, &weight) in params.output_weights.iter().enumerate() {
            params.output_activation += weight * params.hidden_activations[i];
        }

        // pass output through the activation function
        params.output_activation = params.output_activation.activation();

        // return activated output
        self.params.output_activation
    }
}
