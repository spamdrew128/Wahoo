use std::sync::atomic::{AtomicU64, Ordering};

pub type Nodes = u64;
pub type TbHits = u64;

#[macro_export]
macro_rules! create_thread_data {
    ($name:ident, $thread_count:expr) => {
        let v = $crate::thread_data::ThreadData::elem_vec($thread_count);
        let $name = $crate::thread_data::ThreadData::new_primary(&v);
    };

    ($name:ident) => {
        create_thread_data!($name, 1);
    };
}

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

    pub const fn new_primary(data: &'a Vec<ThreadDataElem>) -> Self {
        Self { data, id: 0 }
    }

    pub const fn get_secondary(&self, secondary_id: usize) -> Self {
        assert!(secondary_id > 0);
        Self {
            data: self.data,
            id: secondary_id,
        }
    }

    pub fn combined(&self) -> (Nodes, TbHits) {
        let mut nodes = 0;
        let mut tb_hits = 0;
        self.data.iter().for_each(|x| {
            nodes += x.nodes.load(Ordering::SeqCst);
            tb_hits += x.tb_hits.load(Ordering::SeqCst);
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
