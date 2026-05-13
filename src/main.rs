use ChessEngine2::board::board::Board;
use ChessEngine2::board::debug::Debug;

fn main() {
    let board = Board::new();

    Debug::print_board(&board);
}