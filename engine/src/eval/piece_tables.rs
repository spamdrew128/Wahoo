use crate::{
    board::board_representation::{Color, Square, NUM_COLORS, NUM_RANKS, NUM_SQUARES},
    eval::evaluation::ScoreTuple,
};

pub struct Pst {
    table: [[ScoreTuple; NUM_SQUARES as usize]; NUM_COLORS as usize],
}

impl Pst {
    #[allow(clippy::large_types_passed_by_value)]
    pub const fn new(before: [ScoreTuple; NUM_SQUARES as usize]) -> Self {
        let mut table = [[ScoreTuple::new(0, 0); NUM_SQUARES as usize]; NUM_COLORS as usize];
        let mut i = 0;
        while i < NUM_SQUARES {
            let b_sq = Square::new(i);
            let w_sq = b_sq.mirror();
            let score = before[i as usize];
            table[Color::White.as_index()][w_sq.as_index()] = score;
            table[Color::Black.as_index()][b_sq.as_index()] = score;
            i += 1;
        }

        Self { table }
    }

    pub const fn access(&self, color: Color, sq: Square) -> ScoreTuple {
        self.table[color.as_index()][sq.as_index()]
    }
}

pub struct Prt {
    table: [[ScoreTuple; NUM_RANKS as usize]; NUM_COLORS as usize],
}

impl Prt {
    pub const fn new(before: [ScoreTuple; NUM_RANKS as usize]) -> Self {
        let mut table = [[ScoreTuple::new(0, 0); NUM_RANKS as usize]; NUM_COLORS as usize];
        let mut i = 0;
        while i < NUM_RANKS {
            let b_rank = i;
            let w_rank = 7 - i;
            let score = before[i as usize];
            table[Color::White.as_index()][w_rank as usize] = score;
            table[Color::Black.as_index()][b_rank as usize] = score;
            i += 1;
        }

        Self { table }
    }

    pub const fn access(&self, color: Color, sq: Square) -> ScoreTuple {
        self.table[color.as_index()][sq.rank() as usize]
    }
}
