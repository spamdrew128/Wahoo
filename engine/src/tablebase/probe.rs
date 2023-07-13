use std::ffi::CString;

use crate::{
    board_representation::{Board, Color, Piece},
    evaluation::EvalScore,
};

use super::bindings::{tb_free, tb_init, tb_probe_wdl_impl};

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

pub fn probe_wdl(board: &Board) -> Option<EvalScore> {
    if (board.halfmoves != 0) || board.castle_rights.not_empty() {
        return None;
    }

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
            0,
            board.color_to_move == Color::White,
        );
    }

    None
}
