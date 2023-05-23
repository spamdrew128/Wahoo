use crate::board_representation::{Bitboard, Square, NUM_SQUARES};
use crate::magic;

macro_rules! init_lookup {
    (|$sq:ident|, $body:expr) => {{
        let mut $sq = 0;
        let mut table = [Bitboard::new(0); NUM_SQUARES as usize];
        while $sq < NUM_SQUARES {
            table[$sq as usize] = $body;
            $sq += 1;
        }
        table
    }};
}

const KING_ATTACKS: [Bitboard; NUM_SQUARES as usize] = init_lookup!(|sq|, {
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

const KNIGHT_ATTACKS: [Bitboard; NUM_SQUARES as usize] = init_lookup!(|sq|, {
    let bitset = Square::new(sq).as_bitboard();
    let vert = bitset.shift_north(2).union(bitset.shift_south(2));
    let horiz = bitset.no_wrap_shift_east(2).union(bitset.no_wrap_shift_west(2));

    vert.west_one().union(vert.east_one())
        .union(horiz.north_one().union(horiz.south_one()))
});

const MAGIC_LOOKUPS: magic::MagicLookup = magic::init_magic_lookup();

pub const fn king_attacks(sq: Square) -> Bitboard {
    KING_ATTACKS[sq.as_index()]
}

pub const fn knight_attacks(sq: Square) -> Bitboard {
    KNIGHT_ATTACKS[sq.as_index()]
}

#[cfg(test)]
mod tests {
    use super::{king_attacks, knight_attacks, Bitboard, Square};
    use crate::bb_from_squares;

    #[test]
    fn king_lookup_test() {
        let pos_1 = Square::A1;
        let expected_1 = bb_from_squares!(A2, B2, B1);
        assert_eq!(king_attacks(pos_1), expected_1);

        let pos_2 = Square::E4;
        let expected_2 = bb_from_squares!(E3, E5, D3, D4, D5, F3, F4, F5);
        assert_eq!(king_attacks(pos_2), expected_2);
    }

    #[test]
    fn knight_lookup_test() {
        let pos_1 = Square::A1;
        let expected_1 = bb_from_squares!(C2, B3);
        assert_eq!(knight_attacks(pos_1), expected_1);

        let pos_2 = Square::E4;
        let expected_2 = bb_from_squares!(D2, F2, C3, G3, C5, G5, D6, F6);
        assert_eq!(knight_attacks(pos_2), expected_2);
    }
}
