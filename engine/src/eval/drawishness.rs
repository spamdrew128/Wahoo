use crate::board::board_representation::{Bitboard, Board, Color, Piece};

use super::evaluation::{EvalScore, Phase};

const fn opposite_bishops(board: &Board) -> bool {
    let w_bishops = board.piece_bb(Piece::BISHOP, Color::White);
    let b_bishops = board.piece_bb(Piece::BISHOP, Color::Black);

    let light_sq_bishops = w_bishops.union(b_bishops).intersection(Bitboard::LIGHT_SQ);

    w_bishops.popcount() == 1 && b_bishops.popcount() == 1 && light_sq_bishops.popcount() == 1
}

pub const fn drawishness_adjustment(eval: EvalScore, board: &Board, phase: Phase) -> EvalScore {
    let mut score = eval;

    if phase == 2 && opposite_bishops(board) {
        score /= 4;
    }

    score
}
