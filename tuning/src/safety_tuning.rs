use engine::eval::{king_safety_net::HIDDEN_LAYER_SIZE, trace::SAFETY_TRACE_LEN};

use crate::tuner_val::S;

pub struct Net {
    hidden_weights: [[S; HIDDEN_LAYER_SIZE]; SAFETY_TRACE_LEN],
    hidden_biases: [S; HIDDEN_LAYER_SIZE],
    hidden_activations: [S; HIDDEN_LAYER_SIZE],
    output_weights: [S; HIDDEN_LAYER_SIZE],
    output_bias: S,
    output_activation: S,
}

impl Net {
    pub fn new() -> Self {
        Self {
            hidden_weights: [[S::new(0.0 ,0.0); HIDDEN_LAYER_SIZE]; SAFETY_TRACE_LEN],
            hidden_biases: [S::new(0.0 ,0.0); HIDDEN_LAYER_SIZE],
            hidden_activations: [S::new(0.0 ,0.0); HIDDEN_LAYER_SIZE],
            output_weights: [S::new(0.0 ,0.0); HIDDEN_LAYER_SIZE],
            output_bias: S::new(0.0 ,0.0),
            output_activation: S::new(0.0 ,0.0),
        }
    }
}