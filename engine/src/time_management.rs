use std::time::Instant;

use crate::board_representation::Color;

pub type Milliseconds = u128;

#[derive(Debug, Copy, Clone, Default)]
pub struct TimeArgs {
    pub w_time: Milliseconds,
    pub b_time: Milliseconds,
    pub w_inc: Milliseconds,
    pub b_inc: Milliseconds,
    pub move_time: Milliseconds,
}

#[derive(Debug, Copy, Clone)]
pub struct TimeManager {
    pub overhead: Milliseconds,
}

impl TimeManager {
    pub const fn new(overhead: Milliseconds) -> Self {
        Self { overhead }
    }

    pub const fn calculate_search_time(self, args: TimeArgs, color: Color) -> Milliseconds {
        if args.move_time > 0 {
            return args.move_time.saturating_sub(self.overhead);
        }

        match color {
            Color::White => (args.w_time / 25 + args.w_inc / 2).saturating_sub(self.overhead),
            Color::Black => (args.b_time / 25 + args.b_inc / 2).saturating_sub(self.overhead),
        }
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
}
