use crate::board::board_representation::{Bitboard, Color, Square, NUM_FILES, NUM_SQUARES};
use crate::eval::evaluation::ScoreTuple;

use super::eval_constants::{ATTACKING_PAWN_LOCATIONS, DEFENDING_PAWN_LOCATIONS};

pub const HIDDEN_LAYER_SIZE: usize = 8;

const PAWN_LOCATIONS: [[usize; NUM_SQUARES as usize]; NUM_FILES as usize] = {
    let mut result = [[0; NUM_SQUARES as usize]; NUM_FILES as usize];
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

    i = 0;
    while i < (NUM_FILES as usize) {
        let mut bb = masks[i];
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

struct SafetyNet {
    hidden_sums: [ScoreTuple; HIDDEN_LAYER_SIZE],
}

impl SafetyNet {
    pub const fn new() -> Self {
        Self {
            hidden_sums: [ScoreTuple::new(0, 0); HIDDEN_LAYER_SIZE],
        }
    }

    pub fn update_attacking_pawn(&mut self, location: usize) {
        for (i, &weight) in ATTACKING_PAWN_LOCATIONS[location].iter().enumerate() {
            self.hidden_sums[i] += weight;
        }
    }

    pub fn update_defending_pawn(&mut self, location: usize) {
        for (i, &weight) in DEFENDING_PAWN_LOCATIONS[location].iter().enumerate() {
            self.hidden_sums[i] += weight;
        }
    }
}
