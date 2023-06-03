pub type Milliseconds = i64;
pub type Nodes = u64;

pub struct GoArgs {
    w_time: Milliseconds,
    b_time: Milliseconds,
    w_inc: Milliseconds,
    b_inc: Milliseconds,
}

impl GoArgs {
    pub const fn new(
        w_time: Milliseconds,
        b_time: Milliseconds,
        w_inc: Milliseconds,
        b_inc: Milliseconds,
    ) -> Self {
        Self {
            w_time,
            b_time,
            w_inc,
            b_inc,
        }
    }
}

pub struct Searcher {
    pub overhead: Milliseconds,

    node_count: Nodes,
}
