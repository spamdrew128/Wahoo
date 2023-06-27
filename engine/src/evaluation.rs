use std::ops::{Add, AddAssign, Sub};

use crate::{
    board_representation::{Board, Color, Piece, Square},
    eval_bitloop,
    eval_constants::{PASSER_PST, PST},
    search::MAX_PLY,
};

pub type Phase = u8;
pub const PHASE_MAX: Phase = 24;
pub const MG: usize = 0;
pub const EG: usize = 1;
pub const PHASES: [usize; NUM_PHASES] = [MG, EG];
pub const NUM_PHASES: usize = 2;

pub type EvalScore = i16;
pub const INF: EvalScore = i16::MAX - 10;
pub const EVAL_MAX: EvalScore = INF - 1;
pub const MATE_THRESHOLD: EvalScore = EVAL_MAX - (MAX_PLY as i16);

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

        eval_bitloop!(|sq|, pieces, color, {
            score += PST[piece.as_index()][sq.as_index()];
        });
    }
    score
}

fn passed_pawns(board: &Board, color: Color) -> ScoreTuple {
    let mut score = ScoreTuple::new(0, 0);

    let pawns = board.piece_bb(Piece::PAWN, color);
    let opp_front_span = pawns.forward_fill(color.flip());
    let opp_blocks = opp_front_span | opp_front_span.east_one() | opp_front_span.west_one();

    let mut passers = pawns.without(opp_blocks);
    eval_bitloop!(|sq|, passers, color, {
        score += PASSER_PST[sq.as_index()];
    });
    score
}

pub fn evaluate(board: &Board) -> EvalScore {
    let mut score_tuple = ScoreTuple::new(0, 0);
    score_tuple += pst_eval(board, Color::White) - pst_eval(board, Color::Black);
    score_tuple += passed_pawns(board, Color::White) - passed_pawns(board, Color::Black);

    let mg_phase = i32::from(phase(board));
    let eg_phase = i32::from(PHASE_MAX) - mg_phase;
    let score = (i32::from(score_tuple.mg()) * mg_phase + i32::from(score_tuple.eg()) * eg_phase)
        / i32::from(PHASE_MAX);

    match board.color_to_move {
        Color::White => score as EvalScore,
        Color::Black => -score as EvalScore,
    }
}
