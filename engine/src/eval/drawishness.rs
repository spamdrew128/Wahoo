use crate::{
    board::board_representation::{Bitboard, Board, Color, Piece, NUM_PIECES},
    eval::trace::OppBishop,
    trace_drawishness_update,
};

use super::{
    eval_constants::OPPOSITE_BISHOPS,
    evaluation::{ScoreTuple, DRAWISHNESS_SCALE},
    trace::Trace,
};

fn opp_bishops<const TRACE: bool>(board: &Board, t: &mut Trace) -> ScoreTuple {
    let w_bishops = board.piece_bb(Piece::BISHOP, Color::White);
    let b_bishops = board.piece_bb(Piece::BISHOP, Color::Black);

    let light_sq_bishops = w_bishops.union(b_bishops).intersection(Bitboard::LIGHT_SQ);

    if w_bishops.popcount() == 1 && b_bishops.popcount() == 1 && light_sq_bishops.popcount() == 1 {
        if TRACE {
            trace_drawishness_update!(t, OppBishop, (), 1);
        }

        return OPPOSITE_BISHOPS;
    }

    ScoreTuple::new(0, 0)
}

fn material_imbalance<const TRACE: bool>(board: &Board, t: &mut Trace) -> ScoreTuple {
    let imbalance = ScoreTuple::new(0, 0);
    for piece in Piece::LIST.iter().take(NUM_PIECES as usize - 1) { // exclude king
    }

    imbalance
}

impl ScoreTuple {
    pub fn drawishness_adjustment<const TRACE: bool>(self, board: &Board, t: &mut Trace) -> Self {
        let mut drawishness = Self::new(DRAWISHNESS_SCALE, DRAWISHNESS_SCALE);

        drawishness += opp_bishops::<TRACE>(board, t);

        drawishness = drawishness.clamp(DRAWISHNESS_SCALE / 4, DRAWISHNESS_SCALE);
        self * drawishness / DRAWISHNESS_SCALE
    }
}
