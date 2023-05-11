use std::ops::{BitAnd, BitOr, BitXor, Not};

enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
}
pub const NUM_SQUARES: u8 = 64;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bitboard {
    data: u64
}

impl Bitboard {
    const fn new() -> Self {
        Bitboard { data: 0 }
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

#[cfg(test)]
mod tests {
    use super::Bitboard;

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
}
