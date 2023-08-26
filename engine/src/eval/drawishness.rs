use crate::board::board_representation::{Bitboard, Board, Color, Piece};

use super::{
    eval_constants::OPPOSITE_BISHOPS,
    evaluation::{ScoreTuple, DRAWISHNESS_SCALE},
};

impl ScoreTuple {
    fn drawishness_adjustment(&mut self, board: &Board) {
        let mut drawishness = Self::new(DRAWISHNESS_SCALE, DRAWISHNESS_SCALE);

        let w_bishops = board.piece_bb(Piece::BISHOP, Color::White);
        let b_bishops = board.piece_bb(Piece::BISHOP, Color::Black);

        let light_sq_bishops = w_bishops.union(b_bishops).intersection(Bitboard::LIGHT_SQ);

        if w_bishops.popcount() == 1
            && b_bishops.popcount() == 1
            && light_sq_bishops.popcount() == 1
        {
            drawishness += OPPOSITE_BISHOPS;
        }
    }
}
