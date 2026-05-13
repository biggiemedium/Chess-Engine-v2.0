use crate::board::bitboard::Bitboard;
use crate::board::board::Board;

pub struct Debug;

impl Debug {

    pub fn print_board(board: &Board) {
        println!("  +---+---+---+---+---+---+---+---+");

        for rank in (0..8).rev() {
            print!("{} |", rank + 1);

            for file in 0..8 {
                let sq = rank * 8 + file;
                let piece = Self::piece_at(board, sq);
                print!(" {} |", piece);
            }

            println!();
            println!("  +---+---+---+---+---+---+---+---+");
        }

        println!("    a   b   c   d   e   f   g   h");
    }

    fn piece_at(board: &Board, sq: u8) -> char {
        let mask = 1u64 << sq;

        if board.white_pawns   & mask != 0 { return 'P'; }
        if board.white_knights & mask != 0 { return 'N'; }
        if board.white_bishops & mask != 0 { return 'B'; }
        if board.white_rooks   & mask != 0 { return 'R'; }
        if board.white_queens  & mask != 0 { return 'Q'; }
        if board.white_king    & mask != 0 { return 'K'; }

        if board.black_pawns   & mask != 0 { return 'p'; }
        if board.black_knights & mask != 0 { return 'n'; }
        if board.black_bishops & mask != 0 { return 'b'; }
        if board.black_rooks   & mask != 0 { return 'r'; }
        if board.black_queens  & mask != 0 { return 'q'; }
        if board.black_king    & mask != 0 { return 'k'; }

        '.'
    }
}