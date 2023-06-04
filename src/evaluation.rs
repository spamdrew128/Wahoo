use crate::{
    bitloop,
    board_representation::{Board, Color, Piece, Square, NUM_PIECES},
};

pub type EvalScore = i16;
pub const INF: EvalScore = i16::MAX - 10;

const PIECE_VALUES: [EvalScore; NUM_PIECES as usize] = [300, 320, 500, 900, 100, 0];

pub fn evaluate(board: &Board) -> EvalScore {
    let mut score: EvalScore = 0;
    for piece in Piece::LIST {
        let mut w_pieces = board.piece_bb(piece, Color::White);
        let mut b_pieces = board.piece_bb(piece, Color::Black);
        bitloop!(|_sq|, w_pieces, {
            score += PIECE_VALUES[piece.as_index()];
        });
        bitloop!(|_sq|, b_pieces, {
            score -= PIECE_VALUES[piece.as_index()];
        });
    }

    match board.color_to_move {
        Color::White => score,
        Color::Black => -score,
    }
}
