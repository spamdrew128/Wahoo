use std::ops::{Add, AddAssign, Sub};

use crate::{
    bitloop,
    board_representation::{Board, Color, Piece, Square},
    eval_constants::{
        BISHOP_PAIR_BONUS, ISOLATED_PAWNS_PRT, MATERIAL_PSTS, PASSER_BLOCKERS_PRT, PASSER_PST,
        PHALANX_PAWNS_PRT, TEMPO_BONUS,
    },
    piece_loop_eval::mobility_threats_safety,
    search::MAX_PLY,
    trace::{
        color_adjust, empty_trace, BishopPair, IsolatedPawns, MaterialPst, Passer, PasserBlocker,
        PhalanxPawns, TempoBonus, Trace,
    },
    trace_update,
};

pub type Phase = u8;
pub const PHASE_MAX: Phase = 24;
pub const MG: usize = 0;
pub const EG: usize = 1;
pub const PHASES: [usize; NUM_PHASES] = [MG, EG];
pub const NUM_PHASES: usize = 2;

pub type EvalScore = i32;
pub const INF: EvalScore = (i16::MAX - 10) as i32;
pub const EVAL_MAX: EvalScore = INF - 1;
pub const MATE_THRESHOLD: EvalScore = EVAL_MAX - (MAX_PLY as i32);
pub const TB_WIN: EvalScore = MATE_THRESHOLD - 1000;
pub const TB_LOSS: EvalScore = -TB_WIN;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ScoreTuple(EvalScore, EvalScore);

impl ScoreTuple {
    pub const fn new(mg: EvalScore, eg: EvalScore) -> Self {
        Self(mg, eg)
    }

    const fn mg(self) -> EvalScore {
        self.0
    }

    const fn eg(self) -> EvalScore {
        self.1
    }

    pub const fn mult(self, multiplier: i32) -> Self {
        Self(self.0 * multiplier, self.1 * multiplier)
    }
}

impl Add for ScoreTuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for ScoreTuple {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1);
    }
}

impl Sub for ScoreTuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

pub fn phase(board: &Board) -> Phase {
    let phase = (board.pieces[Piece::KNIGHT.as_index()].popcount()
        + board.pieces[Piece::BISHOP.as_index()].popcount()
        + board.pieces[Piece::ROOK.as_index()].popcount() * 2
        + board.pieces[Piece::QUEEN.as_index()].popcount() * 4) as u8;
    phase.min(PHASE_MAX)
}

fn pst_eval<const TRACE: bool>(board: &Board, color: Color, t: &mut Trace) -> ScoreTuple {
    let mut score = ScoreTuple::new(0, 0);
    for piece in Piece::LIST {
        let mut pieces = board.piece_bb(piece, color);
        let pst = &MATERIAL_PSTS[piece.as_index()];

        bitloop!(|sq| pieces, {
            score += pst.access(color, sq);

            if TRACE {
                let sq = color_adjust(sq, color);
                trace_update!(t, MaterialPst, (piece, sq), color, 1);
            }
        });
    }
    score
}

fn bishop_pair<const TRACE: bool>(board: &Board, color: Color, t: &mut Trace) -> ScoreTuple {
    let bishops = board.piece_bb(Piece::BISHOP, color);
    if bishops.popcount() >= 2 {
        if TRACE {
            trace_update!(t, BishopPair, (), color, 1);
        }

        BISHOP_PAIR_BONUS
    } else {
        ScoreTuple::new(0, 0)
    }
}

fn passed_pawns<const TRACE: bool>(board: &Board, color: Color, t: &mut Trace) -> ScoreTuple {
    let mut score = ScoreTuple::new(0, 0);
    let mut passers = board.passed_pawns(color);

    let mut blockers = match color {
        Color::White => passers
            .north_one()
            .intersection(board.all[Color::Black.as_index()]),
        Color::Black => passers
            .south_one()
            .intersection(board.all[Color::White.as_index()]),
    };

    bitloop!(|sq| passers, {
        score += PASSER_PST.access(color, sq);

        if TRACE {
            let sq = color_adjust(sq, color);
            trace_update!(t, Passer, (sq), color, 1);
        }
    });

    bitloop!(|sq| blockers, {
        score += PASSER_BLOCKERS_PRT.access(color, sq);

        if TRACE {
            let rank = color_adjust(sq, color).rank();
            trace_update!(t, PasserBlocker, (rank), color, 1);
        }
    });

    score
}

fn isolated_pawns<const TRACE: bool>(board: &Board, color: Color, t: &mut Trace) -> ScoreTuple {
    let mut score = ScoreTuple::new(0, 0);

    let mut isolated = board.isolated_pawns(color);
    bitloop!(|sq| isolated, {
        score += ISOLATED_PAWNS_PRT.access(color, sq);

        if TRACE {
            let rank = color_adjust(sq, color).rank();
            trace_update!(t, IsolatedPawns, (rank), color, 1);
        }
    });

    score
}

fn phalanx_pawns<const TRACE: bool>(board: &Board, color: Color, t: &mut Trace) -> ScoreTuple {
    let mut score = ScoreTuple::new(0, 0);

    let mut phalanx = board.phalanx_pawns(color);
    bitloop!(|sq| phalanx, {
        score += PHALANX_PAWNS_PRT.access(color, sq);

        if TRACE {
            let rank = color_adjust(sq, color).rank();
            trace_update!(t, PhalanxPawns, (rank), color, 1);
        }
    });

    score
}

fn eval_or_trace<const TRACE: bool>(board: &Board, t: &mut Trace) -> EvalScore {
    let us = board.color_to_move;
    let them = board.color_to_move.flip();

    if TRACE {
        let color = board.color_to_move;
        trace_update!(t, TempoBonus, (), color, 1);
    }

    let mut score_tuple = TEMPO_BONUS;
    score_tuple += pst_eval::<TRACE>(board, us, t) - pst_eval::<TRACE>(board, them, t);
    score_tuple += bishop_pair::<TRACE>(board, us, t) - bishop_pair::<TRACE>(board, them, t);
    score_tuple += passed_pawns::<TRACE>(board, us, t) - passed_pawns::<TRACE>(board, them, t);
    score_tuple += isolated_pawns::<TRACE>(board, us, t) - isolated_pawns::<TRACE>(board, them, t);
    score_tuple += phalanx_pawns::<TRACE>(board, us, t) - phalanx_pawns::<TRACE>(board, them, t);
    score_tuple += mobility_threats_safety::<TRACE>(board, us, t)
        - mobility_threats_safety::<TRACE>(board, them, t);

    let mg_phase = i32::from(phase(board));
    let eg_phase = i32::from(PHASE_MAX) - mg_phase;

    (score_tuple.mg() * mg_phase + score_tuple.eg() * eg_phase) / i32::from(PHASE_MAX)
}

pub fn evaluate(board: &Board) -> EvalScore {
    eval_or_trace::<false>(board, &mut empty_trace())
}

pub fn trace_of_position(board: &Board) -> Trace {
    let mut trace = empty_trace();
    eval_or_trace::<true>(board, &mut trace);
    trace
}
