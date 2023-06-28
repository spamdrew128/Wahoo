use crate::{
    board_representation::{Color, Square, NUM_COLORS, NUM_RANKS, NUM_SQUARES},
    evaluation::ScoreTuple,
};

pub struct Pst {
    table: [[ScoreTuple; NUM_SQUARES as usize]; NUM_COLORS as usize],
}

impl Pst {
    pub const fn new(before: [ScoreTuple; NUM_SQUARES as usize]) -> Self {
        let mut table = [[ScoreTuple::new(0, 0); NUM_SQUARES as usize]; NUM_COLORS as usize];
        let mut i = 0;
        while i < NUM_SQUARES {
            let sq = Square::new(i);
            let black_score = before[i as usize];
            table[Color::White.as_index()][sq.flip().as_index()] = black_score;
            table[Color::Black.as_index()][sq.as_index()] = black_score;
            i += 1;
        }

        Self { table }
    }

    pub const fn access(&self, color: Color, sq: Square) -> ScoreTuple {
        self.table[color.as_index()][sq.as_index()]
    }
}

pub struct Rst {
    table: [[ScoreTuple; NUM_RANKS as usize]; NUM_COLORS as usize],
}

impl Rst {
    pub const fn new(before: [ScoreTuple; NUM_RANKS as usize]) -> Self {
        let mut table = [[ScoreTuple::new(0, 0); NUM_RANKS as usize]; NUM_COLORS as usize];
        let mut i = 0;
        while i < NUM_RANKS {
            let sq = Square::new(i);
            let black_score = before[i as usize];
            table[Color::White.as_index()][sq.flip().rank() as usize] = black_score;
            table[Color::Black.as_index()][sq.rank() as usize] = black_score;
            i += 1;
        }

        Self { table }
    }

    pub const fn access(&self, color: Color, sq: Square) -> ScoreTuple {
        self.table[color.as_index()][sq.rank() as usize]
    }
}
