use std::sync::atomic::{AtomicU64, Ordering};

pub type Nodes = u64;
pub type TbHits = u64;

#[derive(Debug, Default)]
pub struct ThreadDataElem {
    nodes: AtomicU64,
    tb_hits: AtomicU64,
}

#[derive(Debug)]
pub struct ThreadData<'a> {
    data: &'a Vec<ThreadDataElem>,
    id: usize,
}

impl<'a> ThreadData<'a> {
    pub fn elem_vec(thread_count: usize) -> Vec<ThreadDataElem> {
        let mut data = vec![];
        assert!(thread_count > 0);
        for _ in 0..thread_count {
            data.push(ThreadDataElem::default());
        }
        data
    }

    pub fn new(data: &'a Vec<ThreadDataElem>, id: usize) -> Self {
        Self { data, id }
    }

    pub fn combined(&self) -> (Nodes, TbHits) {
        let mut nodes = 0;
        let mut tb_hits = 0;
        self.data.iter().for_each(|x| {
            nodes += x.nodes.load(Ordering::AcqRel);
            tb_hits += x.tb_hits.load(Ordering::AcqRel);
        });
        (nodes, tb_hits)
    }

    pub fn thread_node_count(&self) -> Nodes {
        self.data[self.id].nodes.load(Ordering::Relaxed)
    }

    pub fn increment_nodes(&self) {
        self.data[self.id].nodes.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_tb_hits(&self) {
        self.data[self.id].tb_hits.fetch_add(1, Ordering::Relaxed);
    }
}
