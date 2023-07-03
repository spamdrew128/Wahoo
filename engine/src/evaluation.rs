use std::ops::{Add, AddAssign, Sub};

use crate::{
    bitloop,
    board_representation::{Board, Color, Piece, Square},
    eval_constants::{BISHOP_PAIR_BONUS, MATERIAL_PSTS, PASSER_BLOCKERS_RST, PASSER_PST},
    piece_loop_eval::mobility,
    search::MAX_PLY,
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

#[derive(Debug, Copy, Clone)]
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

fn pst_eval(board: &Board, color: Color) -> ScoreTuple {
    let mut score = ScoreTuple::new(0, 0);
    for piece in Piece::LIST {
        let mut pieces = board.piece_bb(piece, color);
        let pst = &MATERIAL_PSTS[piece.as_index()];

        bitloop!(|sq|, pieces, {
            score += pst.access(color, sq);
        });
    }
    score
}

const fn bishop_pair(board: &Board, color: Color) -> ScoreTuple {
    let bishops = board.piece_bb(Piece::BISHOP, color);
    if bishops.popcount() >= 2 {
        BISHOP_PAIR_BONUS
    } else {
        ScoreTuple::new(0, 0)
    }
}

fn passed_pawns(board: &Board, color: Color) -> ScoreTuple {
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

    bitloop!(|sq|, passers, {
        score += PASSER_PST.access(color, sq);
    });

    bitloop!(|sq|, blockers, {
        score += PASSER_BLOCKERS_RST.access(color, sq);
    });

    score
}

pub fn evaluate(board: &Board) -> EvalScore {
    let us = board.color_to_move;
    let them = board.color_to_move.flip();

    let mut score_tuple = ScoreTuple::new(0, 0);
    score_tuple += pst_eval(board, us) - pst_eval(board, them);
    score_tuple += bishop_pair(board, us) - bishop_pair(board, them);
    score_tuple += passed_pawns(board, us) - passed_pawns(board, them);
    score_tuple += mobility(board, us) - mobility(board, them);

    let mg_phase = i32::from(phase(board));
    let eg_phase = i32::from(PHASE_MAX) - mg_phase;

    (score_tuple.mg() * mg_phase + score_tuple.eg() * eg_phase) / i32::from(PHASE_MAX)
}
