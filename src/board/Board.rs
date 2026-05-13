use crate::board::bitboard;
use crate::board::bitboard::Bitboard;
use crate::movegen::r#move::Move;

pub struct Board {

    // White
    pub white_pawns: Bitboard,
    pub white_knights: Bitboard,
    pub white_bishops: Bitboard,
    pub white_rooks: Bitboard,
    pub white_queens: Bitboard,
    pub white_king: Bitboard,

    // Black
    pub black_pawns: Bitboard,
    pub black_knights: Bitboard,
    pub black_bishops: Bitboard,
    pub black_rooks: Bitboard,
    pub black_queens: Bitboard,
    pub black_king: Bitboard,

}

impl Board {
    pub fn new() -> Self {
        let mut b = Self {
            white_pawns: 0,
            white_knights: 0,
            white_bishops: 0,
            white_rooks: 0,
            white_queens: 0,
            white_king: 0,

            black_pawns: 0,
            black_knights: 0,
            black_bishops: 0,
            black_rooks: 0,
            black_queens: 0,
            black_king: 0,
        };

        b.init_start();
        b
    }

    pub fn init_start(&mut self) {
        // Pawns
        self.white_pawns = 0x000000000000FF00;
        self.black_pawns = 0x00FF000000000000;

        // Rooks
        self.white_rooks = 0x0000000000000081;
        self.black_rooks = 0x8100000000000000;

        // Knights
        self.white_knights = 0x0000000000000042;
        self.black_knights = 0x4200000000000000;

        // Bishops
        self.white_bishops = 0x0000000000000024;
        self.black_bishops = 0x2400000000000000;

        // Queens
        self.white_queens = 0x0000000000000008;
        self.black_queens = 0x0800000000000000;

        // Kings
        self.white_king = 0x0000000000000010;
        self.black_king = 0x1000000000000000;
    }

    /// Get all white pieces as a single bitboard
    #[inline]
    pub fn white_occupancy(&self) -> Bitboard {
        self.white_pawns | self.white_knights | self.white_bishops
            | self.white_rooks | self.white_queens | self.white_king
    }

    /// Get all black pieces as a single bitboard
    #[inline]
    pub fn black_occupancy(&self) -> Bitboard {
        self.black_pawns | self.black_knights | self.black_bishops
            | self.black_rooks | self.black_queens | self.black_king
    }

    /// Get all pieces (white and black) as a single bitboard
    #[inline]
    pub fn all_occupancy(&self) -> Bitboard {
        self.white_occupancy() | self.black_occupancy()
    }

    /// Get the piece type at a given square (if any)
    pub fn piece_at(&self, square: u8) -> Option<(PieceType, bool)> {
        let bb = bitboard::bit(square);

        if self.white_pawns & bb != 0 { return Some((PieceType::Pawn, true)); }
        if self.white_knights & bb != 0 { return Some((PieceType::Knight, true)); }
        if self.white_bishops & bb != 0 { return Some((PieceType::Bishop, true)); }
        if self.white_rooks & bb != 0 { return Some((PieceType::Rook, true)); }
        if self.white_queens & bb != 0 { return Some((PieceType::Queen, true)); }
        if self.white_king & bb != 0 { return Some((PieceType::King, true)); }

        if self.black_pawns & bb != 0 { return Some((PieceType::Pawn, false)); }
        if self.black_knights & bb != 0 { return Some((PieceType::Knight, false)); }
        if self.black_bishops & bb != 0 { return Some((PieceType::Bishop, false)); }
        if self.black_rooks & bb != 0 { return Some((PieceType::Rook, false)); }
        if self.black_queens & bb != 0 { return Some((PieceType::Queen, false)); }
        if self.black_king & bb != 0 { return Some((PieceType::King, false)); }

        None
    }

    /// Make a move on the board (modifies board state)
    pub fn make_move(&mut self, mv: &Move, white_to_move: bool) {
        let from_bb = bitboard::bit(mv.from);
        let to_bb = bitboard::bit(mv.to);
        let from_to = from_bb | to_bb;

        if white_to_move {
            // First, remove any captured black piece
            if mv.is_capture() {
                self.black_pawns &= !to_bb;
                self.black_knights &= !to_bb;
                self.black_bishops &= !to_bb;
                self.black_rooks &= !to_bb;
                self.black_queens &= !to_bb;
            }

            // Move the white piece
            if self.white_pawns & from_bb != 0 {
                if mv.is_promotion() {
                    // Remove pawn from source square
                    self.white_pawns &= !from_bb;
                    // Add promoted piece to destination
                    if mv.flags.is_promotion_knight() {
                        self.white_knights |= to_bb;
                    } else if mv.flags.is_promotion_bishop() {
                        self.white_bishops |= to_bb;
                    } else if mv.flags.is_promotion_rook() {
                        self.white_rooks |= to_bb;
                    } else {
                        self.white_queens |= to_bb;
                    }
                } else {
                    // Regular pawn move
                    self.white_pawns ^= from_to;
                }
            } else if self.white_knights & from_bb != 0 {
                self.white_knights ^= from_to;
            } else if self.white_bishops & from_bb != 0 {
                self.white_bishops ^= from_to;
            } else if self.white_rooks & from_bb != 0 {
                self.white_rooks ^= from_to;
            } else if self.white_queens & from_bb != 0 {
                self.white_queens ^= from_to;
            } else if self.white_king & from_bb != 0 {
                self.white_king ^= from_to;
            }
        } else {
            // Black's turn - same logic but reversed
            if mv.is_capture() {
                self.white_pawns &= !to_bb;
                self.white_knights &= !to_bb;
                self.white_bishops &= !to_bb;
                self.white_rooks &= !to_bb;
                self.white_queens &= !to_bb;
            }

            if self.black_pawns & from_bb != 0 {
                if mv.is_promotion() {
                    self.black_pawns &= !from_bb;
                    if mv.flags.is_promotion_knight() {
                        self.black_knights |= to_bb;
                    } else if mv.flags.is_promotion_bishop() {
                        self.black_bishops |= to_bb;
                    } else if mv.flags.is_promotion_rook() {
                        self.black_rooks |= to_bb;
                    } else {
                        self.black_queens |= to_bb;
                    }
                } else {
                    self.black_pawns ^= from_to;
                }
            } else if self.black_knights & from_bb != 0 {
                self.black_knights ^= from_to;
            } else if self.black_bishops & from_bb != 0 {
                self.black_bishops ^= from_to;
            } else if self.black_rooks & from_bb != 0 {
                self.black_rooks ^= from_to;
            } else if self.black_queens & from_bb != 0 {
                self.black_queens ^= from_to;
            } else if self.black_king & from_bb != 0 {
                self.black_king ^= from_to;
            }
        }
    }

    /// Unmake a move
    pub fn unmake_move(&mut self, mv: &Move, white_to_move: bool, captured_piece: Option<PieceType>) {
        let from_bb = bitboard::bit(mv.from);
        let to_bb = bitboard::bit(mv.to);
        let from_to = from_bb | to_bb;

        if white_to_move {
            // Move piece back
            if mv.is_promotion() {
                // Remove promoted piece, restore pawn
                self.white_knights &= !to_bb;
                self.white_bishops &= !to_bb;
                self.white_rooks &= !to_bb;
                self.white_queens &= !to_bb;
                self.white_pawns |= from_bb;
            } else if self.white_pawns & to_bb != 0 {
                self.white_pawns ^= from_to;
            } else if self.white_knights & to_bb != 0 {
                self.white_knights ^= from_to;
            } else if self.white_bishops & to_bb != 0 {
                self.white_bishops ^= from_to;
            } else if self.white_rooks & to_bb != 0 {
                self.white_rooks ^= from_to;
            } else if self.white_queens & to_bb != 0 {
                self.white_queens ^= from_to;
            } else if self.white_king & to_bb != 0 {
                self.white_king ^= from_to;
            }

            // Restore captured piece
            if let Some(piece) = captured_piece {
                match piece {
                    PieceType::Pawn => self.black_pawns |= to_bb,
                    PieceType::Knight => self.black_knights |= to_bb,
                    PieceType::Bishop => self.black_bishops |= to_bb,
                    PieceType::Rook => self.black_rooks |= to_bb,
                    PieceType::Queen => self.black_queens |= to_bb,
                    PieceType::King => self.black_king |= to_bb,
                }
            }
        } else {
            // Black unmake - mirror logic
            if mv.is_promotion() {
                self.black_knights &= !to_bb;
                self.black_bishops &= !to_bb;
                self.black_rooks &= !to_bb;
                self.black_queens &= !to_bb;
                self.black_pawns |= from_bb;
            } else if self.black_pawns & to_bb != 0 {
                self.black_pawns ^= from_to;
            } else if self.black_knights & to_bb != 0 {
                self.black_knights ^= from_to;
            } else if self.black_bishops & to_bb != 0 {
                self.black_bishops ^= from_to;
            } else if self.black_rooks & to_bb != 0 {
                self.black_rooks ^= from_to;
            } else if self.black_queens & to_bb != 0 {
                self.black_queens ^= from_to;
            } else if self.black_king & to_bb != 0 {
                self.black_king ^= from_to;
            }

            if let Some(piece) = captured_piece {
                match piece {
                    PieceType::Pawn => self.white_pawns |= to_bb,
                    PieceType::Knight => self.white_knights |= to_bb,
                    PieceType::Bishop => self.white_bishops |= to_bb,
                    PieceType::Rook => self.white_rooks |= to_bb,
                    PieceType::Queen => self.white_queens |= to_bb,
                    PieceType::King => self.white_king |= to_bb,
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}