use std::ops::{BitAnd, BitOr, BitXor, Not};

type Rank = u8;
type File = u8;
type Fen = u8;

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
struct Square(u8);

impl Square {
    pub const A1: Self = Self(0);
    pub const B1: Self = Self(1);
    pub const C1: Self = Self(2);
    pub const D1: Self = Self(3);
    pub const E1: Self = Self(4);
    pub const F1: Self = Self(5);
    pub const G1: Self = Self(6);
    pub const H1: Self = Self(7);
    pub const A2: Self = Self(8);
    pub const B2: Self = Self(9); 
    pub const C2: Self = Self(10); 
    pub const D2: Self = Self(11);
    pub const E2: Self = Self(12); 
    pub const F2: Self = Self(13); 
    pub const G2: Self = Self(14); 
    pub const H2: Self = Self(15);
    pub const A3: Self = Self(16);
    pub const B3: Self = Self(17); 
    pub const C3: Self = Self(18); 
    pub const D3: Self = Self(19); 
    pub const E3: Self = Self(20);
    pub const F3: Self = Self(21);
    pub const G3: Self = Self(22);
    pub const H3: Self = Self(23);
    pub const A4: Self = Self(24);
    pub const B4: Self = Self(25);
    pub const C4: Self = Self(26);
    pub const D4: Self = Self(27);
    pub const E4: Self = Self(28);
    pub const F4: Self = Self(29);
    pub const G4: Self = Self(30);
    pub const H4: Self = Self(31);
    pub const A5: Self = Self(32);
    pub const B5: Self = Self(33);
    pub const C5: Self = Self(34);
    pub const D5: Self = Self(35);
    pub const E5: Self = Self(36);
    pub const F5: Self = Self(37);
    pub const G5: Self = Self(38);
    pub const H5: Self = Self(39);
    pub const A6: Self = Self(40);
    pub const B6: Self = Self(41);
    pub const C6: Self = Self(42);
    pub const D6: Self = Self(43);
    pub const E6: Self = Self(44);
    pub const F6: Self = Self(45);
    pub const G6: Self = Self(46);
    pub const H6: Self = Self(47);
    pub const A7: Self = Self(48);
    pub const B7: Self = Self(49);
    pub const C7: Self = Self(50);
    pub const D7: Self = Self(51);
    pub const E7: Self = Self(52);
    pub const F7: Self = Self(53);
    pub const G7: Self = Self(54);
    pub const H7: Self = Self(55);
    pub const A8: Self = Self(56);
    pub const B8: Self = Self(57);
    pub const C8: Self = Self(58);
    pub const D8: Self = Self(59);
    pub const E8: Self = Self(60);
    pub const F8: Self = Self(61);
    pub const G8: Self = Self(62);
    pub const H8: Self = Self(63);

    const fn as_bitboard(self) -> Bitboard {
        Bitboard { data: 1 << self.0 }
    }

    const fn as_index(self) -> usize {
        self.0 as usize
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

// impl Board {
//     fn from_fen(fen: &FEN) -> Self {
//         let mut board = Board::default();
//     }
// }

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
