use crate::{
    board_representation::{Piece, Square, NUM_PIECES, NUM_RANKS, NUM_SQUARES},
    evaluation::NUM_PHASES,
    piece_loop_eval::MoveCounts,
};

const TRACE_LEN: usize = MaterialPst::LEN
    + Passer::LEN
    + PasserBlocker::LEN
    + BishopPair::LEN
    + Mobility::LEN
    + Safety::LEN
    + IsolatedPawns::LEN
    + PhalanxPawns::LEN
    + Threats::LEN
    + TempoBonus::LEN
    + ForwardMobility::LEN;
pub type Trace = [[f64; TRACE_LEN]; NUM_PHASES];

pub struct MaterialPst;
impl MaterialPst {
    pub const START: usize = 0;
    pub const LEN: usize = (NUM_PIECES as usize) * (NUM_SQUARES as usize);

    pub fn index(piece: Piece, sq: Square) -> usize {
        Self::START + usize::from(NUM_SQUARES) * piece.as_index() + sq.as_index()
    }
}

pub struct Passer;
impl Passer {
    pub const START: usize = MaterialPst::START + MaterialPst::LEN;
    pub const LEN: usize = (NUM_SQUARES as usize);

    pub fn index(sq: Square) -> usize {
        Self::START + sq.as_index()
    }
}

pub struct PasserBlocker;
impl PasserBlocker {
    pub const START: usize = Passer::START + Passer::LEN;
    pub const LEN: usize = (NUM_RANKS as usize);

    pub fn index(rank: u8) -> usize {
        Self::START + rank as usize
    }
}

pub struct BishopPair;
impl BishopPair {
    pub const START: usize = PasserBlocker::START + PasserBlocker::LEN;
    pub const LEN: usize = 1;

    pub fn index() -> usize {
        Self::START
    }
}

struct Mobility;
impl Mobility {
    pub const START: usize = BishopPair::START + BishopPair::LEN;
    pub const PIECE_MOVECOUNTS: [usize; 4] = [
        MoveCounts::KNIGHT,
        MoveCounts::BISHOP,
        MoveCounts::ROOK,
        MoveCounts::QUEEN,
    ];
    pub const PIECE_OFFSETS: [usize; 4] = [
        0,
        MoveCounts::KNIGHT,
        MoveCounts::KNIGHT + MoveCounts::BISHOP,
        MoveCounts::KNIGHT + MoveCounts::BISHOP + MoveCounts::ROOK,
    ];
    pub const LEN: usize =
        MoveCounts::KNIGHT + MoveCounts::BISHOP + MoveCounts::ROOK + MoveCounts::QUEEN;

    pub fn index(piece: Piece, attack_count: u32) -> usize {
        Self::START + (attack_count as usize) + Self::PIECE_OFFSETS[piece.as_index()]
    }
}

pub struct Safety;
impl Safety {
    const START: usize = Mobility::START + Mobility::LEN;
    const LEN: usize = (MoveCounts::QUEEN * (NUM_PIECES - 1) as usize);

    fn index(piece: Piece, enemy_virt_mobility: usize) -> usize {
        Self::START + MoveCounts::QUEEN * piece.as_index() + enemy_virt_mobility
    }
}

pub struct IsolatedPawns;
impl IsolatedPawns {
    pub const START: usize = Safety::START + Safety::LEN;
    pub const LEN: usize = (NUM_RANKS as usize);

    pub fn index(rank: u8) -> usize {
        Self::START + rank as usize
    }
}

pub struct PhalanxPawns;
impl PhalanxPawns {
    pub const START: usize = IsolatedPawns::START + IsolatedPawns::LEN;
    pub const LEN: usize = (NUM_RANKS as usize);

    pub fn index(rank: u8) -> usize {
        Self::START + rank as usize
    }
}

pub struct Threats;
impl Threats {
    pub const START: usize = PhalanxPawns::START + PhalanxPawns::LEN;
    pub const LEN: usize = 11;

    pub const PAWN_THREAT_ON_KNIGHT: usize = Self::START;
    pub const PAWN_THREAT_ON_BISHOP: usize = Self::START + 1;
    pub const PAWN_THREAT_ON_ROOK: usize = Self::START + 2;
    pub const PAWN_THREAT_ON_QUEEN: usize = Self::START + 3;

    pub const KNIGHT_THREAT_ON_BISHOP: usize = Self::START + 4;
    pub const KNIGHT_THREAT_ON_ROOK: usize = Self::START + 5;
    pub const KNIGHT_THREAT_ON_QUEEN: usize = Self::START + 6;

    pub const BISHOP_THREAT_ON_KNIGHT: usize = Self::START + 7;
    pub const BISHOP_THREAT_ON_ROOK: usize = Self::START + 8;
    pub const BISHOP_THREAT_ON_QUEEN: usize = Self::START + 9;

    pub const ROOK_THREAT_ON_QUEEN: usize = Self::START + 10;
}

pub struct TempoBonus;
impl TempoBonus {
    pub const START: usize = Threats::START + Threats::LEN;
    pub const LEN: usize = (NUM_RANKS as usize);

    pub fn index() -> usize {
        Self::START
    }
}

pub struct ForwardMobility;
impl ForwardMobility {
    pub const START: usize = TempoBonus::START + TempoBonus::LEN;
    pub const PIECE_MOVECOUNTS: [usize; 4] = [
        MoveCounts::FORWARD_KNIGHT,
        MoveCounts::FORWARD_BISHOP,
        MoveCounts::FORWARD_ROOK,
        MoveCounts::FORWARD_QUEEN,
    ];
    pub const PIECE_OFFSETS: [usize; 4] = [
        0,
        MoveCounts::FORWARD_KNIGHT,
        MoveCounts::FORWARD_KNIGHT + MoveCounts::FORWARD_BISHOP,
        MoveCounts::FORWARD_KNIGHT + MoveCounts::FORWARD_BISHOP + MoveCounts::FORWARD_ROOK,
    ];
    pub const LEN: usize = MoveCounts::FORWARD_KNIGHT
        + MoveCounts::FORWARD_BISHOP
        + MoveCounts::FORWARD_ROOK
        + MoveCounts::FORWARD_QUEEN;

    pub fn index(piece: Piece, f_mobility: usize) -> usize {
        Self::START + f_mobility + Self::PIECE_OFFSETS[piece.as_index()]
    }
}