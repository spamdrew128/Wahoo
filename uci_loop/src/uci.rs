use engine::{
    board::board_representation::{Board, Color, START_FEN},
    board::chess_move::Move,
    board::zobrist::ZobristHash,
    board::zobrist_stack::ZobristStack,
    create_thread_data,
    eval::evaluation::evaluate,
    search::history_table::History,
    search::search::{self, Depth, SearchLimit, Searcher},
    search::thread_data::Nodes,
    search::time_management::{Milliseconds, TimeArgs, TimeManager},
    search::transposition_table::TranspositionTable,
    tablebase::probe::{Syzygy, FATHOM_IS_COMPILED},
};

use std::thread;

enum UciCommand {
    Uci,
    IsReady,
    UciNewGame,
    Position(String, Vec<String>),
    Go(Vec<String>),
    SetOptionOverhead(Milliseconds),
    SetOptionHash(usize),
    SetOptionThreads(usize),
    SetOptionSyzygyPath(String),

    // User Commands
    StaticEval,
}

pub struct UciHandler {
    board: Board,
    zobrist_stack: ZobristStack,
    history: History,
    tt: TranspositionTable,
    time_manager: TimeManager,
    stored_message: Option<String>,
    num_threads: usize,
    tablebase: Syzygy,
}

macro_rules! send_uci_option {
    ($name:expr, $type:expr, $($format:tt)*) => {
        print!("option name {} type {} ", $name, $type);
        println!($($format)*);
    };
}

fn end_of_transmission(buffer: &str) -> bool {
    buffer
        .chars()
        .next()
        .map_or(false, |c| c == char::from(0x04))
}

fn kill_program() {
    std::process::exit(0);
}

impl UciHandler {
    const OVERHEAD_DEFAULT: Milliseconds = 25;
    const OVERHEAD_MIN: Milliseconds = 0;
    const OVERHEAD_MAX: Milliseconds = 500;

    const HASH_DEFAULT: usize = 16;
    const HASH_MIN: usize = 0;
    const HASH_MAX: usize = 8192;

    const THREADS_DEFAULT: usize = 1;
    const THREADS_MIN: usize = 1;
    const THREADS_MAX: usize = 64;

    const SYZYGY_PATH_DEFAULT: &str = "<empty>";

    pub fn new() -> Self {
        let board = Board::from_fen(START_FEN);
        let zobrist_stack = ZobristStack::new(&board);
        Self {
            board,
            zobrist_stack,
            history: History::new(),
            tt: TranspositionTable::new(Self::HASH_DEFAULT),
            time_manager: TimeManager::new(Self::OVERHEAD_DEFAULT),
            stored_message: None,
            num_threads: Self::THREADS_DEFAULT,
            tablebase: Syzygy::new(),
        }
    }

    fn read_uci_input() -> String {
        let mut buffer = String::new();
        let bytes_read = std::io::stdin()
            .read_line(&mut buffer)
            .expect("UCI Input Failure");

        if bytes_read == 0 || end_of_transmission(buffer.as_str()) {
            kill_program();
        }

        buffer
    }

    pub fn execute_instructions(&mut self) {
        let buffer = if let Some(message) = &self.stored_message {
            message.clone()
        } else {
            Self::read_uci_input()
        };
        self.stored_message = None;

        let message = buffer.split_whitespace().collect::<Vec<&str>>();

        if let Some(&cmd) = message.first() {
            match cmd {
                "uci" => self.process_command(UciCommand::Uci),
                "ucinewgame" => self.process_command(UciCommand::UciNewGame),
                "isready" => self.process_command(UciCommand::IsReady),
                "position" => {
                    let mut i = 1;
                    let fen = if message[i] == "startpos" {
                        i += 2;
                        START_FEN.to_owned()
                    } else {
                        i = 9;
                        format!(
                            "{} {} {} {} {} {}",
                            message[2], message[3], message[4], message[5], message[6], message[7]
                        )
                    };

                    let mut mv_vec: Vec<String> = vec![];
                    if i < message.len() {
                        for &mv_str in &message[i..] {
                            mv_vec.push((mv_str).to_string());
                        }
                    }

                    self.process_command(UciCommand::Position(fen, mv_vec));
                }
                "go" => {
                    let mut arg_vec: Vec<String> = vec![];
                    for arg in &message[1..] {
                        arg_vec.push((*arg).to_string());
                    }
                    self.process_command(UciCommand::Go(arg_vec));
                }
                "setoption" => {
                    let name = message[2];
                    let val = message[4];

                    match name {
                        "Overhead" => self.process_command(UciCommand::SetOptionOverhead(
                            val.parse::<Milliseconds>()
                                .unwrap_or(Self::OVERHEAD_DEFAULT),
                        )),
                        "Hash" => self.process_command(UciCommand::SetOptionHash(
                            val.parse::<usize>().unwrap_or(Self::HASH_DEFAULT),
                        )),
                        "Threads" => self.process_command(UciCommand::SetOptionThreads(
                            val.parse::<usize>().unwrap_or(Self::THREADS_DEFAULT),
                        )),
                        "SyzygyPath" => {
                            let path = &message[4..].join(" ");
                            self.process_command(UciCommand::SetOptionSyzygyPath(path.to_owned()))
                        }
                        _ => (),
                    }
                }
                "eval" => self.process_command(UciCommand::StaticEval),
                "quit" => kill_program(),
                _ => (),
            }
        }
    }

    fn process_command(&mut self, command: UciCommand) {
        match command {
            UciCommand::Uci => {
                println!("id name Wahoo v4.0.0");
                println!("id author Andrew Hockman");

                send_uci_option!(
                    "Overhead",
                    "spin",
                    "default {} min {} max {}",
                    Self::OVERHEAD_DEFAULT,
                    Self::OVERHEAD_MIN,
                    Self::OVERHEAD_MAX
                );
                send_uci_option!(
                    "Hash",
                    "spin",
                    "default {} min {} max {}",
                    Self::HASH_DEFAULT,
                    Self::HASH_MIN,
                    Self::HASH_MAX
                );
                send_uci_option!(
                    "Threads",
                    "spin",
                    "default {} min {} max {}",
                    Self::THREADS_DEFAULT,
                    Self::THREADS_MIN,
                    Self::THREADS_MAX
                );
                if FATHOM_IS_COMPILED {
                    send_uci_option!(
                        "SyzygyPath",
                        "string",
                        "default {}",
                        Self::SYZYGY_PATH_DEFAULT
                    );
                }

                println!("uciok");
            }
            UciCommand::IsReady => println!("readyok"),
            UciCommand::UciNewGame => {
                self.history = History::new();
                self.tt.reset();
            }
            UciCommand::Position(fen, move_vec) => {
                let mut new_board = Board::from_fen(fen.as_str());
                let mut new_zobrist_stack = ZobristStack::new(&new_board);

                for mv_str in move_vec {
                    let hash_base = ZobristHash::incremental_update_base(&new_board);
                    let mv = Move::from_string(mv_str.as_str(), &new_board);
                    let success = new_board.try_play_move(mv, &mut new_zobrist_stack, hash_base);
                    if !success {
                        return;
                    }
                }

                self.board = new_board;
                self.zobrist_stack = new_zobrist_stack;
            }
            UciCommand::Go(arg_vec) => {
                let mut time_args = TimeArgs::default();
                let mut search_limits = vec![];

                let mut args_iterator = arg_vec.iter();
                while let Some(arg) = args_iterator.next() {
                    match arg.as_str() {
                        "wtime" => {
                            time_args.time[Color::White.as_index()] = args_iterator
                                .next()
                                .unwrap()
                                .parse::<Milliseconds>()
                                .unwrap_or(0);
                        }
                        "btime" => {
                            time_args.time[Color::Black.as_index()] = args_iterator
                                .next()
                                .unwrap()
                                .parse::<Milliseconds>()
                                .unwrap_or(0);
                        }
                        "winc" => {
                            time_args.inc[Color::White.as_index()] = args_iterator
                                .next()
                                .unwrap()
                                .parse::<Milliseconds>()
                                .unwrap_or(0);
                        }
                        "binc" => {
                            time_args.inc[Color::Black.as_index()] = args_iterator
                                .next()
                                .unwrap()
                                .parse::<Milliseconds>()
                                .unwrap_or(0);
                        }
                        "movetime" => {
                            time_args.move_time = args_iterator
                                .next()
                                .unwrap()
                                .parse::<Milliseconds>()
                                .unwrap_or(0);
                        }
                        "movestogo" => {
                            time_args.moves_to_go =
                                args_iterator.next().unwrap().parse::<u64>().unwrap_or(0);
                        }
                        "depth" => {
                            let depth = args_iterator.next().unwrap().parse::<Depth>().unwrap_or(0);

                            if depth > 0 {
                                search_limits.push(SearchLimit::Depth(depth));
                            }
                        }
                        "nodes" => {
                            let nodes = args_iterator.next().unwrap().parse::<Nodes>().unwrap_or(0);

                            if nodes > 0 {
                                search_limits.push(SearchLimit::Nodes(nodes));
                            }
                        }
                        _ => (),
                    }
                }

                if time_args != TimeArgs::default() {
                    search_limits.push(SearchLimit::Time(
                        self.time_manager
                            .calculate_search_time(time_args, self.board.color_to_move),
                    ));
                }

                create_thread_data!(thread_data, self.num_threads);

                let mut secondary_searchers = vec![];
                for id in 1..self.num_threads {
                    secondary_searchers.push(Searcher::new(
                        search_limits.clone(),
                        &self.zobrist_stack,
                        &self.history,
                        &self.tt,
                        self.tablebase,
                        thread_data.get_secondary(id),
                    ));
                }

                let mut primary_searcher = Searcher::new(
                    search_limits.clone(),
                    &self.zobrist_stack,
                    &self.history,
                    &self.tt,
                    self.tablebase,
                    thread_data,
                );

                search::write_stop_flag(false);
                thread::scope(|s| {
                    s.spawn(|| {
                        primary_searcher.go::<true>(&self.board, true);
                    });

                    for searcher in secondary_searchers.iter_mut() {
                        s.spawn(|| {
                            searcher.go::<false>(&self.board, false);
                        });
                    }

                    Self::handle_stop_and_quit(&mut self.stored_message);
                });

                primary_searcher.search_complete_actions(&mut self.history);
                self.tt.age_table();
            }
            UciCommand::SetOptionOverhead(overhead) => {
                self.time_manager =
                    TimeManager::new(overhead.clamp(Self::OVERHEAD_MIN, Self::OVERHEAD_MAX));
            }
            UciCommand::SetOptionHash(megabytes) => {
                self.tt = TranspositionTable::new(megabytes.clamp(Self::HASH_MIN, Self::HASH_MAX));
            }
            UciCommand::SetOptionThreads(count) => {
                self.num_threads = count.clamp(Self::THREADS_MIN, Self::THREADS_MAX);
            }
            UciCommand::SetOptionSyzygyPath(path) => {
                if path != Self::SYZYGY_PATH_DEFAULT {
                    self.tablebase.activate(path.as_str());
                }
            }
            UciCommand::StaticEval => {
                println!(
                    "Position: {}\nStatic eval: {} cp",
                    self.board.to_fen(),
                    evaluate(&self.board)
                );
            }
        }
    }

    fn handle_stop_and_quit(stored_message: &mut Option<String>) {
        loop {
            let buffer = Self::read_uci_input();

            match buffer.as_str().trim() {
                "isready" => println!("readyok"),
                "quit" => kill_program(),
                "stop" => {
                    search::write_stop_flag(true);
                    return;
                }
                _ => {
                    if search::stop_flag_is_set() {
                        *stored_message = Some(buffer);
                        return;
                    }
                    eprintln!(
                        "Cannot handle command \"{}\" while searching",
                        buffer.trim()
                    );
                }
            };
        }
    }
}
