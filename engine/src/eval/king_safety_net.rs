use crate::eval::trace::{AttackingPawnLocations, DefendingPawnLocations};
use crate::{bitloop, trace_safety_update};
use crate::board::board_representation::{Bitboard, Color, Piece, Square, NUM_FILES, NUM_SQUARES};
use crate::eval::evaluation::ScoreTuple;

use super::eval_constants::{
    ATTACKING_PAWN_LOCATIONS, ATTACKS, DEFENDING_PAWN_LOCATIONS, DEFENSES, ENEMY_KING_RANK, TROPISM, HIDDEN_BIASES, OUTPUT_BIAS, HIDDEN_WEIGHTS, SAFETY_WEIGHT,
};
use super::trace::Trace;

pub const HIDDEN_LAYER_SIZE: usize = 8;

pub const SCALE: i32 = 128;

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

const PAWN_LOCATIONS: [[usize; NUM_SQUARES as usize]; NUM_FILES as usize] = {
    let mut result = [[0; NUM_SQUARES as usize]; NUM_FILES as usize];

    let mut i = 0;
    while i < (NUM_FILES as usize) {
        let mut bb = PAWN_MASKS[i];
        let mut location = 0;
        while bb.is_not_empty() {
            let sq = bb.lsb();
            result[i][sq.as_index()] = location;
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

    pub fn update_attacking_pawns<const TRACE: bool>(&mut self, pawns: Bitboard, enemy_king_sq: Square, color: Color, t: &mut Trace) {
        let king_file = enemy_king_sq.file() as usize;
        let mut attacking_pawns = pawns & PAWN_MASKS[king_file];

        bitloop!(|sq| attacking_pawns, {
            let location = PAWN_LOCATIONS[king_file][sq.as_index()];
            for (i, &weight) in ATTACKING_PAWN_LOCATIONS[location].iter().enumerate() {
                self.hidden_sums[i] += weight;
            }
            
            if TRACE {
                trace_safety_update!(t, AttackingPawnLocations, (location), color, 1);
            }
        });
    }

    pub fn update_defending_pawns<const TRACE: bool>(&mut self, pawns: Bitboard, friendly_king_sq: Square, color: Color, t: &mut Trace) {
        let king_file = friendly_king_sq.file() as usize;
        let mut defending_pawns = pawns & PAWN_MASKS[king_file];

        bitloop!(|sq| defending_pawns, {
            let location = PAWN_LOCATIONS[king_file][sq.as_index()];
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

    pub fn update_tropism(&mut self, trop: i32) {
        for (i, &weight) in TROPISM.iter().enumerate() {
            self.hidden_sums[i] += weight.mult(trop);
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

    pub fn calculate(&self) -> ScoreTuple {
        let mut output = OUTPUT_BIAS;

        for (i, &sum) in self.hidden_sums.iter().enumerate() {
            let weight = HIDDEN_WEIGHTS[i];
            let activation = sum.activation();
            output += activation * weight;
        }

        output.activation() * SAFETY_WEIGHT / SCALE.pow(2)
    }
}
