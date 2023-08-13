use std::ops::{Add, AddAssign, Div, Mul, Sub};

use crate::{
    bitloop,
    board::board_representation::{Bitboard, Board, Color, Piece, Square, NUM_COLORS, NUM_SQUARES},
    eval::eval_constants::{
        BISHOP_PAIR_BONUS, ISOLATED_PAWNS_PRT, MATERIAL_PSTS, PASSER_BLOCKERS_PRT, PASSER_PST,
        PHALANX_PAWNS_PRT, TEMPO_BONUS,
    },
    eval::trace::{
        color_adjust, BishopPair, IsolatedPawns, MaterialPst, Passer, PasserBlocker, PhalanxPawns,
        TempoBonus, Trace,
    },
    eval::{
        eval_constants::PASSER_SQ_RULE_BONUS, piece_loop_eval::mobility_threats_safety,
        trace::PasserSqRule,
    },
    search::search::MAX_PLY,
    trace_update,
};

use super::king_safety_net::SCALE;

const fn passer_squares_init(
    is_stm: bool,
) -> [[Bitboard; NUM_SQUARES as usize]; NUM_COLORS as usize] {
    let mut result = [[Bitboard::EMPTY; NUM_SQUARES as usize]; NUM_COLORS as usize];

    let mut i = 8;
    while i < (NUM_SQUARES - 8) {
        let (w_sq, b_sq) = if is_stm {
            (Square::new(i), Square::new(i))
        } else {
            (
                Square::new(i).retreat(1, Color::White),
                Square::new(i).retreat(1, Color::Black),
            )
        };
        let (w_ranks_away, b_ranks_away) = (7 - w_sq.rank(), b_sq.rank());
        let (w, b) = (
            w_sq.as_bitboard().row_set(w_ranks_away).fill(Color::White),
            b_sq.as_bitboard().row_set(b_ranks_away).fill(Color::Black),
        );

        result[Color::White.as_index()][i as usize] = w;
        result[Color::Black.as_index()][i as usize] = b;

        i += 1;
    }

    result
}

const STM_PASSER_SQ: [[Bitboard; NUM_SQUARES as usize]; NUM_COLORS as usize] =
    passer_squares_init(true);
const NON_STM_PASSER_SQ: [[Bitboard; NUM_SQUARES as usize]; NUM_COLORS as usize] =
    passer_squares_init(false);

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
pub const TB_WIN_SCORE: EvalScore = MATE_THRESHOLD - 1000;
pub const TB_LOSS_SCORE: EvalScore = -TB_WIN_SCORE;

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

    pub fn activation(self) -> Self {
        Self(self.mg().clamp(0, SCALE), self.eg().clamp(0, SCALE))
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

impl Mul for ScoreTuple {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Div<i32> for ScoreTuple {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        Self(self.0 / rhs, self.1 / rhs)
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

fn passed_pawns<const IS_STM: bool, const TRACE: bool>(
    board: &Board,
    color: Color,
    t: &mut Trace,
) -> ScoreTuple {
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

    let enemy_king = board.piece_bb(Piece::KING, color.flip());
    bitloop!(|sq| passers, {
        score += PASSER_PST.access(color, sq);

        let passer_sq = if IS_STM {
            STM_PASSER_SQ[color.as_index()][sq.as_index()]
        } else {
            NON_STM_PASSER_SQ[color.as_index()][sq.as_index()]
        };

        if passer_sq.intersection(enemy_king).is_empty() {
            score += PASSER_SQ_RULE_BONUS;

            if TRACE {
                trace_update!(t, PasserSqRule, (), color, 1);
            }
        }

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
    score_tuple +=
        passed_pawns::<true, TRACE>(board, us, t) - passed_pawns::<false, TRACE>(board, them, t);
    score_tuple += isolated_pawns::<TRACE>(board, us, t) - isolated_pawns::<TRACE>(board, them, t);
    score_tuple += phalanx_pawns::<TRACE>(board, us, t) - phalanx_pawns::<TRACE>(board, them, t);
    score_tuple += mobility_threats_safety::<TRACE>(board, us, them, t);

    let mg_phase = i32::from(phase(board));
    let eg_phase = i32::from(PHASE_MAX) - mg_phase;

    (score_tuple.mg() * mg_phase + score_tuple.eg() * eg_phase) / i32::from(PHASE_MAX)
}

pub fn evaluate(board: &Board) -> EvalScore {
    eval_or_trace::<false>(board, &mut Trace::empty())
}

pub fn trace_of_position(board: &Board) -> Trace {
    let mut trace = Trace::empty();
    eval_or_trace::<true>(board, &mut trace);
    trace
}

#[cfg(test)]
mod tests {
    use crate::{
        board::board_representation::{Board, Color, Square},
        eval::evaluation::STM_PASSER_SQ,
    };

    #[test]
    fn passer_sq_test() {
        let board = Board::from_fen("8/8/8/2k2PP1/8/8/8/K7 w - - 0 1");
        let k_sq = board.color_king_sq(Color::Black).as_bitboard();

        let p1 = Square::F5.as_index();
        let p2 = Square::G5.as_index();
        assert!(STM_PASSER_SQ[Color::White.as_index()][p1]
            .intersection(k_sq)
            .is_not_empty());
        assert!(STM_PASSER_SQ[Color::White.as_index()][p2]
            .intersection(k_sq)
            .is_empty());
    }
}
