pub type Milliseconds = i64;
pub type Nodes = u64;

#[derive(Debug, Copy, Clone, Default)]
pub struct GoArgs {
    pub w_time: Milliseconds,
    pub b_time: Milliseconds,
    pub w_inc: Milliseconds,
    pub b_inc: Milliseconds,
}

pub struct Searcher {
    pub overhead: Milliseconds,

    node_count: Nodes,
}
