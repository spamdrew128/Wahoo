use std::ffi::CString;

use crate::{
    board_representation::{Board, Color, Piece},
    evaluation::{EvalScore, TB_LOSS_SCORE, TB_WIN_SCORE},
};

use super::bindings::{
    tb_free, tb_init, tb_probe_wdl_impl, TB_BLESSED_LOSS, TB_CURSED_WIN, TB_DRAW, TB_LARGEST,
    TB_LOSS, TB_WIN,
};

pub fn init_tablebase(path: &str) {
    unsafe {
        let syzygy_path = CString::new(path).unwrap();
        assert!(tb_init(syzygy_path.as_ptr()), "TB failed to initalize");
    }
}

pub fn free_tablebase() {
    unsafe {
        tb_free();
    }
}

pub fn tb_max_pieces_count() -> u8 {
    unsafe { TB_LARGEST as u8 }
}

pub fn probe_wdl(board: &Board) -> Option<EvalScore> {
    if (board.halfmoves != 0) || board.castle_rights.not_empty() {
        return None;
    }

    let ep_sq = if let Some(sq) = board.ep_sq {
        sq.as_u16()
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
            ep_sq as u32,
            board.color_to_move == Color::White,
        );

        match wdl {
            TB_WIN => Some(TB_WIN_SCORE),
            TB_LOSS => Some(TB_LOSS_SCORE),
            TB_DRAW | TB_CURSED_WIN | TB_BLESSED_LOSS => Some(0),
            _ => None,
        }
    }
}
