use crate::{
    board_representation::{Board, START_FEN},
    chess_move::Move,
    zobrist_stack::ZobristStack,
    search::Searcher,
    time_management::{Milliseconds, TimeArgs, TimeManager},
    zobrist::hash_position,
};

use std::thread;

#[derive(Debug, Copy, Clone)]
pub enum ProgramStatus {
    Run,
    Quit,
}

enum UciCommand {
    Uci,
    IsReady,
    UciNewGame,
    Position(String, Vec<String>),
    Go(Vec<String>),
}

pub struct UciHandler {
    board: Board,
    zobrist_stack: ZobristStack,
    time_manager: TimeManager,
}

impl UciHandler {
    pub fn new() -> Self {
        let board = Board::from_fen(START_FEN);
        let zobrist_stack = ZobristStack::new(&board);
        Self {
            board,
            zobrist_stack,
            time_manager: TimeManager::new(),
        }
    }

    pub fn execute_instructions(&mut self) -> ProgramStatus {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("UCI Input Failure");

        let message = buffer.split_whitespace().collect::<Vec<&str>>();
        if let Some(cmd) = message.first() {
            match *cmd {
                "uci" => self.process_command(UciCommand::Uci),
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
                        for mv_str in &message[i..] {
                            mv_vec.push((*mv_str).to_string());
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
                "quit" => return ProgramStatus::Quit,
                _ => (),
            }
        }

        ProgramStatus::Run
    }

    fn process_command(&mut self, command: UciCommand) {
        match command {
            UciCommand::Uci => {
                println!("id name Wahoo v0.0.0");
                println!("id author Andrew Hockman");
                println!("uciok");
            }
            UciCommand::IsReady => println!("readyok"),
            UciCommand::UciNewGame => (),
            UciCommand::Position(fen, move_vec) => {
                let mut new_board = Board::from_fen(fen.as_str());
                let mut new_zobrist_stack = ZobristStack::new(&new_board);

                for mv_str in move_vec {
                    let mv = Move::from_string(mv_str.as_str(), &new_board);
                    let success = new_board.try_play_move(mv, &mut new_zobrist_stack);
                    if !success {
                        return;
                    }
                    new_zobrist_stack.add_hash(hash_position(&new_board));
                }

                self.board = new_board;
                self.zobrist_stack = new_zobrist_stack;
            }
            UciCommand::Go(arg_vec) => {
                let mut time_args = TimeArgs::default();
                let mut args_iterator = arg_vec.iter();

                while let Some(arg) = args_iterator.next() {
                    match arg.as_str() {
                        "wtime" => {
                            time_args.w_time = args_iterator
                                .next()
                                .unwrap()
                                .parse::<Milliseconds>()
                                .unwrap_or(0);
                        }
                        "btime" => {
                            time_args.b_time = args_iterator
                                .next()
                                .unwrap()
                                .parse::<Milliseconds>()
                                .unwrap_or(0);
                        }
                        "winc" => {
                            time_args.w_inc = args_iterator
                                .next()
                                .unwrap()
                                .parse::<Milliseconds>()
                                .unwrap_or(0);
                        }
                        "binc" => {
                            time_args.b_inc = args_iterator
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
                        "infinite" => time_args.infinite_mode = true,
                        _ => (),
                    }
                }

                let search_timer = self
                    .time_manager
                    .construct_search_timer(time_args, self.board.color_to_move);

                let mut searcher = Searcher::new(search_timer, self.zobrist_stack.clone());

                let board = self.board.clone();

                thread::spawn(move || {
                    searcher.go(&board);
                });
            }
        }
    }
}
