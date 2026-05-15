use std::env;
use std::io::{self, Write};
use ChessEngine2::board::game::Game;
use ChessEngine2::board::uci::Uci;
use ChessEngine2::Network::bot_runner::BotRunner;

fn main() {
    let args: Vec<String> = env::args().collect();

    // If a CLI argument was passed, use it directly (useful for GUIs launching UCI)
    if let Some(arg) = args.get(1) {
        match arg.as_str() {
            "uci" => {
                let mut uci = Uci::new();
                uci.run();
                return;
            }
            "lichess" => {
                let mut runner = BotRunner::new();
                runner.run();
                return;
            }
            "play" => {
                let mut game = Game::new();
                game.play();
                return;
            }
            _ => {}
        }
    }

    // No argument — show interactive menu
    println!("╔══════════════════════════════════╗");
    println!("║         Chess Engine v2          ║");
    println!("╠══════════════════════════════════╣");
    println!("║  [1] Play locally                ║");
    println!("║  [2] Lichess bot mode            ║");
    println!("║  [3] UCI mode (for chess GUIs)   ║");
    println!("║  [q] Quit                        ║");
    println!("╚══════════════════════════════════╝");
    print!("\nSelect an option: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => {
            println!("\nStarting local game...\n");
            let mut game = Game::new();
            game.play();
        }
        "2" => {
            println!("\nConnecting to Lichess...\n");
            let mut runner = BotRunner::new();
            runner.run();
        }
        "3" => {
            // UCI mode is typically silent — no banner
            let mut uci = Uci::new();
            uci.run();
        }
        "q" | "quit" => {
            println!("Goodbye!");
        }
        other => {
            eprintln!("Unknown option: '{}'. Exiting.", other);
            std::process::exit(1);
        }
    }
}