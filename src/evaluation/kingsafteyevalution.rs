use crate::board::bitboard;
use crate::board::bitboard::Bitboard;
use crate::board::board::Board;

pub struct KingSafteyEvaluation;

impl KingSafteyEvaluation {

    // penalties
    const OPEN_FILE_NEXT_TO_KING: i32 = -20;
    const HALF_OPEN_FILE_NEXT_TO_KING: i32 = -10;
    const PAWN_SHIELD_BONUS: i32 = 10;
    const ENEMY_ATTACKER_NEAR_KING: i32 = -15;
    const KING_ON_OPEN_FILE: i32 = -30;

    pub fn evaluate(board: &Board) -> i32 {
        let mut score = 0;

        score += Self::evaluate_king_safety(
            board,
            board.white_king,
            board.white_pawns,
            board.black_pawns,
            board.black_queens,
            board.black_rooks,
            board.black_bishops,
            board.black_knights,
            true,
        );

        score -= Self::evaluate_king_safety(
            board,
            board.black_king,
            board.black_pawns,
            board.white_pawns,
            board.white_queens,
            board.white_rooks,
            board.white_bishops,
            board.white_knights,
            false,
        );

        score
    }

    fn evaluate_king_safety(
        board: &Board,
        king: Bitboard,
        friendly_pawns: Bitboard,
        enemy_pawns: Bitboard,
        enemy_queens: Bitboard,
        enemy_rooks: Bitboard,
        enemy_bishops: Bitboard,
        enemy_knights: Bitboard,
        is_white: bool,
    ) -> i32 {
        if king == 0 {
            return 0;
        }

        let mut score = 0;
        let king_sq = king.trailing_zeros() as u8;
        let king_file = king_sq % 8;
        let king_rank = king_sq / 8;

        // Evaluate pawn shield
        score += Self::evaluate_pawn_shield(king_sq, friendly_pawns, is_white);

        // Check for open/half-open files near king
        for file_offset in -1..=1i8 {
            let file = king_file as i8 + file_offset;
            if file >= 0 && file < 8 {
                let file_mask = Self::file_mask(file as u8);

                let friendly_pawns_on_file = friendly_pawns & file_mask;
                let enemy_pawns_on_file = enemy_pawns & file_mask;

                // King on open file (no pawns at all)
                if file_offset == 0 && friendly_pawns_on_file == 0 && enemy_pawns_on_file == 0 {
                    score += Self::KING_ON_OPEN_FILE;
                }

                // Open file next to king (no pawns from either side)
                if friendly_pawns_on_file == 0 && enemy_pawns_on_file == 0 {
                    score += Self::OPEN_FILE_NEXT_TO_KING;
                }
                // Half-open file (no friendly pawns, but enemy has pawns)
                else if friendly_pawns_on_file == 0 && enemy_pawns_on_file != 0 {
                    score += Self::HALF_OPEN_FILE_NEXT_TO_KING;
                }
            }
        }

        // Check for enemy pieces near king
        let king_zone = Self::king_zone(king_sq);

        // Queens near king are very dangerous
        if enemy_queens & king_zone != 0 {
            score += Self::ENEMY_ATTACKER_NEAR_KING * 3;
        }

        // Rooks near king
        if enemy_rooks & king_zone != 0 {
            score += Self::ENEMY_ATTACKER_NEAR_KING * 2;
        }

        // Bishops near king
        if enemy_bishops & king_zone != 0 {
            score += Self::ENEMY_ATTACKER_NEAR_KING;
        }

        // Knights near king
        if enemy_knights & king_zone != 0 {
            score += Self::ENEMY_ATTACKER_NEAR_KING;
        }

        // Bonus for castled king (king on g-file or c-file on back rank)
        if is_white && king_rank == 0 {
            if king_file == 6 || king_file == 2 {
                score += 30; // Castled position bonus
            }
        } else if !is_white && king_rank == 7 {
            if king_file == 6 || king_file == 2 {
                score += 30; // Castled position bonus
            }
        }

        // Penalty for king in center in opening/middlegame
        if king_file >= 2 && king_file <= 5 {
            score -= 20;
        }

        score
    }

    fn evaluate_pawn_shield(king_sq: u8, friendly_pawns: Bitboard, is_white: bool) -> i32 {
        let mut score = 0;
        let king_file = king_sq % 8;
        let king_rank = king_sq / 8;

        // Check pawns in front of king (1 and 2 squares ahead)
        for file_offset in -1..=1i8 {
            let file = king_file as i8 + file_offset;
            if file >= 0 && file < 8 {
                let file = file as u8;

                if is_white {
                    // Check one square in front
                    if king_rank < 7 {
                        let sq_front = (king_rank + 1) * 8 + file;
                        if friendly_pawns & bitboard::bit(sq_front) != 0 {
                            score += Self::PAWN_SHIELD_BONUS;
                        }
                    }

                    // Check two squares in front
                    if king_rank < 6 {
                        let sq_front2 = (king_rank + 2) * 8 + file;
                        if friendly_pawns & bitboard::bit(sq_front2) != 0 {
                            score += Self::PAWN_SHIELD_BONUS / 2;
                        }
                    }
                } else {
                    // Black king - check squares below
                    if king_rank > 0 {
                        let sq_front = (king_rank - 1) * 8 + file;
                        if friendly_pawns & bitboard::bit(sq_front) != 0 {
                            score += Self::PAWN_SHIELD_BONUS;
                        }
                    }

                    if king_rank > 1 {
                        let sq_front2 = (king_rank - 2) * 8 + file;
                        if friendly_pawns & bitboard::bit(sq_front2) != 0 {
                            score += Self::PAWN_SHIELD_BONUS / 2;
                        }
                    }
                }
            }
        }

        score
    }

    #[inline]
    fn file_mask(file: u8) -> Bitboard {
        0x0101010101010101u64 << file
    }

    // check for enemys near king
    #[inline]
    fn king_zone(king_sq: u8) -> Bitboard {
        let king_bb = bitboard::bit(king_sq);
        let mut zone = king_bb;

        // Add all adjacent squares
        zone |= (king_bb << 8);                          // up
        zone |= (king_bb >> 8);                          // down
        zone |= (king_bb << 1) & !0x0101010101010101u64; // right
        zone |= (king_bb >> 1) & !0x8080808080808080u64; // left
        zone |= (king_bb << 9) & !0x0101010101010101u64; // up-right
        zone |= (king_bb << 7) & !0x8080808080808080u64; // up-left
        zone |= (king_bb >> 9) & !0x8080808080808080u64; // down-left
        zone |= (king_bb >> 7) & !0x0101010101010101u64; // down-right

        // Extend the zone one more ring outside search zone
        let extended = zone;
        zone |= (extended << 8);
        zone |= (extended >> 8);
        zone |= (extended << 1) & !0x0101010101010101u64;
        zone |= (extended >> 1) & !0x8080808080808080u64;
        zone |= (extended << 9) & !0x0101010101010101u64;
        zone |= (extended << 7) & !0x8080808080808080u64;
        zone |= (extended >> 9) & !0x8080808080808080u64;
        zone |= (extended >> 7) & !0x0101010101010101u64;

        zone
    }
}