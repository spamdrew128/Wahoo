use crate::bitloop;

use super::dummy_types::{Bitboard, Square, NUM_FILES, NUM_SQUARES};

fn pawn_locations() -> [[usize; NUM_SQUARES as usize]; NUM_FILES as usize] {
    let mut result = [[0; NUM_SQUARES as usize]; NUM_FILES as usize];
    let mut masks = [Bitboard::EMPTY; NUM_FILES as usize];

    let mut i = 1;
    while i < 7 {
        let sq_bb = Square::new(i as u8).as_bitboard();
        let mut mask = sq_bb
            .union(sq_bb.east_one())
            .union(sq_bb.west_one())
            .file_fill();
        mask = mask.north_one() & mask.south_one();
        masks[i] = mask;
    }
    masks[0] = masks[1];
    masks[7] = masks[6];

    i = 0;
    while i < (NUM_FILES as usize) {
        let mut location = 0;
        let mut bb = masks[i];
        bitloop!(|sq| bb, {
            result[i][sq.as_index()] = location;
            location += 1;
        });
    }

    result
}

pub fn pawn_locations_table_init_string() -> String {
    let t = pawn_locations();

    let mut result = String::new();
    result.push_str("[\n");
    for i in 0..NUM_FILES {
        result.push('[');
        for j in 0..NUM_SQUARES {
            if j % 8 == 0 {
                result.push_str("\n  ");
            }
            result.push_str(format!("{}, ", t[i as usize][j as usize]).as_str());
        }
        result.push_str("\n],\n");
    }
    result.push_str("]\n");

    result
}