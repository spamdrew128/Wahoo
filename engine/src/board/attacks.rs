use super::{board_representation::{Bitboard, Color, NUM_SQUARES, Square, NUM_COLORS, Piece}, magic::MagicLookup};

macro_rules! init_lookup {
    (|$sq_bb:ident|, $body:expr) => {{
        let mut i = 0;
        let mut table = [Bitboard::EMPTY; NUM_SQUARES as usize];
        while i < NUM_SQUARES {
            let $sq_bb = Square::new(i).as_bitboard();
            table[i as usize] = $body;
            i += 1;
        }
        table
    }};
}

const KING_ATTACKS: [Bitboard; NUM_SQUARES as usize] = init_lookup!(|sq_bb|, {
    sq_bb.north_one()
        .union(sq_bb.northeast_one())
        .union(sq_bb.east_one())
        .union(sq_bb.southeast_one())
        .union(sq_bb.south_one())
        .union(sq_bb.southwest_one())
        .union(sq_bb.west_one())
        .union(sq_bb.northwest_one())
});

const KNIGHT_ATTACKS: [Bitboard; NUM_SQUARES as usize] = init_lookup!(|sq_bb|, {
    let vert = sq_bb.shift_north(2).union(sq_bb.shift_south(2));
    let horiz = sq_bb.no_wrap_shift_east(2).union(sq_bb.no_wrap_shift_west(2));

    vert.west_one().union(vert.east_one())
        .union(horiz.north_one().union(horiz.south_one()))
});

const PAWN_ATTACKS: [[Bitboard; NUM_SQUARES as usize]; NUM_COLORS as usize] = [
    init_lookup!(|sq_bb|, pawn_setwise(sq_bb, Color::White)),
    init_lookup!(|sq_bb|, pawn_setwise(sq_bb, Color::Black)),
];

const MAGIC_LOOKUP: MagicLookup = include!(concat!(env!("OUT_DIR"), "/magic_lookup_init.rs"));

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

pub const fn pawn(sq: Square, color: Color) -> Bitboard {
    PAWN_ATTACKS[color.as_index()][sq.as_index()]
}

pub const fn pawn_setwise(pawns: Bitboard, color: Color) -> Bitboard {
    match color {
        Color::White => pawns.northeast_one().union(pawns.northwest_one()),
        Color::Black => pawns.southeast_one().union(pawns.southwest_one()),
    }
}

pub const fn pawn_single_push(pawns: Bitboard, empty: Bitboard, color: Color) -> Bitboard {
    match color {
        Color::White => pawns.north_one().intersection(empty),
        Color::Black => pawns.south_one().intersection(empty),
    }
}

pub const fn pawn_double_push(single_pushes: Bitboard, empty: Bitboard, color: Color) -> Bitboard {
    match color {
        Color::White => single_pushes
            .north_one()
            .intersection(empty)
            .intersection(Bitboard::RANK_4),
        Color::Black => single_pushes
            .south_one()
            .intersection(empty)
            .intersection(Bitboard::RANK_5),
    }
}

pub fn generic(piece: Piece, sq: Square, occupied: Bitboard) -> Bitboard {
    match piece {
        Piece::KNIGHT => knight(sq),
        Piece::BISHOP => bishop(sq, occupied),
        Piece::ROOK => rook(sq, occupied),
        Piece::QUEEN => queen(sq, occupied),
        Piece::KING => king(sq),
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::{Bitboard, Square};
    use crate::board::attacks;
    use crate::bb_from_squares;
    use crate::board::board_representation::{Board, Color, Piece};

    #[test]
    fn king_attack_test() {
        let pos_1 = Square::A1;
        let expected_1 = bb_from_squares!(A2, B2, B1);
        assert_eq!(attacks::king(pos_1), expected_1);

        let pos_2 = Square::E4;
        let expected_2 = bb_from_squares!(E3, E5, D3, D4, D5, F3, F4, F5);
        assert_eq!(attacks::king(pos_2), expected_2);
    }

    #[test]
    fn knight_attack_test() {
        let pos_1 = Square::A1;
        let expected_1 = bb_from_squares!(C2, B3);
        assert_eq!(attacks::knight(pos_1), expected_1);

        let pos_2 = Square::E4;
        let expected_2 = bb_from_squares!(D2, F2, C3, G3, C5, G5, D6, F6);
        assert_eq!(attacks::knight(pos_2), expected_2);
    }

    #[test]
    fn bishop_attack_test() {
        let board = Board::from_fen("1k6/ppp5/5n2/2b1pB1r/8/2P3BP/P1P2PP1/3R2K1 w - - 1 25");
        let attacks = attacks::bishop(Square::F5, board.occupied());

        let expected = bb_from_squares!(C2, D3, H3, E4, G4, E6, G6, D7, H7, C8);
        assert_eq!(attacks, expected);
    }

    #[test]
    fn rook_attack_test() {
        let board = Board::from_fen("1k6/ppp5/5n2/2b1pB1r/8/2P3BP/P1P2PP1/3R2K1 w - - 1 25");
        let attacks = attacks::rook(Square::H5, board.occupied());

        let expected = bb_from_squares!(F5, G5, H3, H4, H6, H7, H8);
        assert_eq!(attacks, expected);
    }

    #[test]
    fn queen_attack_test() {
        let board =
            Board::from_fen("2kr4/pp3pp1/2p3rp/2p1p3/1PB1P3/1R1P1q2/P1P2P1Q/5K2 b - - 6 26");
        let attacks = attacks::queen(Square::F3, board.occupied());

        let expected =
            bb_from_squares!(D1, H1, E2, F2, G2, D3, E3, G3, H3, E4, F4, G4, F5, H5, F6, F7);
        assert_eq!(attacks, expected);
    }

    #[test]
    fn pawn_attack_test() {
        let board =
            Board::from_fen("2kr4/pp3pp1/2p3rp/2p1p3/1PB1P3/1R1P1q2/P1P2P1Q/5K2 b - - 6 26");
        let color = Color::White;
        let w_pawns = board.piece_bb(Piece::PAWN, color);

        let attacks = attacks::pawn_setwise(w_pawns, color);

        let expected = bb_from_squares!(B3, D3, E3, G3, C4, E4, A5, C5, D5, F5);
        assert_eq!(attacks, expected);
    }

    #[test]
    fn pawn_single_push_test() {
        let board =
            Board::from_fen("2kr4/pp3pp1/2p3rp/2p1p3/1PB1P3/1R1P1q2/P1P2P1Q/5K2 b - - 6 26");
        let color = Color::White;
        let w_pawns = board.piece_bb(Piece::PAWN, color);

        let moves = attacks::pawn_single_push(w_pawns, board.empty(), color);

        let expected = bb_from_squares!(A3, C3, D4, B5);
        assert_eq!(moves, expected);
    }

    #[test]
    fn pawn_double_push_test() {
        let board =
            Board::from_fen("2kr4/pp3pp1/2p3rp/2p1p3/1PB1P3/1R1P1q2/P1P2P1Q/5K2 b - - 6 26");
        let color = Color::Black;
        let b_pawns = board.piece_bb(Piece::PAWN, color);
        let single_pushs = attacks::pawn_single_push(b_pawns, board.empty(), color);

        let moves = attacks::pawn_double_push(single_pushs, board.empty(), color);

        let expected = bb_from_squares!(F5, B5, A5);
        assert_eq!(moves, expected);
    }
}
