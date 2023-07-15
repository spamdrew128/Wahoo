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
            (time / u128::from(args.moves_to_go)).saturating_sub(self.overhead)
        } else {
            0
        };

        normal_time.max(to_go_time)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SearchTimer {
    timer: Instant,
    hard_limit: u128,
    soft_limit: u128,
}

impl SearchTimer {
    pub fn new(time_to_use: Milliseconds) -> Self {
        Self {
            timer: Instant::now(),
            hard_limit: time_to_use.saturating_mul(1000),
            soft_limit: time_to_use.saturating_mul(1000) / 2,
        }
    }

    pub fn is_expired(&self) -> bool {
        (self.timer.elapsed().as_micros()) > self.hard_limit
    }

    pub fn is_soft_expired(&self) -> bool {
        self.timer.elapsed().as_micros() > self.soft_limit
    }

    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_precision_loss)]
    pub fn update_soft_limit(&mut self, widenings: u16) {
        let w = f64::from(widenings);
        let scale: f64 = (0.006 * w).mul_add(w, 0.5);
        self.soft_limit = ((self.hard_limit as f64) * scale) as u128;
    }
}
