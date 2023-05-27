#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::module_name_repetitions)]
#![allow(dead_code)] // remove later

mod attacks;
mod board_representation;
mod chess_move;
mod magic;
mod util_macros;

fn main() {
    println!("Hello, world!");
}
