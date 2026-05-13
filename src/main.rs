use std::env;
use ChessEngine2::board::game::Game;
use ChessEngine2::board::uci::Uci;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "uci" {
        // UCI mode
        let mut uci = Uci::new();
        uci.run();
    } else {
        // Interactive game mode
        let mut game = Game::new();
        game.play();
    }
}