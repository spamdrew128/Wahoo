use crate::eval::evaluation::{EvalScore, INF};

use super::search::{MAX_PLY, Ply};

#[derive(Debug)]
pub struct EvalStack {
    table: [EvalScore; (MAX_PLY + 2) as usize]
}

impl EvalStack {
    pub const fn new() -> Self {
        Self {
            table: [INF; (MAX_PLY + 2) as usize]
        }
    }

    pub fn improving(&mut self, eval: EvalScore, ply: Ply) -> bool {
        self.table[(ply + 2) as usize] = eval;

        let prev_eval = self.table[ply as usize];
        eval > prev_eval
    }
}