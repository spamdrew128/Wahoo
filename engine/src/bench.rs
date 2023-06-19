use crate::{
    board_representation::Board,
    history_table::History,
    perft::{test_postions, PerftTest},
    search::{SearchLimit, Searcher},
    zobrist_stack::ZobristStack,
};

pub fn bench() {
    let positions: Vec<PerftTest> = test_postions();

    let stopwatch = std::time::Instant::now();
    let mut nodes = 0;

    for pos in positions {
        let board = Board::from_fen(pos.fen);
        let mut searcher = Searcher::new(
            SearchLimit::None,
            &ZobristStack::new(&board),
            &History::new(),
        );
        nodes += searcher.bench(&board, 9);
    }

    let nps = (u128::from(nodes) * 1_000_000) / stopwatch.elapsed().as_micros();
    println!("{nodes} nodes {nps} nps");
}
