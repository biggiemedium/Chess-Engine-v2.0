use crate::board::board::Board;
use crate::board::bitboard::{self, Bitboard};

// https://www.chessprogramming.org/Pawn_Structure
// https://www.chessprogramming.org/Pawn_Hash_Table
pub struct PawnStructureEvaluator;

impl PawnStructureEvaluator {
    // Bonuses
    const PASSED_PAWN_BONUS: [i32; 8] = [0, 10, 20, 40, 70, 120, 200, 0]; // By rank
    const CONNECTED_PAWN_BONUS: i32 = 10;
    const PHALANX_BONUS: i32 = 15; // Pawns side-by-side
    const PROTECTED_PASSED_PAWN_BONUS: i32 = 20;

    // Penalties
    const ISOLATED_PAWN_PENALTY: i32 = -15;
    const DOUBLED_PAWN_PENALTY: i32 = -12;
    const BACKWARD_PAWN_PENALTY: i32 = -10;
    const WEAK_PAWN_PENALTY: i32 = -8;

    pub fn evaluate(board: &Board) -> i32 {
        let mut score = 0;

        // Evaluate white pawns
        score += Self::evaluate_pawns(
            board.white_pawns,
            board.black_pawns,
            board.white_pawns,
            true,
        );

        // Evaluate black pawns
        score -= Self::evaluate_pawns(
            board.black_pawns,
            board.white_pawns,
            board.black_pawns,
            false,
        );

        score
    }

    fn evaluate_pawns(
        friendly_pawns: Bitboard,
        enemy_pawns: Bitboard,
        all_friendly_pawns: Bitboard,
        is_white: bool,
    ) -> i32 {
        let mut score = 0;
        let mut pawns = friendly_pawns;

        while pawns != 0 {
            let sq = pawns.trailing_zeros() as u8;
            let file = sq % 8;
            let rank = sq / 8;

            // Isolated pawns
            if Self::is_isolated(sq, all_friendly_pawns) {
                score += Self::ISOLATED_PAWN_PENALTY;
            }

            // Doubled pawns
            if Self::is_doubled(sq, all_friendly_pawns, is_white) {
                score += Self::DOUBLED_PAWN_PENALTY;
            }

            // Passed pawns
            if Self::is_passed(sq, enemy_pawns, is_white) {
                let pawn_rank = if is_white { rank } else { 7 - rank };
                score += Self::PASSED_PAWN_BONUS[pawn_rank as usize];

                // Bonus for protected passed pawns
                if Self::is_protected(sq, all_friendly_pawns, is_white) {
                    score += Self::PROTECTED_PASSED_PAWN_BONUS;
                }
            }

            // Backward pawns
            if Self::is_backward(sq, all_friendly_pawns, enemy_pawns, is_white) {
                score += Self::BACKWARD_PAWN_PENALTY;
            }

            // Connected pawns (diagonal support)
            if Self::is_connected(sq, all_friendly_pawns, is_white) {
                score += Self::CONNECTED_PAWN_BONUS;
            }

            // Phalanx (side-by-side pawns)
            if Self::is_phalanx(sq, all_friendly_pawns) {
                score += Self::PHALANX_BONUS;
            }

            // Weak pawns (neither isolated nor backward, but not protected)
            if !Self::is_protected(sq, all_friendly_pawns, is_white)
                && !Self::is_isolated(sq, all_friendly_pawns)
            {
                score += Self::WEAK_PAWN_PENALTY;
            }

            pawns &= pawns - 1;
        }

        score
    }

    // Check if pawn has no friendly pawns on adjacent files
    #[inline]
    fn is_isolated(sq: u8, friendly_pawns: Bitboard) -> bool {
        let file = sq % 8;
        let mut adjacent_files_mask = 0u64;

        if file > 0 {
            adjacent_files_mask |= Self::file_mask(file - 1);
        }
        if file < 7 {
            adjacent_files_mask |= Self::file_mask(file + 1);
        }

        friendly_pawns & adjacent_files_mask == 0
    }

    // Check if there are multiple pawns on the same file
    #[inline]
    fn is_doubled(sq: u8, friendly_pawns: Bitboard, is_white: bool) -> bool {
        let file = sq % 8;
        let rank = sq / 8;
        let file_mask = Self::file_mask(file);

        let pawns_on_file = friendly_pawns & file_mask;

        // Check if there's another pawn ahead on the same file
        if is_white {
            let ahead_mask = file_mask & (0xFFFFFFFFFFFFFFFFu64 << ((rank + 1) * 8));
            pawns_on_file & ahead_mask != 0
        } else {
            let ahead_mask = file_mask & ((1u64 << (rank * 8)) - 1);
            pawns_on_file & ahead_mask != 0
        }
    }

    // Check if pawn has no enemy pawns in front on same or adjacent files
    #[inline]
    fn is_passed(sq: u8, enemy_pawns: Bitboard, is_white: bool) -> bool {
        let file = sq % 8;
        let rank = sq / 8;

        let mut front_span = Self::file_mask(file);
        if file > 0 {
            front_span |= Self::file_mask(file - 1);
        }
        if file < 7 {
            front_span |= Self::file_mask(file + 1);
        }

        // Mask to only include squares in front of this pawn
        if is_white {
            front_span &= 0xFFFFFFFFFFFFFFFFu64 << ((rank + 1) * 8);
        } else {
            front_span &= (1u64 << (rank * 8)) - 1;
        }

        enemy_pawns & front_span == 0
    }

    // Check if pawn is protected by another friendly pawn
    #[inline]
    fn is_protected(sq: u8, friendly_pawns: Bitboard, is_white: bool) -> bool {
        let file = sq % 8;
        let rank = sq / 8;

        if is_white {
            // White pawns attack from below (rank - 1)
            if rank == 0 {
                return false;
            }

            let mut defenders = 0u64;
            if file > 0 {
                defenders |= bitboard::bit((rank - 1) * 8 + (file - 1));
            }
            if file < 7 {
                defenders |= bitboard::bit((rank - 1) * 8 + (file + 1));
            }

            friendly_pawns & defenders != 0
        } else {
            // Black pawns attack from above (rank + 1)
            if rank == 7 {
                return false;
            }

            let mut defenders = 0u64;
            if file > 0 {
                defenders |= bitboard::bit((rank + 1) * 8 + (file - 1));
            }
            if file < 7 {
                defenders |= bitboard::bit((rank + 1) * 8 + (file + 1));
            }

            friendly_pawns & defenders != 0
        }
    }

    // Check if pawn is backward (can't advance safely and can't be supported)
    #[inline]
    fn is_backward(
        sq: u8,
        friendly_pawns: Bitboard,
        enemy_pawns: Bitboard,
        is_white: bool,
    ) -> bool {
        let file = sq % 8;
        let rank = sq / 8;

        // Can't be backward on first/last rank
        if (is_white && rank >= 6) || (!is_white && rank <= 1) {
            return false;
        }

        // Check if pawn can't advance because the square ahead is controlled by enemy
        let square_ahead = if is_white {
            (rank + 1) * 8 + file
        } else {
            (rank - 1) * 8 + file
        };

        // Check if enemy pawns control the square ahead
        let enemy_controls_ahead = if is_white {
            let mut control = 0u64;
            if file > 0 {
                control |= (enemy_pawns >> 9) & bitboard::bit(square_ahead);
            }
            if file < 7 {
                control |= (enemy_pawns >> 7) & bitboard::bit(square_ahead);
            }
            control != 0
        } else {
            let mut control = 0u64;
            if file > 0 {
                control |= (enemy_pawns << 7) & bitboard::bit(square_ahead);
            }
            if file < 7 {
                control |= (enemy_pawns << 9) & bitboard::bit(square_ahead);
            }
            control != 0
        };

        // Check if no friendly pawns on adjacent files are behind or equal to support
        let adjacent_pawns_behind = {
            let mut mask = 0u64;

            if file > 0 {
                let adj_file_mask = Self::file_mask(file - 1);
                if is_white {
                    mask |= adj_file_mask & ((1u64 << ((rank + 1) * 8)) - 1);
                } else {
                    mask |= adj_file_mask & (0xFFFFFFFFFFFFFFFFu64 << (rank * 8));
                }
            }

            if file < 7 {
                let adj_file_mask = Self::file_mask(file + 1);
                if is_white {
                    mask |= adj_file_mask & ((1u64 << ((rank + 1) * 8)) - 1);
                } else {
                    mask |= adj_file_mask & (0xFFFFFFFFFFFFFFFFu64 << (rank * 8));
                }
            }

            friendly_pawns & mask == 0
        };

        enemy_controls_ahead && adjacent_pawns_behind
    }

    // Check if pawn is diagonally connected to another friendly pawn
    #[inline]
    fn is_connected(sq: u8, friendly_pawns: Bitboard, is_white: bool) -> bool {
        let file = sq % 8;
        let rank = sq / 8;

        let mut support_squares = 0u64;

        if is_white {
            if rank > 0 {
                if file > 0 {
                    support_squares |= bitboard::bit((rank - 1) * 8 + (file - 1));
                }
                if file < 7 {
                    support_squares |= bitboard::bit((rank - 1) * 8 + (file + 1));
                }
            }
        } else {
            if rank < 7 {
                if file > 0 {
                    support_squares |= bitboard::bit((rank + 1) * 8 + (file - 1));
                }
                if file < 7 {
                    support_squares |= bitboard::bit((rank + 1) * 8 + (file + 1));
                }
            }
        }

        friendly_pawns & support_squares != 0
    }

    // Check if pawn has a friendly pawn directly beside it
    #[inline]
    fn is_phalanx(sq: u8, friendly_pawns: Bitboard) -> bool {
        let file = sq % 8;
        let rank = sq / 8;

        let mut adjacent = 0u64;
        if file > 0 {
            adjacent |= bitboard::bit(rank * 8 + (file - 1));
        }
        if file < 7 {
            adjacent |= bitboard::bit(rank * 8 + (file + 1));
        }

        friendly_pawns & adjacent != 0
    }

    #[inline]
    fn file_mask(file: u8) -> Bitboard {
        0x0101010101010101u64 << file
    }
}