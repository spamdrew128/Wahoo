use crate::{
    board::chess_move::Move,
    search::search::{Ply, MAX_PLY},
};

#[derive(Debug)]
pub struct Killers {
    moves: [Move; MAX_PLY as usize],
}

impl Killers {
    pub fn new() -> Self {
        Self {
            moves: [Move::nullmove(); MAX_PLY as usize],
        }
    }

    pub fn update(&mut self, mv: Move, ply: Ply) {
        self.moves[ply as usize] = mv;
    }

    pub const fn killer(&self, ply: Ply) -> Move {
        self.moves[ply as usize]
    }
}
