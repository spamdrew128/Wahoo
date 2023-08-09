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

    fn reset(&mut self) {
        self.params.hidden_activations = [S::new(0.0, 0.0); HIDDEN_LAYER_SIZE];
        self.params.output_activation = S::new(0.0, 0.0);

        self.partials = NetInfo::new();
    }
}
