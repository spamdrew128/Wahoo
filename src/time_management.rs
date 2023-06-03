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

    pub const fn new() -> Self {
        Self {
            overhead: Self::OVERHEAD_DEFAULT,
        }
    }
}
