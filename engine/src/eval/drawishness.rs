use crate::board::board_representation::{Board, Color, Piece};

use super::evaluation::{ScoreTuple, DRAWISHNESS_SCALE};

impl ScoreTuple {
    fn drawishness_adjustment(&mut self, board: &Board) {
        let drawishness = ScoreTuple::new(DRAWISHNESS_SCALE, DRAWISHNESS_SCALE);

        let w_bishops = board.piece_bb(Piece::BISHOP, Color::White);
        let b_bishops = board.piece_bb(Piece::BISHOP, Color::Black);

        if w_bishops.popcount() == 1 && b_bishops.popcount() == 1 && {

        }
    }
}
