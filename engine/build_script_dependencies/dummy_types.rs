use std::ops::{BitOrAssign, Not};

pub const NUM_SQUARES: u8 = 64;
pub const NUM_PIECES: u8 = 6;
pub const NUM_COLORS: u8 = 2;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Square(u8);

impl Square {
    pub const fn new(data: u8) -> Self {
        Self(data)
    }

    pub const fn as_bitboard(self) -> Bitboard {
        Bitboard::new(1 << self.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Bitboard {
    data: u64,
}

impl Bitboard {
    pub const EMPTY: Self = Self::new(0);
    pub const A_FILE: Self = Self::new(0x0101010101010101);
    pub const H_FILE: Self = Self::new(0x8080808080808080);

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

    pub const fn intersection(self, rhs: Self) -> Self {
        Self {
            data: self.data & rhs.data,
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
