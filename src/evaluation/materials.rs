use crate::board::board::Board;
use crate::board::bitboard;

pub struct MaterialEvaluator;

pub mod piece_value {
    pub const PAWN: i32 = 100;
    pub const KNIGHT: i32 = 320;
    pub const BISHOP: i32 = 330;
    pub const ROOK: i32 = 500;
    pub const QUEEN: i32 = 900;
}

impl MaterialEvaluator {

    pub const PAWN_VALUE: i32 = 100;
    pub const KNIGHT_VALUE: i32 = 320;
    pub const BISHOP_VALUE: i32 = 330;
    pub const ROOK_VALUE: i32 = 500;
    pub const QUEEN_VALUE: i32 = 900;

    pub fn evaluate(board: &Board) -> i32 {
        let mut score = 0;

        // white
        score += bitboard::popcount(board.white_pawns) as i32 * Self::PAWN_VALUE;
        score += bitboard::popcount(board.white_knights) as i32 * Self::KNIGHT_VALUE;
        score += bitboard::popcount(board.white_bishops) as i32 * Self::BISHOP_VALUE;
        score += bitboard::popcount(board.white_rooks) as i32 * Self::ROOK_VALUE;
        score += bitboard::popcount(board.white_queens) as i32 * Self::QUEEN_VALUE;

        // (-) black
        score -= bitboard::popcount(board.black_pawns) as i32 * Self::PAWN_VALUE;
        score -= bitboard::popcount(board.black_knights) as i32 * Self::KNIGHT_VALUE;
        score -= bitboard::popcount(board.black_bishops) as i32 * Self::BISHOP_VALUE;
        score -= bitboard::popcount(board.black_rooks) as i32 * Self::ROOK_VALUE;
        score -= bitboard::popcount(board.black_queens) as i32 * Self::QUEEN_VALUE;

        score
    }
}