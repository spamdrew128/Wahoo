use std::time::Instant;

use crate::board_representation::Color;

pub type Milliseconds = i64;

#[derive(Debug, Copy, Clone, Default)]
pub struct TimeArgs {
    pub w_time: Milliseconds,
    pub b_time: Milliseconds,
    pub w_inc: Milliseconds,
    pub b_inc: Milliseconds,
    pub move_time: Milliseconds,
    pub infinite_mode: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct TimeManager {
    pub overhead: Milliseconds,
}

impl TimeManager {
    const OVERHEAD_DEFAULT: Milliseconds = 25;
    const MAX_TIME: Milliseconds = i64::MAX;

    pub const fn new() -> Self {
        Self {
            overhead: Self::OVERHEAD_DEFAULT,
        }
    }

    pub fn construct_search_timer(self, args: TimeArgs, color: Color) -> SearchTimer {
        if args.infinite_mode {
            return SearchTimer::new(Self::MAX_TIME);
        }

        if args.move_time > 0 {
            return SearchTimer::new(args.move_time - self.overhead);
        }

        match color {
            Color::White => SearchTimer::new((args.w_time / 25 + args.w_inc / 2) - self.overhead),
            Color::Black => SearchTimer::new((args.b_time / 25 + args.b_inc / 2) - self.overhead),
        }
    }
}

#[derive(Debug)]
pub struct SearchTimer {
    timer: Instant,
    search_time: Milliseconds,
}

impl SearchTimer {
    pub fn new(time_to_use: Milliseconds) -> Self {
        Self {
            timer: Instant::now(),
            search_time: time_to_use,
        }
    }

    pub fn is_expired(&self) -> bool {
        (self.timer.elapsed().as_millis() as Milliseconds) > self.search_time // todo: make this more precise?
    }
}
