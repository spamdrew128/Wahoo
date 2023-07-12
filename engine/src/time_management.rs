use std::time::Instant;

use crate::board_representation::{Color, NUM_COLORS};

pub type Milliseconds = u128;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct TimeArgs {
    pub time: [Milliseconds; NUM_COLORS as usize],
    pub inc: [Milliseconds; NUM_COLORS as usize],
    pub move_time: Milliseconds,
    pub moves_to_go: u64,
}

#[derive(Debug, Copy, Clone)]
pub struct TimeManager {
    pub overhead: Milliseconds,
}

impl TimeManager {
    pub const fn new(overhead: Milliseconds) -> Self {
        Self { overhead }
    }

    pub fn calculate_search_time(self, args: TimeArgs, color: Color) -> Milliseconds {
        if args.move_time > 0 {
            return args.move_time.saturating_sub(self.overhead);
        }

        let time = args.time[color.as_index()];
        let inc = args.inc[color.as_index()];

        let normal_time = (time / 20 + inc / 2).saturating_sub(self.overhead);
        let to_go_time = if args.moves_to_go > 0 {
            time / u128::from(args.moves_to_go).saturating_sub(self.overhead)
        } else {
            0
        };

        normal_time.max(to_go_time)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SearchTimer {
    timer: Instant,
    search_time: u128,
}

impl SearchTimer {
    pub fn new(time_to_use: Milliseconds) -> Self {
        Self {
            timer: Instant::now(),
            search_time: time_to_use.saturating_mul(1000),
        }
    }

    pub fn is_expired(&self) -> bool {
        (self.timer.elapsed().as_micros()) > self.search_time
    }

    pub fn soft_cutoff_is_expired(&self) -> bool {
        (self.timer.elapsed().as_micros()) > (6 * self.search_time / 10)
    }
}
