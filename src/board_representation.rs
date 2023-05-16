use std::ops::{BitAnd, BitOr, BitXor, Not, BitOrAssign};

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

#[derive(Copy, Clone, PartialEq, Eq)]
struct Piece(u8);

#[derive(Copy, Clone)]
struct Square(u8);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Bitboard {
    data: u64
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Board {
    pub all: [Bitboard; NUM_COLORS as usize],
    pub pieces: [Bitboard; NUM_PIECES as usize],
    pub color_to_move: Color,
}

impl Piece {
    pub const KNIGHT: Self = Self(0);
    pub const BISHOP: Self = Self(1);
    pub const ROOK: Self = Self(2);
    pub const QUEEN: Self = Self(3);
    pub const PAWN: Self = Self(4);
    pub const KING: Self = Self(5);
    pub const NONE_PIECE: Self = Self(6);

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
}

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
}

impl Bitboard {
    const fn is_not_empty(self) -> bool {
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

    fn overlaps(self, rhs: Self) -> bool {
        (self & rhs).is_not_empty()
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

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.data |= rhs.data;
    }
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
                    let color = if bitset.overlaps(self.all[Color::White as usize]) { Color::White } else { Color::Black };
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
                    _ => panic!("Invalid FEN")
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

        assert!(color_char == 'w' || color_char == 'b', "Invalid color specifier");
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
                    let color = if bitset.overlaps(self.all[Color::White as usize]) { Color::White } else { Color::Black };
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

        let color_char = if self.color_to_move == Color::White { 'w' } else { 'b' };

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
    use super::{Bitboard, Square, Board, START_FEN, Color};

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
        let expected = Bitboard { data: 0b00010000000000000000 };
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

    #[test]
    fn correctly_interprets_startpos_fen() {
        let actual = Board::from_fen(START_FEN);

        let white = Bitboard { data: 0x000000000000ffff };
        let black = Bitboard { data: 0xffff000000000000 };

        let knights = Bitboard { data: 0x4200000000000042 };
        let bishops = Bitboard { data: 0x2400000000000024 };
        let rooks = Bitboard { data: 0x8100000000000081 };
        let queens = Square::D1.as_bitboard() | Square::D8.as_bitboard();
        let pawns = Bitboard { data: 0x00ff00000000ff00 };
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
