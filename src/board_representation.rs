use std::ops::{BitAnd, BitOr, BitXor, Not};

type Rank = u8;
type File = u8;

pub const NUM_SQUARES: u8 = 64;
pub const NUM_PIECES: u8 = 6;
pub const NUM_COLORS: u8 = 2;

#[derive(Copy, Clone)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone)]
enum Piece {
    Knight,
    Bishop,
    Rook,
    Queen,
    Pawn,
    King,
    None,
}

#[derive(Copy, Clone)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    const fn as_bitboard(self) -> Bitboard {
        Bitboard { data: 1 << self as u8 }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Bitboard {
    data: u64
}

impl Bitboard {
    const fn not_empty(self) -> bool {
        self.data > 0
    }

    const fn popcount(self) -> u32 {
        self.data.count_ones()
    }

    const fn lsb(self) -> u32 {
        self.data.trailing_zeros()
    }

    const fn lsb_as_bitboard(self) -> Self {
        Self { data: self.data & ((!self.data) + 1) }
    }

    fn reset_lsb(&mut self) {
        self.data = self.data & (self.data - 1);
    }

    fn bit_is_set(self, pos: u32) -> bool {
        let bitset = Self { data: 1 << pos };
        (self & bitset).not_empty()
    }
}

impl BitAnd for Bitboard {
    type Output = Self;
    
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { data: self.data & rhs.data }
    }
}

impl BitOr for Bitboard {
    type Output = Self;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        Self{ data: self.data | rhs.data }
    }
}

impl BitXor for Bitboard {
    type Output = Self;
    
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self{ data: self.data ^ rhs.data }
    }
}

impl Not for Bitboard {
    type Output = Self;
    
    fn not(self) -> Self::Output {
        Self { data: !self.data }
    }
}

#[derive(Debug, Default)]
pub struct Board {
    pub all: [Bitboard; NUM_COLORS as usize],
    pub pieces: [Bitboard; NUM_PIECES as usize],
}

#[cfg(test)]
mod tests {
    use super::{Bitboard, Square};

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
        let bb = Bitboard { data: 0xFFFF0000FFFF0000 };
        let expected = Bitboard { data: 0x0000FFFF0000FFFF };
        assert_eq!(!bb, expected);
    }

    #[test]
    fn combinations_work() {
        let data1: u64 = 894378932;
        let data2: u64 = 18981928111;

        let bb1 = Bitboard { data: data1 };
        let bb2 = Bitboard { data: data2 };
        let expected = Bitboard { data: data1 & !data2 };
        assert_eq!(bb1 & !bb2, expected);
    }

    #[test]
    fn sq_to_bb_works() {
        let bitset = Square::A3.as_bitboard();
        let expected = Bitboard { data: 0b0001_0000_0000_0000_0000 };
        assert_eq!(bitset, expected);
    }

    #[test]
    fn lsb_as_bitboard_works() {
        let bb = Bitboard { data: 0b0111100111000 };
        let expected = Bitboard { data: 0b01000 };
        assert_eq!(bb.lsb_as_bitboard(), expected);
    }

    #[test]
    fn reset_lsb_works() {
        let mut bb = Bitboard { data: 0b0111100111000 };
        bb.reset_lsb();
        let expected = Bitboard { data: 0b0111100110000 };
        assert_eq!(bb, expected);
    }
}
