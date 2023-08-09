use crate::{
    board::board_representation::{
        Color, Piece, Square, NUM_COLORS, NUM_PIECES, NUM_RANKS, NUM_SQUARES,
    },
    eval::piece_loop_eval::MoveCounts,
};

pub const LINEAR_TRACE_LEN: usize = MaterialPst::LEN
    + Passer::LEN
    + PasserBlocker::LEN
    + BishopPair::LEN
    + Mobility::LEN
    + IsolatedPawns::LEN
    + PhalanxPawns::LEN
    + Threats::LEN
    + TempoBonus::LEN
    + ForwardMobility::LEN
    + PasserSqRule::LEN;

pub const SAFETY_TRACE_LEN: usize = Attacks::LEN
    + Defenses::LEN
    + EnemyKingRank::LEN
    + Tropism::LEN
    + AttackingPawnLocations::LEN
    + DefendingPawnLocations::LEN;

pub struct Trace {
    pub linear: [i8; LINEAR_TRACE_LEN],
    pub safety: [[i8; SAFETY_TRACE_LEN]; NUM_COLORS as usize],
}

impl Trace {
    pub const fn empty() -> Self {
        Self {
            linear: [0; LINEAR_TRACE_LEN],
            safety: [[0; SAFETY_TRACE_LEN]; NUM_COLORS as usize],
        }
    }
}

pub const fn color_adjust(sq: Square, color: Color) -> Square {
    match color {
        Color::White => sq.mirror(),
        Color::Black => sq,
    }
}

#[macro_export]
macro_rules! trace_update {
    ($trace:ident, $name:ident, ($($arg:ident),*), $color:expr, $val:expr) => {
        let mult = match $color {
            Color::White => 1,
            Color::Black => -1,
        };
        let index = $name::index($($arg,)*);
        $trace.linear[index] += mult * ($val as i8);
    };
}

#[macro_export]
macro_rules! trace_threat_update {
    ($trace:ident, $index_name:ident, $color:expr, $attacks:expr, $enemy:expr) => {
        let mult = match $color {
            Color::White => 1,
            Color::Black => -1,
        };
        let val = ($attacks & $enemy).popcount();
        $trace.linear[Threats::$index_name] += mult * (val as i8);
    };
}

#[macro_export]
macro_rules! trace_safety_update {
    ($trace:ident, $name:ident, ($($arg:ident),*), $color:expr, $val:expr) => {
        let index = $name::index($($arg,)*);
        $trace.safety[$color.as_index()][index] += $val as i8;
    };
}

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

    pub const fn index(sq: Square) -> usize {
        Self::START + sq.as_index()
    }
}

pub struct PasserBlocker;
impl PasserBlocker {
    pub const START: usize = Passer::START + Passer::LEN;
    pub const LEN: usize = (NUM_RANKS as usize);

    pub const fn index(rank: u8) -> usize {
        Self::START + rank as usize
    }
}

pub struct BishopPair;
impl BishopPair {
    pub const START: usize = PasserBlocker::START + PasserBlocker::LEN;
    pub const LEN: usize = 1;

    pub const fn index() -> usize {
        Self::START
    }
}

pub struct Mobility;
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

    pub const fn index(piece: Piece, attack_count: usize) -> usize {
        Self::START + attack_count + Self::PIECE_OFFSETS[piece.as_index()]
    }
}

pub struct IsolatedPawns;
impl IsolatedPawns {
    pub const START: usize = Mobility::START + Mobility::LEN;
    pub const LEN: usize = (NUM_RANKS as usize);

    pub const fn index(rank: u8) -> usize {
        Self::START + rank as usize
    }
}

pub struct PhalanxPawns;
impl PhalanxPawns {
    pub const START: usize = IsolatedPawns::START + IsolatedPawns::LEN;
    pub const LEN: usize = (NUM_RANKS as usize);

    pub const fn index(rank: u8) -> usize {
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

    pub const fn index() -> usize {
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

    pub const fn index(piece: Piece, f_mobility: usize) -> usize {
        Self::START + f_mobility + Self::PIECE_OFFSETS[piece.as_index()]
    }
}

pub struct PasserSqRule;
impl PasserSqRule {
    pub const START: usize = ForwardMobility::START + ForwardMobility::LEN;
    pub const LEN: usize = 1;

    pub const fn index() -> usize {
        Self::START
    }
}

// SAFETY STUFF
pub struct Attacks;
impl Attacks {
    pub const START: usize = 0;
    const LEN: usize = (MoveCounts::QUEEN * (NUM_PIECES - 2) as usize);

    pub const fn index(piece: Piece) -> usize {
        Self::START + piece.as_index()
    }
}

pub struct Defenses;
impl Defenses {
    pub const START: usize = Attacks::START + Attacks::LEN;
    const LEN: usize = (MoveCounts::QUEEN * (NUM_PIECES - 2) as usize);

    pub const fn index(piece: Piece) -> usize {
        Self::START + piece.as_index()
    }
}

pub struct EnemyKingRank;
impl EnemyKingRank {
    pub const START: usize = Defenses::START + Defenses::LEN;
    pub const LEN: usize = NUM_RANKS as usize;

    pub const fn index(rank: u8) -> usize {
        Self::START + rank as usize
    }
}

pub struct Tropism;
impl Tropism {
    pub const START: usize = EnemyKingRank::START + EnemyKingRank::LEN;
    pub const LEN: usize = 1;

    pub const fn index() -> usize {
        Self::START
    }
}

pub struct AttackingPawnLocations;
impl AttackingPawnLocations {
    pub const START: usize = Tropism::START + Tropism::LEN;
    pub const LEN: usize = 18;

    pub const fn index(location: usize) -> usize {
        Self::START + location
    }
}

pub struct DefendingPawnLocations;
impl DefendingPawnLocations {
    pub const START: usize = AttackingPawnLocations::START + AttackingPawnLocations::LEN;
    pub const LEN: usize = 18;

    pub const fn index(location: usize) -> usize {
        Self::START + location
    }
}
