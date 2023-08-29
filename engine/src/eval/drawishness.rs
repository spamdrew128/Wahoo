use crate::board::board_representation::Board;

use super::evaluation::{Phase, ScoreTuple};

impl ScoreTuple {
    pub fn drawishness_adjustment(&mut self, board: &Board, phase: Phase) {
        if phase == 2 {
            *self = *self / 2;
        }
    }
}
