use engine::{bench, uci};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    for arg in args {
        if arg == "bench" {
            bench::bench();
            return;
        }
    }

    std::env::set_var("RUST_BACKTRACE", "1");

    let mut uci_handler = uci::UciHandler::new();
    loop {
        uci_handler.execute_instructions()
    };
}
