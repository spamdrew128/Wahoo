use crate::board_representation::{Bitboard, Square, NUM_SQUARES};

macro_rules! init_lookup {
    (|$sq: ident|, $body:expr) => {{
        let mut $sq = 0;
        let mut table = [Bitboard::new(0); NUM_SQUARES as usize];
        while $sq < NUM_SQUARES {
            table[$sq as usize] = $body;
            $sq += 1;
        }
        table
    }};
}

pub const KING_ATTACKS: [Bitboard; NUM_SQUARES as usize] = init_lookup!(|sq|, {
    let bitset = Square::new(sq).as_bitboard();
    bitset.north_one()
        .union(bitset.northeast_one())
        .union(bitset.east_one())
        .union(bitset.southeast_one())
        .union(bitset.south_one())
        .union(bitset.southwest_one())
        .union(bitset.west_one())
        .union(bitset.northwest_one())
});

pub const KNIGHT_ATTACKS: [Bitboard; NUM_SQUARES as usize] = init_lookup!(|sq|, {
    let bitset = Square::new(sq).as_bitboard();
    bitset.north_one()
        .union(bitset.northeast_one())
        .union(bitset.east_one())
        .union(bitset.southeast_one())
        .union(bitset.south_one())
        .union(bitset.southwest_one())
        .union(bitset.west_one())
        .union(bitset.northwest_one())
});
