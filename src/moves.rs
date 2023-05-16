use crate::board_representation::{Piece, Square};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move {
    data: u16,
}

impl Move {
    const TO_BITFIELD: u16 = 0b0000000000111111;
    const FROM_BITFIELD: u16 = 0b0000111111000000;
    const PROMO_BITFIELD: u16 = 0b0011000000000000;
    const FLAGS_BITFIELD: u16 = 0b1100000000000000;

    const CASTLE_FLAG: u16 = 0b0100000000000000;
    const PROMO_FLAG: u16 = 0b1000000000000000;
    const EP_FLAG: u16 = 0b1100000000000000;

    const FROM_OFFSET: u8 = 6;
    const PROMO_OFFSET: u8 = 12;

    const fn new_default(to: Square, from: Square) -> Self {
        Self {
            data: (to.0 as u16) | ((from.0 as u16) << Self::FROM_OFFSET),
        }
    }

    const fn new_promo(to: Square, from: Square, promo_piece: Piece) -> Self {
        Self {
            data: (to.0 as u16)
                | ((from.0 as u16) << Self::FROM_OFFSET)
                | ((promo_piece.0 as u16) << Self::PROMO_OFFSET)
                | Self::PROMO_FLAG,
        }
    }

    const fn new_castle(to: Square, from: Square) -> Self {
        Self {
            data: (to.0 as u16) | ((from.0 as u16) << Self::FROM_OFFSET) | Self::CASTLE_FLAG,
        }
    }

    const fn new_ep(to: Square, from: Square) -> Self {
        Self {
            data: (to.0 as u16) | ((from.0 as u16) << Self::FROM_OFFSET) | Self::EP_FLAG,
        }
    }

    const fn to(self) -> Square {
        Square((self.data & Self::TO_BITFIELD) as u8)
    }

    const fn from(self) -> Square {
        Square(((self.data & Self::FROM_BITFIELD) >> Self::FROM_OFFSET) as u8)
    }

    const fn promo_piece(self) -> Piece {
        Piece(((self.data & Self::PROMO_BITFIELD) >> Self::PROMO_OFFSET) as u8)
    }

    const fn is_castle(self) -> bool {
        (self.data & Self::FLAGS_BITFIELD) == Self::CASTLE_FLAG
    }

    const fn is_promotion(self) -> bool {
        (self.data & Self::FLAGS_BITFIELD) == Self::PROMO_FLAG
    }

    const fn is_en_passant(self) -> bool {
        (self.data & Self::FLAGS_BITFIELD) == Self::EP_FLAG
    }
}
