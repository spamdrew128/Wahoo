use crate::{
    chess_move::MAX_MOVECOUNT,
    search::{Depth, MAX_PLY},
};

const LMR_TABLE: [[Depth; MAX_MOVECOUNT]; MAX_PLY as usize] =
    include!(concat!(env!("OUT_DIR"), "/lmr_init.rs"));

#[allow(clippy::cast_sign_loss)]
pub const fn get_reduction(depth: Depth, move_count: i32) -> Depth {
    LMR_TABLE[depth as usize][move_count as usize]
}
