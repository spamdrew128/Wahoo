use crate::{
    board_representation::Board,
    history_table::History,
    perft::{test_postions, PerftTest},
    search::Searcher,
    tablebase::probe::Syzygy,
    thread_data::ThreadData,
    transposition_table::TranspositionTable,
    zobrist_stack::ZobristStack,
};

pub fn bench() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let positions: Vec<PerftTest> = test_postions();

    let stopwatch = std::time::Instant::now();
    let mut nodes = 0;

    let tt = TranspositionTable::new(16);
    for pos in positions {
        let board = Board::from_fen(pos.fen);

        let v = ThreadData::elem_vec(1);
        let thread_data = ThreadData::new(&v, 0);

        let mut searcher = Searcher::new(
            vec![],
            &ZobristStack::new(&board),
            &History::new(),
            &tt,
            Syzygy::new(),
            thread_data,
        );
        nodes += searcher.bench(&board, 14);
    }

    let nps = (u128::from(nodes) * 1_000_000) / stopwatch.elapsed().as_micros();
    println!("{nodes} nodes {nps} nps");
}
