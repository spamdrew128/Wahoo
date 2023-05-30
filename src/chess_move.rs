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

    pub const fn nullmove() -> Self {
        Self { data: 0 }
    }

    pub const fn new_default(to: Square, from: Square) -> Self {
        Self {
            data: to.as_u16() | (from.as_u16() << Self::FROM_OFFSET),
        }
    }

    pub const fn new_promo(to: Square, from: Square, promo_piece: Piece) -> Self {
        Self {
            data: to.as_u16()
                | (from.as_u16() << Self::FROM_OFFSET)
                | ((promo_piece.as_u16()) << Self::PROMO_OFFSET)
                | Self::PROMO_FLAG,
        }
    }

    pub const fn new_ks_castle(king_sq: Square) -> Self {
        Self {
            data: king_sq.right(2).as_u16()
                | (king_sq.as_u16() << Self::FROM_OFFSET)
                | Self::CASTLE_FLAG,
        }
    }

    pub const fn new_qs_castle(king_sq: Square) -> Self {
        Self {
            data: king_sq.left(2).as_u16()
                | (king_sq.as_u16() << Self::FROM_OFFSET)
                | Self::CASTLE_FLAG,
        }
    }

    pub const fn new_ep(to: Square, from: Square) -> Self {
        Self {
            data: to.as_u16() | (from.as_u16() << Self::FROM_OFFSET) | Self::EP_FLAG,
        }
    }

    pub const fn to(self) -> Square {
        Square::new((self.data & Self::TO_BITFIELD) as u8)
    }

    pub const fn from(self) -> Square {
        Square::new(((self.data & Self::FROM_BITFIELD) >> Self::FROM_OFFSET) as u8)
    }

    pub const fn promo_piece(self) -> Piece {
        Piece::new(((self.data & Self::PROMO_BITFIELD) >> Self::PROMO_OFFSET) as u8)
    }

    pub const fn is_castle(self) -> bool {
        (self.data & Self::FLAGS_BITFIELD) == Self::CASTLE_FLAG
    }

    pub const fn is_promo(self) -> bool {
        (self.data & Self::FLAGS_BITFIELD) == Self::PROMO_FLAG
    }

    pub const fn is_ep(self) -> bool {
        (self.data & Self::FLAGS_BITFIELD) == Self::EP_FLAG
    }
}

#[cfg(test)]
mod tests {
    use super::{Move, Piece, Square};

    #[test]
    fn test_simple_move() {
        let m = Move::new_default(Square::B1, Square::H8);
        assert_eq!(m.to(), Square::B1);
        assert_eq!(m.from(), Square::H8);
        assert!(!m.is_ep());
        assert!(!m.is_castle());
        assert!(!m.is_promo());
    }

    #[test]
    fn test_promotion() {
        let m = Move::new_promo(Square::A7, Square::A8, Piece::QUEEN);
        assert_eq!(m.to(), Square::A7);
        assert_eq!(m.from(), Square::A8);
        assert!(m.is_promo());
        assert!(!m.is_ep());
        assert!(!m.is_castle());
        assert_eq!(m.promo_piece(), Piece::QUEEN);
    }
}
