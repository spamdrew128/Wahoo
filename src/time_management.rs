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

#[derive(Debug)]
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

    pub const fn search_time(&self, args: TimeArgs, color: Color) -> Milliseconds {
        if args.infinite_mode {
            return Self::MAX_TIME;
        }

        if args.move_time > 0 {
            return args.move_time;
        }

        match color {
            Color::White => (args.w_time / 25 + args.w_inc / 2) - self.overhead,
            Color::Black => (args.b_time / 25 + args.b_inc / 2) - self.overhead,
        }
    }
}
