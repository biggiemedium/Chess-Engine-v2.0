use crate::board::board::Board;
use crate::board::board::PieceType;

pub fn piece_value(board: &Board, square: u8) -> i32 {
    if let Some((piece, is_white)) = board.piece_at(square) {
        match piece {
            PieceType::Pawn => 100,
            PieceType::Knight => 320,
            PieceType::Bishop => 330,
            PieceType::Rook => 500,
            PieceType::Queen => 900,
            PieceType::King => 0,
        }
    } else {
        0
    }
}