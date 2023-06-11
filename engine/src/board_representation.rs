use crate::chess_move::Move;
use crate::tuple_constants_enum;
use crate::zobrist::ZobristHash;
use crate::zobrist_stack::ZobristStack;
use crate::{attacks, chess_move::Flag};
use std::ops::{BitAnd, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

type Row = u8;
type Col = u8;

pub const NUM_SQUARES: u8 = 64;
pub const NUM_PIECES: u8 = 6;
pub const NUM_COLORS: u8 = 2;
pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0";

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum Color {
    #[default]
    White,
    Black,
}

impl Color {
    pub const LIST: [Self; NUM_COLORS as usize] = [Self::White, Self::Black];

    pub const fn flip(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    pub const fn as_index(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece(u8);

impl Piece {
    #[rustfmt::skip]
    tuple_constants_enum!(Self,
        KNIGHT,
        BISHOP,
        ROOK,
        QUEEN,
        PAWN,
        KING,
        NONE
    );

    pub const LIST: [Self; NUM_PIECES as usize] = [
        Self::KNIGHT,
        Self::BISHOP,
        Self::ROOK,
        Self::QUEEN,
        Self::PAWN,
        Self::KING,
    ];

    pub const fn new(data: u8) -> Self {
        Self(data)
    }

    fn as_char(self, color: Color) -> Option<char> {
        let mut ch = match self {
            Self::KNIGHT => 'n',
            Self::BISHOP => 'b',
            Self::ROOK => 'r',
            Self::QUEEN => 'q',
            Self::PAWN => 'p',
            Self::KING => 'k',
            _ => return None,
        };

        if color == Color::White {
            ch = ch.to_uppercase().next().unwrap();
        }

        Some(ch)
    }

    pub const fn from_char(char: char) -> Option<Self> {
        match char {
            'n' | 'N' => Some(Self::KNIGHT),
            'b' | 'B' => Some(Self::BISHOP),
            'r' | 'R' => Some(Self::ROOK),
            'q' | 'Q' => Some(Self::QUEEN),
            'p' | 'P' => Some(Self::PAWN),
            'k' | 'K' => Some(Self::KING),
            _ => None,
        }
    }

    pub const fn as_index(self) -> usize {
        self.0 as usize
    }

    pub const fn as_u16(self) -> u16 {
        self.0 as u16
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Square(u8);

impl Square {
    #[rustfmt::skip]
    tuple_constants_enum!(Self,
        A1, B1, C1, D1, E1, F1, G1, H1,
        A2, B2, C2, D2, E2, F2, G2, H2,
        A3, B3, C3, D3, E3, F3, G3, H3,
        A4, B4, C4, D4, E4, F4, G4, H4,
        A5, B5, C5, D5, E5, F5, G5, H5,
        A6, B6, C6, D6, E6, F6, G6, H6,
        A7, B7, C7, D7, E7, F7, G7, H7,
        A8, B8, C8, D8, E8, F8, G8, H8
    );

    pub const fn new(data: u8) -> Self {
        Self(data)
    }

    pub const fn as_bitboard(self) -> Bitboard {
        Bitboard::new(1 << self.0)
    }

    pub const fn as_u16(self) -> u16 {
        self.0 as u16
    }

    pub const fn as_index(self) -> usize {
        self.0 as usize
    }

    const fn rank(self) -> u8 {
        self.0 / 8
    }

    pub const fn file(self) -> u8 {
        self.0 % 8
    }

    pub fn as_string(self) -> String {
        let col: Col = self.file();
        let row: Row = self.rank();

        let col_char = (col + 97) as char;
        let row_char = (row + 49) as char;

        format!("{col_char}{row_char}")
    }

    pub fn from_string(str: &str) -> Option<Self> {
        if str.len() != 2 {
            return None;
        }

        let mut chars = str.chars();
        let col_char = chars.next().unwrap();
        let row_char = chars.next().unwrap();

        let col: Col = (col_char as Col) - 97;
        let row: Row = (row_char as Col) - 49;

        Some(Self::new(row * 8 + col))
    }

    pub const fn retreat(self, count: u8, color: Color) -> Self {
        match color {
            Color::White => Self::new(self.0 - (8 * count)),
            Color::Black => Self::new(self.0 + (8 * count)),
        }
    }

    pub const fn left(self, count: u8) -> Self {
        Self(self.0 - count)
    }

    pub const fn right(self, count: u8) -> Self {
        Self(self.0 + count)
    }

    pub const fn is_attacked(self, board: &Board) -> bool {
        let opp_color = board.color_to_move.flip();
        let occupied = board.occupied();

        let opp_king = board.piece_bb(Piece::KING, opp_color);
        let opp_knights = board.piece_bb(Piece::KNIGHT, opp_color);
        let opp_pawns = board.piece_bb(Piece::PAWN, opp_color);
        let opp_bishops = board.piece_bb(Piece::BISHOP, opp_color);
        let opp_rooks = board.piece_bb(Piece::ROOK, opp_color);
        let opp_queens = board.piece_bb(Piece::QUEEN, opp_color);

        let hv_sliders = opp_rooks.union(opp_queens);
        let d_sliders = opp_bishops.union(opp_queens);

        attacks::king(self)
            .intersection(opp_king)
            .union(attacks::knight(self).intersection(opp_knights))
            .union(attacks::pawn(self, board.color_to_move).intersection(opp_pawns))
            .union(attacks::rook(self, occupied).intersection(hv_sliders))
            .union(attacks::bishop(self, occupied).intersection(d_sliders))
            .is_not_empty()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Bitboard {
    data: u64,
}

impl Bitboard {
    pub const A_FILE: Self = Self::new(0x0101010101010101);
    pub const H_FILE: Self = Self::new(0x8080808080808080);

    pub const RANK_1: Self = Self::new(0x00000000000000ff);
    pub const RANK_2: Self = Self::new(0x000000000000ff00);
    pub const RANK_4: Self = Self::new(0x00000000ff000000);
    pub const RANK_5: Self = Self::new(0x000000ff00000000);
    pub const RANK_7: Self = Self::new(0x00ff000000000000);
    pub const RANK_8: Self = Self::new(0xff00000000000000);

    pub const fn new(data: u64) -> Self {
        Self { data }
    }

    pub const fn as_u64(self) -> u64 {
        self.data
    }

    // redundant implementations for const operations
    pub const fn complement(self) -> Self {
        Self { data: !self.data }
    }

    pub const fn union(self, rhs: Self) -> Self {
        Self {
            data: self.data | rhs.data,
        }
    }

    pub const fn intersection(self, rhs: Self) -> Self {
        Self {
            data: self.data & rhs.data,
        }
    }

    pub const fn without(self, rhs: Self) -> Self {
        Self {
            data: self.data & !rhs.data,
        }
    }

    const fn xor(self, rhs: Self) -> Self {
        Self {
            data: self.data ^ rhs.data,
        }
    }

    const fn l_shift(self, shift: u8) -> Self {
        Self {
            data: self.data << shift,
        }
    }

    const fn r_shift(self, shift: u8) -> Self {
        Self {
            data: self.data >> shift,
        }
    }

    pub const fn is_not_empty(self) -> bool {
        self.data > 0
    }

    pub const fn overlaps(self, rhs: Self) -> bool {
        self.intersection(rhs).is_not_empty()
    }

    pub const fn popcount(self) -> u32 {
        self.data.count_ones()
    }

    const fn lsb(self) -> Square {
        Square::new(self.data.trailing_zeros() as u8)
    }

    fn reset_lsb(&mut self) {
        self.data = self.data & (self.data - 1);
    }

    pub fn pop_lsb(&mut self) -> Square {
        debug_assert!(self.is_not_empty());
        let sq = self.lsb();
        self.reset_lsb();
        sq
    }

    pub const fn north_one(self) -> Self {
        self.l_shift(8)
    }

    pub const fn northeast_one(self) -> Self {
        self.intersection(Self::H_FILE.complement()).l_shift(9)
    }

    pub const fn east_one(self) -> Self {
        self.intersection(Self::H_FILE.complement()).l_shift(1)
    }

    pub const fn southeast_one(self) -> Self {
        self.intersection(Self::H_FILE.complement()).r_shift(7)
    }

    pub const fn south_one(self) -> Self {
        self.r_shift(8)
    }

    pub const fn southwest_one(self) -> Self {
        self.intersection(Self::A_FILE.complement()).r_shift(9)
    }

    pub const fn west_one(self) -> Self {
        self.intersection(Self::A_FILE.complement()).r_shift(1)
    }

    pub const fn northwest_one(self) -> Self {
        self.intersection(Self::A_FILE.complement()).l_shift(7)
    }

    pub const fn shift_north(self, shift: u8) -> Self {
        self.l_shift(8 * shift)
    }

    pub const fn shift_south(self, shift: u8) -> Self {
        self.r_shift(8 * shift)
    }

    pub const fn shift_east(self, shift: u8) -> Self {
        self.l_shift(shift)
    }

    pub const fn shift_west(self, shift: u8) -> Self {
        self.r_shift(shift)
    }

    pub const fn no_wrap_shift_east(self, count: u8) -> Self {
        let mut result = self;
        let mut i = 0;
        while i < count {
            result = result.east_one();
            i += 1;
        }
        result
    }

    pub const fn no_wrap_shift_west(self, count: u8) -> Self {
        let mut result = self;
        let mut i = 0;
        while i < count {
            result = result.west_one();
            i += 1;
        }
        result
    }

    pub fn print(self) {
        for i in 0..NUM_SQUARES {
            let bitset = fen_index_as_bitboard(i);
            if bitset.overlaps(self) {
                print!("X ");
            } else {
                print!(". ");
            }

            if (i + 1) % 8 == 0 {
                println!();
            }
        }
        println!();
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(rhs)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.data |= rhs.data;
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.data ^= rhs.data;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct CastleRights(u8);

impl CastleRights {
    const W_KINGSIDE_MASK: u8 = 0b0001;
    const W_QUEENSIDE_MASK: u8 = 0b0010;
    const B_KINGSIDE_MASK: u8 = 0b0100;
    const B_QUEENSIDE_MASK: u8 = 0b1000;

    const KS_THRU_SQUARE: [Square; NUM_COLORS as usize] = [Square::F1, Square::F8];
    const QS_THRU_SQUARES: [[Square; 2]; NUM_COLORS as usize] =
        [[Square::C1, Square::D1], [Square::C8, Square::D8]];
    const KS_OCC_MASK: [Bitboard; NUM_COLORS as usize] = [
        Square::F1.as_bitboard().union(Square::G1.as_bitboard()),
        Square::F8.as_bitboard().union(Square::G8.as_bitboard()),
    ];
    const QS_OCC_MASK: [Bitboard; NUM_COLORS as usize] = [
        Square::B1
            .as_bitboard()
            .union(Square::C1.as_bitboard().union(Square::D1.as_bitboard())),
        Square::B8
            .as_bitboard()
            .union(Square::C8.as_bitboard().union(Square::D8.as_bitboard())),
    ];

    const UPDATE_MASKS: [u8; NUM_SQUARES as usize] = {
        let mut table = [0b1111; NUM_SQUARES as usize];
        table[Square::A1.as_index()] ^= Self::W_QUEENSIDE_MASK;
        table[Square::H1.as_index()] ^= Self::W_KINGSIDE_MASK;
        table[Square::E1.as_index()] ^= Self::W_KINGSIDE_MASK | Self::W_QUEENSIDE_MASK;
        table[Square::A8.as_index()] ^= Self::B_QUEENSIDE_MASK;
        table[Square::H8.as_index()] ^= Self::B_KINGSIDE_MASK;
        table[Square::E8.as_index()] ^= Self::B_KINGSIDE_MASK | Self::B_QUEENSIDE_MASK;
        table
    };

    const fn new(data: u8) -> Self {
        Self(data)
    }

    const fn has_kingside(self, color: Color) -> bool {
        match color {
            Color::White => (self.0 & Self::W_KINGSIDE_MASK) > 0,
            Color::Black => (self.0 & Self::B_KINGSIDE_MASK) > 0,
        }
    }

    const fn has_queenside(self, color: Color) -> bool {
        match color {
            Color::White => (self.0 & Self::W_QUEENSIDE_MASK) > 0,
            Color::Black => (self.0 & Self::B_QUEENSIDE_MASK) > 0,
        }
    }

    pub const fn can_ks_castle(self, board: &Board) -> bool {
        let color = board.color_to_move;
        let thru_sq = Self::KS_THRU_SQUARE[color.as_index()];
        let occ_mask = Self::KS_OCC_MASK[color.as_index()];
        self.has_kingside(color)
            && !(occ_mask.overlaps(board.occupied())
                || thru_sq.is_attacked(board)
                || board.king_sq().is_attacked(board))
    }

    pub const fn can_qs_castle(self, board: &Board) -> bool {
        let color = board.color_to_move;
        let thru_sq_1 = Self::QS_THRU_SQUARES[color.as_index()][0];
        let thru_sq_2 = Self::QS_THRU_SQUARES[color.as_index()][1];
        let occ_mask = Self::QS_OCC_MASK[color.as_index()];
        self.has_queenside(color)
            && !(occ_mask.overlaps(board.occupied())
                || thru_sq_1.is_attacked(board)
                || thru_sq_2.is_attacked(board)
                || board.king_sq().is_attacked(board))
    }

    pub const fn as_index(self) -> usize {
        self.0 as usize
    }

    fn update(&mut self, mv: Move) {
        self.0 &= Self::UPDATE_MASKS[mv.from().as_index()] & Self::UPDATE_MASKS[mv.to().as_index()];
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Board {
    pub all: [Bitboard; NUM_COLORS as usize],
    pub pieces: [Bitboard; NUM_PIECES as usize],
    pub color_to_move: Color,
    pub ep_sq: Option<Square>,
    pub castle_rights: CastleRights,
    pub halfmoves: u16,
}

const fn fen_index_as_bitboard(i: u8) -> Bitboard {
    let row = 7 - (i / 8);
    let col = i % 8;
    Square(row * 8 + col).as_bitboard()
}

impl Board {
    pub fn print(&self) {
        for i in 0..NUM_SQUARES {
            let bitset = fen_index_as_bitboard(i);
            let mut ch = '.';

            for piece in 0..NUM_PIECES {
                if bitset.overlaps(self.pieces[piece as usize]) {
                    let color = if bitset.overlaps(self.all[Color::White as usize]) {
                        Color::White
                    } else {
                        Color::Black
                    };
                    ch = Piece(piece).as_char(color).unwrap();
                }
            }

            if (i + 1) % 8 == 0 {
                println!("{ch} ");
            } else {
                print!("{ch} ");
            }
        }
        println!();
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut board = Self::default();
        let mut i: u8 = 0;
        let split_fen = fen.split_whitespace().collect::<Vec<&str>>();

        let board_info_string = split_fen[0].chars();
        let color_char = split_fen[1].chars().next().unwrap();
        let castling_rights = split_fen[2].chars();
        let ep_sq = split_fen[3];

        for ch in board_info_string {
            assert!(i < NUM_SQUARES);
            let bitset = fen_index_as_bitboard(i);

            if ch.is_numeric() {
                let digit = ch.to_digit(10).unwrap();
                assert!((1..9).contains(&digit), "Invalid FEN number");
                i += digit as u8;
            } else if ch.is_alphabetic() {
                match ch.to_lowercase().next().unwrap() {
                    'n' => board.pieces[Piece::KNIGHT.as_index()] |= bitset,
                    'b' => board.pieces[Piece::BISHOP.as_index()] |= bitset,
                    'r' => board.pieces[Piece::ROOK.as_index()] |= bitset,
                    'q' => board.pieces[Piece::QUEEN.as_index()] |= bitset,
                    'p' => board.pieces[Piece::PAWN.as_index()] |= bitset,
                    'k' => board.pieces[Piece::KING.as_index()] |= bitset,
                    _ => panic!("Invalid FEN"),
                };

                if ch.is_uppercase() {
                    board.all[Color::White as usize] |= bitset;
                } else {
                    board.all[Color::Black as usize] |= bitset;
                }

                i += 1;
            } else if ch != '/' {
                panic!("Invalid FEN character");
            }
        }

        assert!(
            color_char == 'w' || color_char == 'b',
            "Invalid color specifier"
        );
        if color_char == 'b' {
            board.color_to_move = Color::Black;
        }

        let mut castle_data: u8 = 0;
        for c in castling_rights {
            match c {
                'K' => castle_data |= CastleRights::W_KINGSIDE_MASK,
                'Q' => castle_data |= CastleRights::W_QUEENSIDE_MASK,
                'k' => castle_data |= CastleRights::B_KINGSIDE_MASK,
                'q' => castle_data |= CastleRights::B_QUEENSIDE_MASK,
                _ => (),
            }
        }
        board.castle_rights = CastleRights::new(castle_data);

        board.ep_sq = Square::from_string(ep_sq);

        board
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_fen(&self) -> String {
        let mut pos = String::new();
        let mut blank_space: u8 = 0;

        for i in 0..NUM_SQUARES {
            let bitset = fen_index_as_bitboard(i);
            blank_space += 1;

            for piece in 0..NUM_PIECES {
                if bitset.overlaps(self.pieces[piece as usize]) {
                    let color = if bitset.overlaps(self.all[Color::White as usize]) {
                        Color::White
                    } else {
                        Color::Black
                    };
                    let ch = Piece(piece).as_char(color).unwrap();
                    blank_space -= 1;

                    if blank_space > 0 {
                        pos.push(char::from_digit(blank_space.into(), 10).unwrap());
                    }

                    blank_space = 0;
                    pos.push(ch);
                }
            }

            if (i + 1) % 8 == 0 {
                if blank_space > 0 {
                    pos.push(char::from_digit(blank_space.into(), 10).unwrap());
                }
                blank_space = 0;

                if (i + 1) < NUM_SQUARES {
                    pos.push('/');
                }
            }
        }

        let color_char = if self.color_to_move == Color::White {
            'w'
        } else {
            'b'
        };

        let mut castling_rights = Vec::<char>::new();
        if self.castle_rights.has_kingside(Color::White) {
            castling_rights.push('K');
        }
        if self.castle_rights.has_queenside(Color::White) {
            castling_rights.push('Q');
        }
        if self.castle_rights.has_kingside(Color::Black) {
            castling_rights.push('k');
        }
        if self.castle_rights.has_queenside(Color::Black) {
            castling_rights.push('q');
        }
        if castling_rights.is_empty() {
            castling_rights.push('-');
        }

        let castling_rights: String = castling_rights.into_iter().collect();

        #[allow(clippy::redundant_closure_for_method_calls)]
        let ep = self
            .ep_sq
            .map_or_else(|| "-".to_string(), |sq| sq.as_string());

        // TODO: handle these later
        let halfmoves = "0";
        let fullmoves = '0';

        format!("{pos} {color_char} {castling_rights} {ep} {halfmoves} {fullmoves}")
    }

    pub const fn occupied(&self) -> Bitboard {
        self.all[Color::White as usize].union(self.all[Color::Black as usize])
    }

    pub const fn empty(&self) -> Bitboard {
        self.occupied().complement()
    }

    pub const fn us(&self) -> Bitboard {
        self.all[self.color_to_move as usize]
    }

    pub const fn them(&self) -> Bitboard {
        self.all[self.color_to_move.flip() as usize]
    }

    pub const fn piece_bb(&self, piece: Piece, color: Color) -> Bitboard {
        self.all[color.as_index()].intersection(self.pieces[piece.as_index()])
    }

    pub const fn king_sq(&self) -> Square {
        self.piece_bb(Piece::KING, self.color_to_move).lsb()
    }

    pub const fn promotable_pawns(&self) -> Bitboard {
        let color = self.color_to_move;
        let pawns = self.piece_bb(Piece::PAWN, color);
        match color {
            Color::White => pawns.intersection(Bitboard::RANK_7),
            Color::Black => pawns.intersection(Bitboard::RANK_2),
        }
    }

    pub fn piece_on_sq(&self, sq: Square) -> Piece {
        let bitset = sq.as_bitboard();
        for piece in Piece::LIST {
            if bitset.overlaps(self.pieces[piece.as_index()]) {
                return piece;
            }
        }
        Piece::NONE
    }

    fn toggle(&mut self, mask: Bitboard, piece: Piece, color: Color) {
        self.all[color.as_index()] ^= mask;
        self.pieces[piece.as_index()] ^= mask;
    }

    fn toggle_promotion(&mut self, mask: Bitboard, promo_piece: Piece, hash_base: &mut ZobristHash, to_sq: Square) {
        self.pieces[Piece::PAWN.as_index()] ^= mask;
        self.pieces[promo_piece.as_index()] ^= mask;
        hash_base.hash_piece(self.color_to_move, Piece::PAWN, to_sq);
        hash_base.hash_piece(self.color_to_move, promo_piece, to_sq);
    }

    fn toggle_capture_promotion(
        &mut self,
        mask: Bitboard,
        captured_piece: Piece,
        promo_piece: Piece,
        hash_base: &mut ZobristHash,
        to_sq: Square
    ) {
        self.toggle_promotion(mask, promo_piece, hash_base, to_sq);
        self.toggle(mask, captured_piece, self.color_to_move.flip());
    }

    fn ep_sq_after_double_push(
        &self,
        to_sq: Square,
        hash_base: &mut ZobristHash,
    ) -> Option<Square> {
        let ep_sq = to_sq.retreat(1, self.color_to_move);
        let opp_pawns = self.piece_bb(Piece::PAWN, self.color_to_move.flip());

        if attacks::pawn(ep_sq, self.color_to_move).overlaps(opp_pawns) {
            hash_base.hash_ep(ep_sq);
            Some(ep_sq)
        } else {
            None
        }
    }

    #[rustfmt::skip]
    #[doc="returns success: bool"]
    pub fn try_play_move(&mut self, mv: Move, zobrist_stack: &mut ZobristStack, mut hash_base: ZobristHash) -> bool {
        let color = self.color_to_move;
        let opp_color = color.flip();

        let to_sq = mv.to();
        let from_sq = mv.from();
        let to_bb = to_sq.as_bitboard();
        let from_bb = from_sq.as_bitboard();
        let piece = self.piece_on_sq(from_sq);
        debug_assert!(piece != Piece::NONE);

        if piece == Piece::PAWN {
            self.halfmoves = 0;
        }

        let captured_piece = if mv.is_capture() {
            self.halfmoves = 0;
            self.piece_on_sq(mv.to())
        } else {
            self.halfmoves += 1;
            Piece::NONE
        };

        if captured_piece != Piece::NONE {
            hash_base.hash_piece(opp_color, captured_piece, to_sq);
        }

        self.toggle(from_bb | to_bb, piece, color);
        hash_base.hash_piece(color, piece, from_sq);
        hash_base.hash_piece(color, piece, to_sq);

        self.ep_sq = None;

        let flag = mv.flag();
        match flag {
            Flag::NONE => (),
            Flag::CAPTURE => self.toggle(to_bb, captured_piece, opp_color),
            Flag::DOUBLE_PUSH => self.ep_sq = self.ep_sq_after_double_push(to_sq, &mut hash_base),
            Flag::KS_CASTLE => {
                let rook_to = from_sq.right(1);
                let rook_from = from_sq.right(3);
                self.toggle(rook_to.as_bitboard() | rook_from.as_bitboard(), Piece::ROOK, color);
                hash_base.hash_piece(color, Piece::ROOK, rook_to);
                hash_base.hash_piece(color, Piece::ROOK, rook_from);
            }
            Flag::QS_CASTLE => {
                let rook_to = from_sq.left(1);
                let rook_from = from_sq.left(4);
                self.toggle(rook_to.as_bitboard() | rook_from.as_bitboard(), Piece::ROOK, color);
                hash_base.hash_piece(color, Piece::ROOK, rook_to);
                hash_base.hash_piece(color, Piece::ROOK, rook_from);
            }
            Flag::QUEEN_PROMO => self.toggle_promotion(to_bb, Piece::QUEEN, &mut hash_base, to_sq),
            Flag::QUEEN_CAPTURE_PROMO => self.toggle_capture_promotion(to_bb, captured_piece, Piece::QUEEN, &mut hash_base, to_sq),
            Flag::KNIGHT_PROMO => self.toggle_promotion(to_bb, Piece::KNIGHT, &mut hash_base, to_sq),
            Flag::KNIGHT_CAPTURE_PROMO => self.toggle_capture_promotion(to_bb, captured_piece, Piece::KNIGHT, &mut hash_base, to_sq),
            Flag::BISHOP_PROMO => self.toggle_promotion(to_bb, Piece::BISHOP, &mut hash_base, to_sq),
            Flag::BISHOP_CAPTURE_PROMO => self.toggle_capture_promotion(to_bb, captured_piece, Piece::BISHOP, &mut hash_base, to_sq),
            Flag::ROOK_PROMO => self.toggle_promotion(to_bb, Piece::ROOK, &mut hash_base, to_sq),
            Flag::ROOK_CAPTURE_PROMO => self.toggle_capture_promotion(to_bb, captured_piece, Piece::ROOK, &mut hash_base, to_sq),
            Flag::EP => {
                let ep_sq = to_sq.retreat(1, color);
                self.toggle(ep_sq.as_bitboard(), Piece::PAWN, opp_color);
                hash_base.hash_piece(opp_color, Piece::PAWN, ep_sq);
            }
            _ => panic!("Invalid Move!"),
        }

        if self.king_sq().is_attacked(self) {
            return false;
        }

        self.castle_rights.update(mv);
        hash_base.hash_castling(self.castle_rights);

        self.color_to_move = self.color_to_move.flip();

        debug_assert_eq!(zobrist_stack.current_zobrist_hash().combine(hash_base), ZobristHash::complete(self));
        zobrist_stack.add_hash(hash_base);

        true
    }

    pub fn simple_try_play_move(&mut self, mv: Move) -> bool {
        let mut dummy_stack = ZobristStack::new(self);
        let dummy_base = ZobristHash::incremental_update_base(self);
        self.try_play_move(mv, &mut dummy_stack, dummy_base)
    }

    pub const fn fifty_move_draw(&self) -> bool {
        self.halfmoves >= 100
    }
}

#[cfg(test)]
mod tests {
    use super::{Bitboard, Board, Color, Square, START_FEN};
    use crate::{bb_from_squares, board_representation::CastleRights};

    #[test]
    fn bit_and_works() {
        let bb1 = Bitboard::new(0b111);
        let bb2 = Bitboard::new(0b101);
        let expected = Bitboard::new(0b101);

        assert_eq!(bb1 & bb2, expected);
    }

    #[test]
    fn bit_or_works() {
        let bb1 = Bitboard::new(0b011);
        let bb2 = Bitboard::new(0b101);
        let expected = Bitboard::new(0b111);

        assert_eq!(bb1 | bb2, expected);
    }

    #[test]
    fn bit_xor_works() {
        let bb1 = Bitboard::new(0b011);
        let bb2 = Bitboard::new(0b101);
        let expected = Bitboard::new(0b110);

        assert_eq!(bb1 ^ bb2, expected);
    }

    #[test]
    fn bit_not_works() {
        let bb = Bitboard::new(0xFFFF0000FFFF0000);
        let expected = Bitboard::new(0x0000FFFF0000FFFF);

        assert_eq!(!bb, expected);
    }

    #[test]
    fn combinations_work() {
        let data1: u64 = 894378932;
        let data2: u64 = 18981928111;

        let bb1 = Bitboard::new(data1);
        let bb2 = Bitboard::new(data2);
        let expected = Bitboard::new(data1 & !data2);

        assert_eq!(bb1 & !bb2, expected);
    }

    #[test]
    fn sq_to_bb_works() {
        let bitset = Square::A3.as_bitboard();
        let expected = Bitboard::new(0b00010000000000000000);

        assert_eq!(bitset, expected);
    }

    #[test]
    fn pop_lsb_works() {
        let mut bb = Bitboard::new(0b0111100111000);
        let sq = bb.pop_lsb();

        let expected_bb = Bitboard::new(0b0111100110000);
        let expected_sq = Square::new(3);

        assert_eq!(bb, expected_bb);
        assert_eq!(sq, expected_sq);
    }

    #[test]
    fn correctly_interprets_startpos_fen() {
        let actual = Board::from_fen(START_FEN);

        let white = Bitboard::new(0x000000000000ffff);
        let black = Bitboard::new(0xffff000000000000);

        let knights = Bitboard::new(0x4200000000000042);
        let bishops = Bitboard::new(0x2400000000000024);
        let rooks = Bitboard::new(0x8100000000000081);
        let queens = bb_from_squares!(D1, D8);
        let pawns = Bitboard::new(0x00ff00000000ff00);
        let kings = bb_from_squares!(E1, E8);

        let expected = Board {
            all: [white, black],
            pieces: [knights, bishops, rooks, queens, pawns, kings],
            color_to_move: Color::White,
            castle_rights: CastleRights::new(0b1111),
            ep_sq: None,
            halfmoves: 0,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn to_fen_works_with_startpos() {
        let startpos_board = Board::from_fen(START_FEN);
        let actual = startpos_board.to_fen();

        let expected = START_FEN;

        assert_eq!(actual, expected);
    }

    #[test]
    fn sq_to_str_works() {
        assert_eq!(Square::A1.as_string(), "a1");
        assert_eq!(Square::A6.as_string(), "a6");
        assert_eq!(Square::H8.as_string(), "h8");
    }

    #[test]
    fn sq_from_str_works() {
        assert_eq!(Square::from_string("a1").unwrap(), Square::A1);
        assert_eq!(Square::from_string("a6").unwrap(), Square::A6);
        assert_eq!(Square::from_string("h8").unwrap(), Square::H8);
    }
}
