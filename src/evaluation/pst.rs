use crate::board::board::Board;
use crate::board::bitboard::{self, Bitboard};

pub struct PieceSquareTableEvaluator;

impl PieceSquareTableEvaluator {

    const PAWN_TABLE: [i32; 64] = [
        0,  0,  0,  0,  0,  0,  0,  0,
        50, 50, 50, 50, 50, 50, 50, 50,
        10, 10, 20, 30, 30, 20, 10, 10,
        5,  5, 10, 25, 25, 10,  5,  5,
        0,  0,  0, 20, 20,  0,  0,  0,
        5, -5,-10,  0,  0,-10, -5,  5,
        5, 10, 10,-20,-20, 10, 10,  5,
        0,  0,  0,  0,  0,  0,  0,  0
    ];

    const KNIGHT_TABLE: [i32; 64] = [
        -50,-40,-30,-30,-30,-30,-40,-50,
        -40,-20,  0,  0,  0,  0,-20,-40,
        -30,  0, 10, 15, 15, 10,  0,-30,
        -30,  5, 15, 20, 20, 15,  5,-30,
        -30,  0, 15, 20, 20, 15,  0,-30,
        -30,  5, 10, 15, 15, 10,  5,-30,
        -40,-20,  0,  5,  5,  0,-20,-40,
        -50,-40,-30,-30,-30,-30,-40,-50,
    ];

    const BISHOP_TABLE: [i32; 64] = [
        -20,-10,-10,-10,-10,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5, 10, 10,  5,  0,-10,
        -10,  5,  5, 10, 10,  5,  5,-10,
        -10,  0, 10, 10, 10, 10,  0,-10,
        -10, 10, 10, 10, 10, 10, 10,-10,
        -10,  5,  0,  0,  0,  0,  5,-10,
        -20,-10,-10,-10,-10,-10,-10,-20,
    ];

    const ROOK_TABLE: [i32; 64] = [
        0,  0,  0,  0,  0,  0,  0,  0,
        5, 10, 10, 10, 10, 10, 10,  5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        0,  0,  0,  5,  5,  0,  0,  0
    ];

    const QUEEN_TABLE: [i32; 64] = [
        -20,-10,-10, -5, -5,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5,  5,  5,  5,  0,-10,
        -5,  0,  5,  5,  5,  5,  0, -5,
        0,  0,  5,  5,  5,  5,  0, -5,
        -10,  5,  5,  5,  5,  5,  0,-10,
        -10,  0,  5,  0,  0,  0,  0,-10,
        -20,-10,-10, -5, -5,-10,-10,-20
    ];

    const KING_MIDDLE_GAME_TABLE: [i32; 64] = [
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -20,-30,-30,-40,-40,-30,-30,-20,
        -10,-20,-20,-20,-20,-20,-20,-10,
        20, 20,  0,  0,  0,  0, 20, 20,
        20, 30, 10,  0,  0, 10, 30, 20
    ];

    const KING_END_GAME_TABLE: [i32; 64] = [
        -50,-40,-30,-20,-20,-30,-40,-50,
        -30,-20,-10,  0,  0,-10,-20,-30,
        -30,-10, 20, 30, 30, 20,-10,-30,
        -30,-10, 30, 40, 40, 30,-10,-30,
        -30,-10, 30, 40, 40, 30,-10,-30,
        -30,-10, 20, 30, 30, 20,-10,-30,
        -30,-30,  0,  0,  0,  0,-30,-30,
        -50,-30,-30,-30,-30,-30,-30,-50,
        ];

    pub fn evaluate(board: &Board) -> i32 {
        let mut score = 0;

        // white score
        score += Self::evaluate_pieces(board.white_pawns, &Self::PAWN_TABLE, false);
        score += Self::evaluate_pieces(board.white_knights, &Self::KNIGHT_TABLE, false);
        score += Self::evaluate_pieces(board.white_bishops, &Self::BISHOP_TABLE, false);
        score += Self::evaluate_pieces(board.white_rooks, &Self::ROOK_TABLE, false);
        score += Self::evaluate_pieces(board.white_queens, &Self::QUEEN_TABLE, false);
        score += Self::evaluate_pieces(board.white_king, &Self::KING_MIDDLE_GAME_TABLE, false);

        // black score
        score -= Self::evaluate_pieces(board.black_pawns, &Self::PAWN_TABLE, true);
        score -= Self::evaluate_pieces(board.black_knights, &Self::KNIGHT_TABLE, true);
        score -= Self::evaluate_pieces(board.black_bishops, &Self::BISHOP_TABLE, true);
        score -= Self::evaluate_pieces(board.black_rooks, &Self::ROOK_TABLE, true);
        score -= Self::evaluate_pieces(board.black_queens, &Self::QUEEN_TABLE, true);

        score -= Self::evaluate_pieces(board.black_king, &Self::KING_MIDDLE_GAME_TABLE, true);

        score
    }

    #[inline]
    fn is_endgame() -> bool {
        let piececount = 0;

        let whiteQueenAlive = false;
        let blackQueenAlive = false;

        false // TODO
    }

    #[inline]
    fn evaluate_pieces(mut pieces: Bitboard, table: &[i32; 64], flip: bool) -> i32 {
        let mut score = 0;
        while pieces != 0 {
            let sq = pieces.trailing_zeros() as u8;
            let index = if flip {
                // Flip the square for black's perspective (mirror vertically)
                sq ^ 56
            } else {
                sq
            };
            score += table[index as usize];
            pieces &= pieces - 1;
        }
        score
    }
}