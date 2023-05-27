use crate::board_representation::{Bitboard, Square, NUM_SQUARES};
use crate::magic::{MagicEntry, MagicLookup};

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

const MAGIC_LOOKUP: MagicLookup = include!(concat!(env!("OUT_DIR"), "/magic_lookup_init.txt"));

pub const fn king(sq: Square) -> Bitboard {
    KING_ATTACKS[sq.as_index()]
}

pub const fn knight(sq: Square) -> Bitboard {
    KNIGHT_ATTACKS[sq.as_index()]
}

pub const fn bishop(sq: Square, occupied: Bitboard) -> Bitboard {
    MAGIC_LOOKUP.bishop_attack_set(sq, occupied)
}

pub const fn rook(sq: Square, occupied: Bitboard) -> Bitboard {
    MAGIC_LOOKUP.rook_attack_set(sq, occupied)
}

pub const fn queen(sq: Square, occupied: Bitboard) -> Bitboard {
    MAGIC_LOOKUP
        .rook_attack_set(sq, occupied)
        .union(MAGIC_LOOKUP.bishop_attack_set(sq, occupied))
}

#[cfg(test)]
mod tests {
    use super::{Bitboard, Square};
    use crate::attacks;
    use crate::bb_from_squares;
    use crate::board_representation::Board;

    #[test]
    fn king_lookup_test() {
        let pos_1 = Square::A1;
        let expected_1 = bb_from_squares!(A2, B2, B1);
        assert_eq!(attacks::king(pos_1), expected_1);

        let pos_2 = Square::E4;
        let expected_2 = bb_from_squares!(E3, E5, D3, D4, D5, F3, F4, F5);
        assert_eq!(attacks::king(pos_2), expected_2);
    }

    #[test]
    fn knight_lookup_test() {
        let pos_1 = Square::A1;
        let expected_1 = bb_from_squares!(C2, B3);
        assert_eq!(attacks::knight(pos_1), expected_1);

        let pos_2 = Square::E4;
        let expected_2 = bb_from_squares!(D2, F2, C3, G3, C5, G5, D6, F6);
        assert_eq!(attacks::knight(pos_2), expected_2);
    }

    #[test]
    fn bishop_lookup_test() {
        let board = Board::from_fen("1k6/ppp5/5n2/2b1pB1r/8/2P3BP/P1P2PP1/3R2K1 w - - 1 25");
        let attacks = attacks::bishop(Square::F5, board.occupied());

        let expected = bb_from_squares!(C2, D3, H3, E4, G4, E6, G6, D7, H7, C8);
        assert_eq!(attacks, expected);
    }

    #[test]
    fn rook_lookup_test() {
        let board = Board::from_fen("1k6/ppp5/5n2/2b1pB1r/8/2P3BP/P1P2PP1/3R2K1 w - - 1 25");
        let attacks = attacks::rook(Square::H5, board.occupied());

        let expected = bb_from_squares!(F5, G5, H3, H4, H6, H7, H8);
        assert_eq!(attacks, expected);
    }

    #[test]
    fn queen_lookup_test() {
        let board =
            Board::from_fen("2kr4/pp3pp1/2p3rp/2p1p3/1PB1P3/1R1P1q2/P1P2P1Q/5K2 b - - 6 26");
        let attacks = attacks::queen(Square::F3, board.occupied());

        let expected =
            bb_from_squares!(D1, H1, E2, F2, G2, D3, E3, G3, H3, E4, F4, G4, F5, H5, F6, F7);
        assert_eq!(attacks, expected);
    }
}
