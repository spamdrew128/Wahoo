use crate::{
    bitloop,
    board_representation::{Board, Color, Piece, Square, NUM_PIECES},
    search::MAX_PLY,
};

pub type Phase = u8;
pub const PHASE_MAX: Phase = 24;
pub const NUM_PHASES: usize = 2;

pub type EvalScore = i16;
pub const INF: EvalScore = i16::MAX - 10;
pub const EVAL_MAX: EvalScore = INF - 1;
pub const MATE_THRESHOLD: EvalScore = EVAL_MAX - (MAX_PLY as i16);

const PIECE_VALUES: [EvalScore; NUM_PIECES as usize] = [300, 320, 500, 900, 100, 0];

pub fn phase(board: &Board) -> Phase {
    let phase = (board.pieces[Piece::KNIGHT.as_index()].popcount()
        + board.pieces[Piece::BISHOP.as_index()].popcount()
        + board.pieces[Piece::ROOK.as_index()].popcount() * 2
        + board.pieces[Piece::QUEEN.as_index()].popcount() * 4) as u8;
    phase.min(PHASE_MAX)
}

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
