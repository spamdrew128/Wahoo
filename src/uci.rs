use crate::board_representation::{Board, START_FEN};

#[derive(Debug, Copy, Clone)]
pub enum ProgramStatus {
    Run,
    Quit,
}

#[derive(Debug, Copy, Clone)]
enum UciCommand {
    Uci,
    IsReady,
    UciNewGame,
}

pub struct UciHandler {
    board: Board,
}

impl UciHandler {
    pub fn new() -> Self {
        Self {
            board: Board::from_fen(START_FEN),
        }
    }

    pub fn execute_instructions(&self) -> ProgramStatus {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("UCI Input Failure");

        let message = buffer.split_whitespace().collect::<Vec<&str>>();
        if let Some(cmd) = message.first() {
            match *cmd {
                "uci" => self.process_command(UciCommand::Uci),
                "isready" => self.process_command(UciCommand::IsReady),
                "quit" => return ProgramStatus::Quit,
                _ => (),
            }
        }

        ProgramStatus::Run
    }

    fn process_command(&self, command: UciCommand) {
        match command {
            UciCommand::Uci => {
                println!("id name Wahoo v0.0.0");
                println!("id author Andrew Hockman");
                println!("uci ok");
            }
            UciCommand::IsReady => println!("readyok"),
            UciCommand::UciNewGame => unimplemented!(),
        }
    }
}
