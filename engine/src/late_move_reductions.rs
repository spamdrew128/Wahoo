use crate::{
    chess_move::MAX_MOVECOUNT,
    search::{Depth, MAX_PLY},
};

type ReductionTable = [[Depth; MAX_MOVECOUNT]; MAX_PLY as usize];

const LMR_TABLE: ReductionTable = include!(concat!(env!("OUT_DIR"), "/lmr_init.rs"));

// fn GetReduction(depth: Depth, move_count: i32) -> Depth {
//     return reductionTable[depth][move_count];
// }
