#![allow(
    dead_code,
    unused_imports,
    clippy::unused_self,
    unused_variables,
    clippy::missing_const_for_fn
)]

use std::{ffi::CString, ptr};

use crate::{
    board::board_representation::{Board, Color, Piece, Square},
    board::chess_move::{Flag, Move},
    board::movegen::MoveGenerator,
    eval::evaluation::{EvalScore, TB_LOSS_SCORE, TB_WIN_SCORE},
    search::search::Ply,
};

use super::bindings::{
    tb_init, tb_probe_root_impl, tb_probe_wdl_impl, TB_BLESSED_LOSS, TB_CURSED_WIN, TB_DRAW,
    TB_LARGEST, TB_LOSS, TB_PROMOTES_BISHOP, TB_PROMOTES_KNIGHT, TB_PROMOTES_QUEEN,
    TB_PROMOTES_ROOK, TB_RESULT_FROM_MASK, TB_RESULT_FROM_SHIFT, TB_RESULT_PROMOTES_MASK,
    TB_RESULT_PROMOTES_SHIFT, TB_RESULT_TO_MASK, TB_RESULT_TO_SHIFT, TB_RESULT_WDL_MASK,
    TB_RESULT_WDL_SHIFT, TB_WIN,
};

#[cfg(feature = "syzygy")]
pub const FATHOM_IS_COMPILED: bool = true;
#[cfg(not(feature = "syzygy"))]
pub const FATHOM_IS_COMPILED: bool = false;

#[derive(Debug, Copy, Clone)]
struct SyzygyResult(u32);

impl SyzygyResult {
    const fn score(self) -> EvalScore {
        let wdl = (self.0 & TB_RESULT_WDL_MASK) >> TB_RESULT_WDL_SHIFT;
        match wdl {
            TB_WIN => TB_WIN_SCORE,
            TB_LOSS => TB_LOSS_SCORE,
            _ => 0,
        }
    }

    fn matches_move(self, mv: Move) -> bool {
        let from = Square::new(((self.0 & TB_RESULT_FROM_MASK) >> TB_RESULT_FROM_SHIFT) as u8);
        let to = Square::new(((self.0 & TB_RESULT_TO_MASK) >> TB_RESULT_TO_SHIFT) as u8);

        if mv.to() != to || mv.from() != from {
            return false;
        }

        if mv.is_promo() {
            let promo = (self.0 & TB_RESULT_PROMOTES_MASK) >> TB_RESULT_PROMOTES_SHIFT;
            match mv.flag() {
                Flag::KNIGHT_PROMO | Flag::KNIGHT_CAPTURE_PROMO => promo == TB_PROMOTES_KNIGHT,
                Flag::BISHOP_PROMO | Flag::BISHOP_CAPTURE_PROMO => promo == TB_PROMOTES_BISHOP,
                Flag::ROOK_PROMO | Flag::ROOK_CAPTURE_PROMO => promo == TB_PROMOTES_ROOK,
                Flag::QUEEN_PROMO | Flag::QUEEN_CAPTURE_PROMO => promo == TB_PROMOTES_QUEEN,
                _ => false,
            }
        } else {
            true
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Syzygy {
    active: bool,
    n_men: u8,
}

impl Syzygy {
    pub const fn new() -> Self {
        Self {
            active: false,
            n_men: 0,
        }
    }

    #[cfg(feature = "syzygy")]
    const fn can_probe(self, board: &Board) -> bool {
        self.active && self.n_men >= (board.occupied().popcount() as u8)
    }

    #[cfg(feature = "syzygy")]
    pub fn activate(&mut self, path: &str) {
        unsafe {
            let syzygy_path = CString::new(path).unwrap();
            assert!(
                tb_init(syzygy_path.as_ptr()),
                "TB failed to initalize with path {path}"
            );
        }
        self.active = true;
        self.n_men = unsafe { TB_LARGEST as u8 };
    }

    #[cfg(feature = "syzygy")]
    pub fn probe_score(self, board: &Board, ply: Ply) -> Option<EvalScore> {
        if !self.can_probe(board) || board.halfmoves != 0 || board.castle_rights.not_empty() {
            return None;
        }

        let ep_sq = if let Some(sq) = board.ep_sq {
            u32::from(sq.as_u16())
        } else {
            0
        };

        unsafe {
            let wdl = tb_probe_wdl_impl(
                board.all[Color::White.as_index()].as_u64(),
                board.all[Color::Black.as_index()].as_u64(),
                board.pieces[Piece::KING.as_index()].as_u64(),
                board.pieces[Piece::QUEEN.as_index()].as_u64(),
                board.pieces[Piece::ROOK.as_index()].as_u64(),
                board.pieces[Piece::BISHOP.as_index()].as_u64(),
                board.pieces[Piece::KNIGHT.as_index()].as_u64(),
                board.pieces[Piece::PAWN.as_index()].as_u64(),
                ep_sq,
                board.color_to_move == Color::White,
            );

            match wdl {
                TB_WIN => Some(TB_WIN_SCORE - i32::from(ply)),
                TB_LOSS => Some(TB_LOSS_SCORE + i32::from(ply)),
                TB_DRAW | TB_CURSED_WIN | TB_BLESSED_LOSS => Some(0),
                _ => None,
            }
        }
    }

    #[cfg(feature = "syzygy")]
    pub fn probe_root(self, board: &Board) -> Option<(Move, EvalScore)> {
        if !self.can_probe(board) || board.castle_rights.not_empty() {
            return None;
        }

        let ep_sq = if let Some(sq) = board.ep_sq {
            u32::from(sq.as_u16())
        } else {
            0
        };

        unsafe {
            let result = SyzygyResult(tb_probe_root_impl(
                board.all[Color::White.as_index()].as_u64(),
                board.all[Color::Black.as_index()].as_u64(),
                board.pieces[Piece::KING.as_index()].as_u64(),
                board.pieces[Piece::QUEEN.as_index()].as_u64(),
                board.pieces[Piece::ROOK.as_index()].as_u64(),
                board.pieces[Piece::BISHOP.as_index()].as_u64(),
                board.pieces[Piece::KNIGHT.as_index()].as_u64(),
                board.pieces[Piece::PAWN.as_index()].as_u64(),
                u32::from(board.halfmoves),
                ep_sq,
                board.color_to_move == Color::White,
                ptr::null_mut(),
            ));

            let score = result.score();

            let mut generator = MoveGenerator::new();
            while let Some(mv) = generator.simple_next::<true>(board) {
                if result.matches_move(mv) {
                    return Some((mv, score));
                }
            }
        }

        None
    }

    #[cfg(not(feature = "syzygy"))]
    const fn can_probe(self, board: &Board) -> bool {
        false
    }

    #[cfg(not(feature = "syzygy"))]
    pub fn activate(&mut self, path: &str) {}

    #[cfg(not(feature = "syzygy"))]
    pub fn probe_score(self, board: &Board, ply: Ply) -> Option<EvalScore> {
        None
    }

    #[cfg(not(feature = "syzygy"))]
    pub fn probe_root(self, board: &Board) -> Option<(Move, EvalScore)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{board::board_representation::Board, eval::evaluation::TB_WIN_SCORE};

    use super::Syzygy;

    #[test]
    #[ignore = "wrong path"]
    fn can_probe() {
        let path = "C:\\Users\\Andrew\\OneDrive\\Desktop\\Bitmapped Chess\\3-4-5";
        let mut tb = Syzygy::new();
        tb.activate(path);

        let board = Board::from_fen("8/3kr3/8/8/8/8/2K5/2Q5 w - - 0 1");
        if let Some((mv, score)) = tb.probe_root(&board) {
            println!("{}", mv.as_string());
            assert_eq!(score, TB_WIN_SCORE);
            assert_eq!(score, tb.probe_score(&board, 0).unwrap());
        } else {
            panic!("Probe failed")
        }
    }

    #[test]
    #[ignore = "wrong path"]
    fn halfmove_handling() {
        let path = "C:\\Users\\Andrew\\OneDrive\\Desktop\\Bitmapped Chess\\3-4-5";
        let mut tb = Syzygy::new();
        tb.activate(path);
        let mut board = Board::from_fen("8/3kr3/8/8/8/8/2K5/2Q5 w - - 0 1");
        board.halfmoves = 70;

        if let Some((mv, score)) = tb.probe_root(&board) {
            println!("{}", mv.as_string());
            assert_eq!(score, 0);
            assert!(tb.probe_score(&board, 0).is_none());
        } else {
            panic!("Probe failed")
        }
    }
}
