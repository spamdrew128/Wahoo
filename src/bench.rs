use crate::{perft::{PerftTest, test_postions}, board_representation::Board, search::Searcher};

pub fn bench() {
    let positions: Vec<PerftTest> = test_postions();
    let mut searcher = Searcher::new();

    let stopwatch = std::time::Instant::now();
    let mut nodes = 0;

    for pos in positions {
        let board = Board::from_fen(pos.fen);
        nodes += searcher.bench(&board, 6);
    }

    let nps = (u128::from(nodes) * 1_000_000) / stopwatch.elapsed().as_micros();
    println!("{nodes} nodes {nps} nps");
}