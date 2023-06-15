use crate::{
    bitloop,
    board_representation::{Board, Color, Piece, Square},
    eval_constants::PST,
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

    const fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }

    const fn subtract(self, rhs: Self) -> Self {
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

fn pst_eval(board: &Board) -> ScoreTuple {
    let mut score = ScoreTuple::new(0, 0);
    for piece in Piece::LIST {
        let mut w_pieces = board.piece_bb(piece, Color::White);
        let mut b_pieces = board.piece_bb(piece, Color::Black);

        bitloop!(|sq|, w_pieces, {
            score = score.add(PST[piece.as_index()][sq.flip().as_index()]);
        });

        bitloop!(|sq|, b_pieces, {
            score = score.subtract(PST[piece.as_index()][sq.as_index()]);
        });
    }
    score
}

pub fn evaluate(board: &Board) -> EvalScore {
    let mut score = ScoreTuple::new(0, 0);
    score = score.add(pst_eval(board));

    let mg_phase = i16::from(phase(board));
    let eg_phase = i16::from(PHASE_MAX) - mg_phase;
    (score.mg() * mg_phase + score.eg() * eg_phase) / i16::from(PHASE_MAX)
}
