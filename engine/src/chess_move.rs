use crate::{
    attacks,
    board_representation::{Board, Piece, Square},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Flag(u16);

impl Flag {
    pub const NONE: Self = Self(0 << Move::FLAGS_OFFSET);
    pub const KS_CASTLE: Self = Self(1 << Move::FLAGS_OFFSET);
    pub const QS_CASTLE: Self = Self(2 << Move::FLAGS_OFFSET);
    pub const EP: Self = Self(3 << Move::FLAGS_OFFSET);
    pub const DOUBLE_PUSH: Self = Self(4 << Move::FLAGS_OFFSET);
    pub const KNIGHT_PROMO: Self = Self(5 << Move::FLAGS_OFFSET);
    pub const BISHOP_PROMO: Self = Self(6 << Move::FLAGS_OFFSET);
    pub const ROOK_PROMO: Self = Self(7 << Move::FLAGS_OFFSET);
    pub const QUEEN_PROMO: Self = Self(8 << Move::FLAGS_OFFSET);
    pub const KNIGHT_CAPTURE_PROMO: Self = Self(9 << Move::FLAGS_OFFSET);
    pub const BISHOP_CAPTURE_PROMO: Self = Self(10 << Move::FLAGS_OFFSET);
    pub const ROOK_CAPTURE_PROMO: Self = Self(11 << Move::FLAGS_OFFSET);
    pub const QUEEN_CAPTURE_PROMO: Self = Self(12 << Move::FLAGS_OFFSET);
    pub const CAPTURE: Self = Self(13 << Move::FLAGS_OFFSET);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move {
    data: u16,
}

impl Move {
    const TO_BITFIELD: u16 = 0b0000000000111111;
    const FROM_BITFIELD: u16 = 0b0000111111000000;
    const FLAGS_BITFIELD: u16 = 0b1111000000000000;

    const FROM_OFFSET: u8 = 6;
    const FLAGS_OFFSET: u8 = 12;

    pub const fn nullmove() -> Self {
        Self { data: 0 }
    }

    pub const fn new(to: Square, from: Square, flag: Flag) -> Self {
        Self {
            data: to.as_u16() | (from.as_u16() << Self::FROM_OFFSET) | flag.0,
        }
    }

    pub const fn new_ks_castle(king_sq: Square) -> Self {
        Self {
            data: king_sq.right(2).as_u16()
                | (king_sq.as_u16() << Self::FROM_OFFSET)
                | Flag::KS_CASTLE.0,
        }
    }

    pub const fn new_qs_castle(king_sq: Square) -> Self {
        Self {
            data: king_sq.left(2).as_u16()
                | (king_sq.as_u16() << Self::FROM_OFFSET)
                | Flag::QS_CASTLE.0,
        }
    }

    pub const fn to(self) -> Square {
        Square::new((self.data & Self::TO_BITFIELD) as u8)
    }

    pub const fn from(self) -> Square {
        Square::new(((self.data & Self::FROM_BITFIELD) >> Self::FROM_OFFSET) as u8)
    }

    pub const fn flag(self) -> Flag {
        Flag(self.data & Self::FLAGS_BITFIELD)
    }

    pub fn is_promo(self) -> bool {
        (self.flag() >= Flag::KNIGHT_PROMO) && (self.flag() <= Flag::QUEEN_CAPTURE_PROMO)
    }

    pub fn is_capture(self) -> bool {
        self.flag() >= Flag::KNIGHT_CAPTURE_PROMO
    }

    pub fn as_string(self) -> String {
        let mut move_str = String::new();
        move_str.push_str(self.from().as_string().as_str());
        move_str.push_str(self.to().as_string().as_str());

        match self.flag() {
            Flag::KNIGHT_PROMO | Flag::KNIGHT_CAPTURE_PROMO => move_str.push('n'),
            Flag::BISHOP_PROMO | Flag::BISHOP_CAPTURE_PROMO => move_str.push('b'),
            Flag::ROOK_PROMO | Flag::ROOK_CAPTURE_PROMO => move_str.push('r'),
            Flag::QUEEN_PROMO | Flag::QUEEN_CAPTURE_PROMO => move_str.push('q'),
            _ => (),
        }

        move_str
    }

    pub fn from_string(mv_str: &str, board: &Board) -> Self {
        let mut chars = mv_str.chars();
        let from_str = format!("{}{}", chars.next().unwrap(), chars.next().unwrap());
        let to_str = format!("{}{}", chars.next().unwrap(), chars.next().unwrap());
        let promo = chars.next();

        let from = Square::from_string(from_str.as_str()).unwrap();
        let to = Square::from_string(to_str.as_str()).unwrap();
        let piece = board.piece_on_sq(from);
        let captured_piece = board.piece_on_sq(to);

        let promo_flags = [
            Flag::KNIGHT_PROMO,
            Flag::BISHOP_PROMO,
            Flag::ROOK_PROMO,
            Flag::QUEEN_PROMO,
        ];
        let cap_promo_flags = [
            Flag::KNIGHT_CAPTURE_PROMO,
            Flag::BISHOP_CAPTURE_PROMO,
            Flag::ROOK_CAPTURE_PROMO,
            Flag::QUEEN_CAPTURE_PROMO,
        ];

        if piece == Piece::KING && (!attacks::king(from).overlaps(to.as_bitboard())) {
            if to.file() >= from.file() {
                return Self::new_ks_castle(from);
            }
            if to.file() <= from.file() {
                return Self::new_qs_castle(from);
            }
        }

        if board.promotable_pawns().overlaps(from.as_bitboard()) {
            let promo_type = Piece::from_char(promo.unwrap()).unwrap();
            let flag = if captured_piece == Piece::NONE {
                promo_flags[promo_type.as_index()]
            } else {
                cap_promo_flags[promo_type.as_index()]
            };
            return Self::new(to, from, flag);
        }

        if piece == Piece::PAWN {
            if let Some(ep_sq) = board.ep_sq {
                if ep_sq == to {
                    return Self::new(to, from, Flag::EP);
                }
            }

            if from == to.retreat(2, board.color_to_move) {
                return Self::new(to, from, Flag::DOUBLE_PUSH);
            }
        }

        if captured_piece == Piece::NONE {
            Self::new(to, from, Flag::NONE)
        } else {
            Self::new(to, from, Flag::CAPTURE)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Flag, Move, Square};

    #[test]
    fn test_move() {
        let m = Move::new(Square::B1, Square::H8, Flag::NONE);
        assert_eq!(m.to(), Square::B1);
        assert_eq!(m.from(), Square::H8);
        assert!(m.flag() == Flag::NONE);
    }
}