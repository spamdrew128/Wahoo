use crate::board::board_representation::{
    Bitboard, Color, Piece, Square, NUM_COLORS, NUM_FILES, NUM_SQUARES,
};
use crate::eval::evaluation::ScoreTuple;
use crate::eval::trace::{
    AttackingPawnLocations, AttackingPieceLocations, DefendingPawnLocations,
    DefendingPieceLocations,
};
use crate::{bitloop, trace_safety_update};

use super::eval_constants::{
    ATTACKING_PAWN_LOCATIONS, ATTACKING_PIECE_LOCATIONS, ATTACKS, DEFENDING_PAWN_LOCATIONS,
    DEFENDING_PIECE_LOCATIONS, DEFENSES, ENEMY_KING_RANK, HIDDEN_BIASES, OUTPUT_BIAS,
    OUTPUT_WEIGHTS,
};
use super::trace::Trace;

pub const HIDDEN_LAYER_SIZE: usize = 8;

pub const SCALE: i32 = 128;

const INVALID_LOCATION: usize = 64;

const PAWN_MASKS: [Bitboard; NUM_FILES as usize] = {
    let mut masks = [Bitboard::EMPTY; NUM_FILES as usize];

    let mut i = 1;
    while i < 7 {
        let sq_bb = Square::new(i as u8).as_bitboard();
        masks[i] = sq_bb
            .union(sq_bb.east_one())
            .union(sq_bb.west_one())
            .forward_fill(Color::White)
            .without(Bitboard::RANK_8);
        i += 1;
    }
    masks[0] = masks[1];
    masks[7] = masks[6];

    masks
};

const PIECE_MASKS: [Bitboard; NUM_FILES as usize] = {
    let mut masks = [Bitboard::EMPTY; NUM_FILES as usize];

    let mut i = 0;
    while i < (NUM_FILES as usize) {
        masks[i] = PAWN_MASKS[i].file_fill();
        i += 1;
    }

    masks
};

const PAWN_LOCATIONS: [[[usize; NUM_SQUARES as usize]; NUM_FILES as usize]; NUM_COLORS as usize] = {
    let mut result =
        [[[INVALID_LOCATION; NUM_SQUARES as usize]; NUM_FILES as usize]; NUM_COLORS as usize];

    let mut i = 0;
    while i < (NUM_FILES as usize) {
        let mut bb = PAWN_MASKS[i];
        let mut location = 0;
        while bb.is_not_empty() {
            let sq = bb.lsb();
            result[Color::White.as_index()][i][sq.mirror().as_index()] = location;
            result[Color::Black.as_index()][i][sq.as_index()] = location;
            location += 1;
            bb = bb.xor(sq.as_bitboard());
        }
        i += 1;
    }

    result
};

const PIECE_LOCATIONS: [[[usize; NUM_SQUARES as usize]; NUM_FILES as usize]; NUM_COLORS as usize] = {
    let mut result =
        [[[INVALID_LOCATION; NUM_SQUARES as usize]; NUM_FILES as usize]; NUM_COLORS as usize];

    let mut i = 0;
    while i < (NUM_FILES as usize) {
        let mut bb = PIECE_MASKS[i];
        let mut location = 0;
        while bb.is_not_empty() {
            let sq = bb.lsb();
            result[Color::White.as_index()][i][sq.mirror().as_index()] = location;
            result[Color::Black.as_index()][i][sq.as_index()] = location;
            location += 1;
            bb = bb.xor(sq.as_bitboard());
        }
        i += 1;
    }

    result
};

pub struct SafetyNet {
    hidden_sums: [ScoreTuple; HIDDEN_LAYER_SIZE],
}

impl SafetyNet {
    pub const fn new() -> Self {
        Self {
            hidden_sums: HIDDEN_BIASES,
        }
    }

    pub fn update_attacking_pawns<const TRACE: bool>(
        &mut self,
        pawns: Bitboard,
        enemy_king_sq: Square,
        color: Color,
        t: &mut Trace,
    ) {
        let king_file = enemy_king_sq.file() as usize;
        let mut attacking_pawns = pawns & PAWN_MASKS[king_file];

        bitloop!(|sq| attacking_pawns, {
            let location = PAWN_LOCATIONS[color.as_index()][king_file][sq.as_index()];
            for (i, &weight) in ATTACKING_PAWN_LOCATIONS[location].iter().enumerate() {
                self.hidden_sums[i] += weight;
            }

            if TRACE {
                trace_safety_update!(t, AttackingPawnLocations, (location), color, 1);
            }
        });
    }

    pub fn update_defending_pawns<const TRACE: bool>(
        &mut self,
        pawns: Bitboard,
        friendly_king_sq: Square,
        color: Color,
        t: &mut Trace,
    ) {
        let king_file = friendly_king_sq.file() as usize;
        let mut defending_pawns = pawns & PAWN_MASKS[king_file];

        bitloop!(|sq| defending_pawns, {
            let location = PAWN_LOCATIONS[color.as_index()][king_file][sq.as_index()];
            for (i, &weight) in DEFENDING_PAWN_LOCATIONS[location].iter().enumerate() {
                self.hidden_sums[i] += weight;
            }

            if TRACE {
                trace_safety_update!(t, DefendingPawnLocations, (location), color, 1);
            }
        });
    }

    pub fn update_enemy_king_rank(&mut self, sq: Square, color: Color) {
        for (i, &weight) in ENEMY_KING_RANK.access(color, sq).iter().enumerate() {
            self.hidden_sums[i] += weight;
        }
    }

    pub fn update_attacks(&mut self, piece: Piece, count: i32) {
        for (i, &weight) in ATTACKS[piece.as_index()].iter().enumerate() {
            self.hidden_sums[i] += weight.mult(count);
        }
    }

    pub fn update_defenses(&mut self, piece: Piece, count: i32) {
        for (i, &weight) in DEFENSES[piece.as_index()].iter().enumerate() {
            self.hidden_sums[i] += weight.mult(count);
        }
    }

    pub fn update_piece_attacker<const TRACE: bool>(
        &mut self,
        piece: Piece,
        sq: Square,
        color: Color,
        enemy_king_sq: Square,
        t: &mut Trace,
    ) {
        let location =
            PIECE_LOCATIONS[color.as_index()][enemy_king_sq.file() as usize][sq.as_index()];

        if location == INVALID_LOCATION {
            return;
        }

        for (i, &weight) in ATTACKING_PIECE_LOCATIONS[piece.as_index()][location]
            .iter()
            .enumerate()
        {
            self.hidden_sums[i] += weight;
        }

        if TRACE {
            trace_safety_update!(t, AttackingPieceLocations, (piece, location), color, 1);
        }
    }

    pub fn update_piece_defender<const TRACE: bool>(
        &mut self,
        piece: Piece,
        sq: Square,
        color: Color,
        friendly_king_sq: Square,
        t: &mut Trace,
    ) {
        let location =
            PIECE_LOCATIONS[color.as_index()][friendly_king_sq.file() as usize][sq.as_index()];

        if location == INVALID_LOCATION {
            return;
        }

        for (i, &weight) in DEFENDING_PIECE_LOCATIONS[piece.as_index()][location]
            .iter()
            .enumerate()
        {
            self.hidden_sums[i] += weight;
        }

        if TRACE {
            trace_safety_update!(t, DefendingPieceLocations, (piece, location), color, 1);
        }
    }

    pub fn calculate(&self) -> ScoreTuple {
        let mut output = OUTPUT_BIAS.mult(SCALE);

        for (i, &sum) in self.hidden_sums.iter().enumerate() {
            let weight = OUTPUT_WEIGHTS[i];
            let activation = sum.activation();
            output += activation * weight;
        }

        output / SCALE.pow(2)
    }
}
