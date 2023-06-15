#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::new_without_default)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_panics_doc)]
#![allow(dead_code)] // remove later

mod attacks;
pub mod bench;
pub mod board_representation;
pub mod chess_move;
mod eval_constants;
pub mod evaluation;
mod magic;
pub mod movegen;
mod perft;
mod pv_table;
pub mod search;
pub mod time_management;
pub mod uci;
mod util_macros;
pub mod zobrist;
pub mod zobrist_stack;
