use std::ops::{BitAnd, BitOr, BitOrAssign, BitXor, Not};

use crate::tuple_constants_enum;

type Rank = u8;
type File = u8;

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

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
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
        NONE_PIECE
    );

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

    const fn as_index(self) -> usize {
        self.0 as usize
    }

    pub const fn as_u16(self) -> u16 {
        self.0 as u16
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Bitboard {
    data: u64,
}

impl Bitboard {
    pub const A_FILE: Self = Self::new(0x0101010101010101);
    pub const H_FILE: Self = Self::new(0x8080808080808080);

    pub const fn new(data: u64) -> Self {
        Self { data }
    }

    // redundant implementations for const operations
    const fn complement(self) -> Self {
        Self { data: !self.data }
    }

    pub const fn union(self, rhs: Self) -> Self {
        Self {
            data: self.data | rhs.data,
        }
    }

    const fn intersection(self, rhs: Self) -> Self {
        Self {
            data: self.data & rhs.data,
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

    const fn is_not_empty(self) -> bool {
        self.data > 0
    }

    const fn overlaps(self, rhs: Self) -> bool {
        self.intersection(rhs).is_not_empty()
    }

    const fn popcount(self) -> u32 {
        self.data.count_ones()
    }

    const fn lsb(self) -> u32 {
        self.data.trailing_zeros()
    }

    const fn lsb_as_bitboard(self) -> Self {
        Self {
            data: self.data & ((!self.data) + 1),
        }
    }

    fn reset_lsb(&mut self) {
        self.data = self.data & (self.data - 1);
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

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Board {
    pub all: [Bitboard; NUM_COLORS as usize],
    pub pieces: [Bitboard; NUM_PIECES as usize],
    pub color_to_move: Color,
}

const fn fen_index_as_bitboard(i: u8) -> Bitboard {
    let row = 7 - (i / 8);
    let col = i % 8;
    Square(row * 8 + col).as_bitboard()
}

impl Board {
    fn print(&self) {
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
    }

    fn from_fen(fen: &str) -> Self {
        let mut board = Self::default();
        let mut i: u8 = 0;
        let split_fen = fen.split_whitespace().collect::<Vec<&str>>();
        let board_info_string = split_fen[0].chars();
        let color_char = split_fen[1].chars().next().unwrap();

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

        board
    }

    fn to_fen(&self) -> String {
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

        // TODO: handle these later
        let castling_rights = "KQkq";
        let ep = "-";
        let halfmoves = "0";
        let fullmoves = '0';

        format!("{pos} {color_char} {castling_rights} {ep} {halfmoves} {fullmoves}")
    }
}

#[cfg(test)]
mod tests {
    use super::{Bitboard, Board, Color, Square, START_FEN};

    #[test]
    fn bit_and_works() {
        let bb1 = Bitboard { data: 0b111 };
        let bb2 = Bitboard { data: 0b101 };
        let expected = Bitboard { data: 0b101 };
        assert_eq!(bb1 & bb2, expected);
    }

    #[test]
    fn bit_or_works() {
        let bb1 = Bitboard { data: 0b011 };
        let bb2 = Bitboard { data: 0b101 };
        let expected = Bitboard { data: 0b111 };
        assert_eq!(bb1 | bb2, expected);
    }

    #[test]
    fn bit_xor_works() {
        let bb1 = Bitboard { data: 0b011 };
        let bb2 = Bitboard { data: 0b101 };
        let expected = Bitboard { data: 0b110 };
        assert_eq!(bb1 ^ bb2, expected);
    }

    #[test]
    fn bit_not_works() {
        let bb = Bitboard {
            data: 0xFFFF0000FFFF0000,
        };
        let expected = Bitboard {
            data: 0x0000FFFF0000FFFF,
        };
        assert_eq!(!bb, expected);
    }

    #[test]
    fn combinations_work() {
        let data1: u64 = 894378932;
        let data2: u64 = 18981928111;

        let bb1 = Bitboard { data: data1 };
        let bb2 = Bitboard { data: data2 };
        let expected = Bitboard {
            data: data1 & !data2,
        };
        assert_eq!(bb1 & !bb2, expected);
    }

    #[test]
    fn sq_to_bb_works() {
        let bitset = Square::A3.as_bitboard();
        let expected = Bitboard {
            data: 0b00010000000000000000,
        };
        assert_eq!(bitset, expected);
    }

    #[test]
    fn lsb_as_bitboard_works() {
        let bb = Bitboard {
            data: 0b0111100111000,
        };
        let expected = Bitboard { data: 0b01000 };
        assert_eq!(bb.lsb_as_bitboard(), expected);
    }

    #[test]
    fn reset_lsb_works() {
        let mut bb = Bitboard {
            data: 0b0111100111000,
        };
        bb.reset_lsb();
        let expected = Bitboard {
            data: 0b0111100110000,
        };
        assert_eq!(bb, expected);
    }

    #[test]
    fn correctly_interprets_startpos_fen() {
        let actual = Board::from_fen(START_FEN);

        let white = Bitboard {
            data: 0x000000000000ffff,
        };
        let black = Bitboard {
            data: 0xffff000000000000,
        };

        let knights = Bitboard {
            data: 0x4200000000000042,
        };
        let bishops = Bitboard {
            data: 0x2400000000000024,
        };
        let rooks = Bitboard {
            data: 0x8100000000000081,
        };
        let queens = Square::D1.as_bitboard() | Square::D8.as_bitboard();
        let pawns = Bitboard {
            data: 0x00ff00000000ff00,
        };
        let kings = Square::E1.as_bitboard() | Square::E8.as_bitboard();

        let expected = Board {
            all: [white, black],
            pieces: [knights, bishops, rooks, queens, pawns, kings],
            color_to_move: Color::White,
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
}
