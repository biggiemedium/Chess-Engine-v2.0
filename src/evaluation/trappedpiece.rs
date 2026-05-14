use crate::board::board::Board;
use crate::board::bitboard::{self, Bitboard};
use crate::movegen::movegeneration::MoveGen;

// https://www.chessprogramming.org/Trapped_Pieces
// https://www.chessprogramming.org/Mobility
pub struct TrappedPieceEvaluator;

impl TrappedPieceEvaluator {
    // Trapped piece penalties
    const TRAPPED_KNIGHT_PENALTY: i32 = -150;
    const TRAPPED_BISHOP_PENALTY: i32 = -150;
    const TRAPPED_ROOK_PENALTY: i32 = -100;
    const BLOCKED_ROOK_PENALTY: i32 = -50;

    // Bishop trapped in corner specific patterns
    const BISHOP_TRAPPED_A7: i32 = -150;
    const BISHOP_TRAPPED_A6: i32 = -100;
    const BISHOP_TRAPPED_H7: i32 = -150;
    const BISHOP_TRAPPED_H6: i32 = -100;

    // Knight trapped on rim
    const KNIGHT_ON_RIM_PENALTY: i32 = -30;

    pub fn evaluate(board: &Board, movegen: &MoveGen) -> i32 {
        let mut score = 0;

        // Evaluate white pieces
        score += Self::evaluate_side(
            board,
            movegen,
            board.white_knights,
            board.white_bishops,
            board.white_rooks,
            board.white_pawns,
            board.black_pawns,
            board.white_occupancy(),
            true,
        );

        // Evaluate black pieces
        score -= Self::evaluate_side(
            board,
            movegen,
            board.black_knights,
            board.black_bishops,
            board.black_rooks,
            board.black_pawns,
            board.white_pawns,
            board.black_occupancy(),
            false,
        );

        score
    }

    fn evaluate_side(
        board: &Board,
        movegen: &MoveGen,
        knights: Bitboard,
        bishops: Bitboard,
        rooks: Bitboard,
        friendly_pawns: Bitboard,
        enemy_pawns: Bitboard,
        friendly_occupancy: Bitboard,
        is_white: bool,
    ) -> i32 {
        let mut score = 0;

        // Evaluate trapped knights
        score += Self::evaluate_knights(knights, friendly_pawns, enemy_pawns, is_white);

        // Evaluate trapped bishops
        score += Self::evaluate_bishops(bishops, friendly_pawns, enemy_pawns, is_white);

        // Evaluate blocked/trapped rooks
        score += Self::evaluate_rooks(rooks, friendly_pawns, is_white);

        score
    }

    fn evaluate_knights(
        knights: Bitboard,
        friendly_pawns: Bitboard,
        enemy_pawns: Bitboard,
        is_white: bool,
    ) -> i32 {
        let mut score = 0;
        let mut kn = knights;

        while kn != 0 {
            let sq = kn.trailing_zeros() as u8;
            let file = sq % 8;
            let rank = sq / 8;

            // Knight on the rim is dim
            if file == 0 || file == 7 || rank == 0 || rank == 7 {
                score += Self::KNIGHT_ON_RIM_PENALTY;
            }

            // Specific trapped knight patterns
            if is_white {
                // Knight trapped on a8 by pawns on a7, b7
                if sq == 56 {
                    if (friendly_pawns & bitboard::bit(48)) != 0 &&
                        (enemy_pawns & bitboard::bit(49)) != 0 {
                        score += Self::TRAPPED_KNIGHT_PENALTY;
                    }
                }
                // Knight trapped on h8 by pawns
                if sq == 63 {
                    if (friendly_pawns & bitboard::bit(55)) != 0 &&
                        (enemy_pawns & bitboard::bit(54)) != 0 {
                        score += Self::TRAPPED_KNIGHT_PENALTY;
                    }
                }
                // Knight trapped on a7
                if sq == 48 && (enemy_pawns & bitboard::bit(49)) != 0 {
                    score += Self::TRAPPED_KNIGHT_PENALTY / 2;
                }
                // Knight trapped on h7
                if sq == 55 && (enemy_pawns & bitboard::bit(54)) != 0 {
                    score += Self::TRAPPED_KNIGHT_PENALTY / 2;
                }
            } else {
                // Knight trapped on a1 by pawns on a2, b2
                if sq == 0 {
                    if (friendly_pawns & bitboard::bit(8)) != 0 &&
                        (enemy_pawns & bitboard::bit(9)) != 0 {
                        score += Self::TRAPPED_KNIGHT_PENALTY;
                    }
                }
                // Knight trapped on h1 by pawns
                if sq == 7 {
                    if (friendly_pawns & bitboard::bit(15)) != 0 &&
                        (enemy_pawns & bitboard::bit(14)) != 0 {
                        score += Self::TRAPPED_KNIGHT_PENALTY;
                    }
                }
                // Knight trapped on a2
                if sq == 8 && (enemy_pawns & bitboard::bit(9)) != 0 {
                    score += Self::TRAPPED_KNIGHT_PENALTY / 2;
                }
                // Knight trapped on h2
                if sq == 15 && (enemy_pawns & bitboard::bit(14)) != 0 {
                    score += Self::TRAPPED_KNIGHT_PENALTY / 2;
                }
            }

            kn &= kn - 1;
        }

        score
    }

    fn evaluate_bishops(
        bishops: Bitboard,
        friendly_pawns: Bitboard,
        enemy_pawns: Bitboard,
        is_white: bool,
    ) -> i32 {
        let mut score = 0;
        let mut bp = bishops;

        while bp != 0 {
            let sq = bp.trailing_zeros() as u8;
            let file = sq % 8;
            let rank = sq / 8;

            if is_white {
                // Classic trapped bishop on a7/h7
                // Bishop on a7 trapped by pawn on b6
                if sq == 48 && (enemy_pawns & bitboard::bit(41)) != 0 {
                    score += Self::BISHOP_TRAPPED_A7;
                }
                // Bishop on a6 blocked by pawn on b5
                if sq == 40 && (enemy_pawns & bitboard::bit(33)) != 0 {
                    score += Self::BISHOP_TRAPPED_A6;
                }
                // Bishop on h7 trapped by pawn on g6
                if sq == 55 && (enemy_pawns & bitboard::bit(46)) != 0 {
                    score += Self::BISHOP_TRAPPED_H7;
                }
                // Bishop on h6 blocked by pawn on g5
                if sq == 47 && (enemy_pawns & bitboard::bit(38)) != 0 {
                    score += Self::BISHOP_TRAPPED_H6;
                }

                // Bishop trapped behind own pawns (fianchetto gone wrong)
                if sq == 41 && (friendly_pawns & bitboard::bit(49)) != 0 &&
                    (enemy_pawns & bitboard::bit(50)) != 0 {
                    score += Self::TRAPPED_BISHOP_PENALTY / 2;
                }
                if sq == 46 && (friendly_pawns & bitboard::bit(54)) != 0 &&
                    (enemy_pawns & bitboard::bit(53)) != 0 {
                    score += Self::TRAPPED_BISHOP_PENALTY / 2;
                }
            } else {
                // Black bishop trapped patterns
                // Bishop on a2 trapped by pawn on b3
                if sq == 8 && (enemy_pawns & bitboard::bit(17)) != 0 {
                    score += Self::BISHOP_TRAPPED_A7;
                }
                // Bishop on a3 blocked by pawn on b4
                if sq == 16 && (enemy_pawns & bitboard::bit(25)) != 0 {
                    score += Self::BISHOP_TRAPPED_A6;
                }
                // Bishop on h2 trapped by pawn on g3
                if sq == 15 && (enemy_pawns & bitboard::bit(22)) != 0 {
                    score += Self::BISHOP_TRAPPED_H7;
                }
                // Bishop on h3 blocked by pawn on g4
                if sq == 23 && (enemy_pawns & bitboard::bit(30)) != 0 {
                    score += Self::BISHOP_TRAPPED_H6;
                }

                // Bishop trapped behind own pawns
                if sq == 17 && (friendly_pawns & bitboard::bit(9)) != 0 &&
                    (enemy_pawns & bitboard::bit(8)) != 0 {
                    score += Self::TRAPPED_BISHOP_PENALTY / 2;
                }
                if sq == 22 && (friendly_pawns & bitboard::bit(14)) != 0 &&
                    (enemy_pawns & bitboard::bit(13)) != 0 {
                    score += Self::TRAPPED_BISHOP_PENALTY / 2;
                }
            }

            bp &= bp - 1;
        }

        score
    }

    fn evaluate_rooks(
        rooks: Bitboard,
        friendly_pawns: Bitboard,
        is_white: bool,
    ) -> i32 {
        let mut score = 0;
        let mut rk = rooks;

        while rk != 0 {
            let sq = rk.trailing_zeros() as u8;
            let file = sq % 8;
            let rank = sq / 8;

            // Rook trapped by own king (common endgame mistake)
            if is_white {
                // Rook on a1/h1 trapped by king on b1/g1
                if rank == 0 {
                    if sq == 0 && Self::has_blocking_pawn(1, 0, friendly_pawns) {
                        score += Self::TRAPPED_ROOK_PENALTY;
                    }
                    if sq == 7 && Self::has_blocking_pawn(6, 0, friendly_pawns) {
                        score += Self::TRAPPED_ROOK_PENALTY;
                    }
                }

                // Rook blocked by own pawns on the same file
                if Self::is_rook_blocked_by_pawns(sq, friendly_pawns, true) {
                    score += Self::BLOCKED_ROOK_PENALTY;
                }
            } else {
                // Black rook trapped
                if rank == 7 {
                    if sq == 56 && Self::has_blocking_pawn(1, 7, friendly_pawns) {
                        score += Self::TRAPPED_ROOK_PENALTY;
                    }
                    if sq == 63 && Self::has_blocking_pawn(6, 7, friendly_pawns) {
                        score += Self::TRAPPED_ROOK_PENALTY;
                    }
                }

                if Self::is_rook_blocked_by_pawns(sq, friendly_pawns, false) {
                    score += Self::BLOCKED_ROOK_PENALTY;
                }
            }

            rk &= rk - 1;
        }

        score
    }

    #[inline]
    fn has_blocking_pawn(file: u8, rank: u8, pawns: Bitboard) -> bool {
        pawns & bitboard::bit(rank * 8 + file) != 0
    }

    #[inline]
    fn is_rook_blocked_by_pawns(sq: u8, friendly_pawns: Bitboard, is_white: bool) -> bool {
        let file = sq % 8;
        let rank = sq / 8;
        let file_mask = 0x0101010101010101u64 << file;

        if is_white {
            // Check if there are friendly pawns in front of the rook
            let ahead_mask = file_mask & (0xFFFFFFFFFFFFFFFFu64 << ((rank + 1) * 8));
            friendly_pawns & ahead_mask != 0
        } else {
            // Check if there are friendly pawns in front of the rook
            let ahead_mask = file_mask & ((1u64 << (rank * 8)) - 1);
            friendly_pawns & ahead_mask != 0
        }
    }
}

// Mobility evaluator
pub struct MobilityEvaluator;

impl MobilityEvaluator {
    // Mobility weights per piece type
    const KNIGHT_MOBILITY_BONUS: i32 = 4;
    const BISHOP_MOBILITY_BONUS: i32 = 3;
    const ROOK_MOBILITY_BONUS: i32 = 2;
    const QUEEN_MOBILITY_BONUS: i32 = 1;

    // Safe mobility (squares not attacked by enemy pawns) is worth more
    const SAFE_MOBILITY_MULTIPLIER: i32 = 2;

    pub fn evaluate(board: &Board, movegen: &MoveGen) -> i32 {
        let mut score = 0;

        let white_occupied = board.white_occupancy();
        let black_occupied = board.black_occupancy();
        let occupied = white_occupied | black_occupied;

        // Calculate pawn attack maps for safe mobility
        let white_pawn_attacks = Self::pawn_attacks::<true>(board.white_pawns);
        let black_pawn_attacks = Self::pawn_attacks::<false>(board.black_pawns);

        // White mobility
        score += Self::knight_mobility(
            board.white_knights,
            white_occupied,
            black_pawn_attacks,
            movegen,
        );
        score += Self::bishop_mobility(
            board.white_bishops,
            white_occupied,
            occupied,
            black_pawn_attacks,
            movegen,
        );
        score += Self::rook_mobility(
            board.white_rooks,
            white_occupied,
            occupied,
            black_pawn_attacks,
            movegen,
        );
        score += Self::queen_mobility(
            board.white_queens,
            white_occupied,
            occupied,
            black_pawn_attacks,
            movegen,
        );

        // Black mobility
        score -= Self::knight_mobility(
            board.black_knights,
            black_occupied,
            white_pawn_attacks,
            movegen,
        );
        score -= Self::bishop_mobility(
            board.black_bishops,
            black_occupied,
            occupied,
            white_pawn_attacks,
            movegen,
        );
        score -= Self::rook_mobility(
            board.black_rooks,
            black_occupied,
            occupied,
            white_pawn_attacks,
            movegen,
        );
        score -= Self::queen_mobility(
            board.black_queens,
            black_occupied,
            occupied,
            white_pawn_attacks,
            movegen,
        );

        score
    }

    fn knight_mobility(
        knights: Bitboard,
        friendly_occupied: Bitboard,
        enemy_pawn_attacks: Bitboard,
        movegen: &MoveGen,
    ) -> i32 {
        let mut score = 0;
        let mut kn = knights;

        while kn != 0 {
            let sq = kn.trailing_zeros() as u8;
            let attacks = movegen.knight_attacks[sq as usize] & !friendly_occupied;

            let total_mobility = bitboard::popcount(attacks);
            let safe_mobility = bitboard::popcount(attacks & !enemy_pawn_attacks);

            score += total_mobility as i32 * Self::KNIGHT_MOBILITY_BONUS;
            score += safe_mobility as i32 * Self::SAFE_MOBILITY_MULTIPLIER;

            kn &= kn - 1;
        }

        score
    }

    fn bishop_mobility(
        bishops: Bitboard,
        friendly_occupied: Bitboard,
        occupied: Bitboard,
        enemy_pawn_attacks: Bitboard,
        movegen: &MoveGen,
    ) -> i32 {
        let mut score = 0;
        let mut bp = bishops;

        while bp != 0 {
            let sq = bp.trailing_zeros() as u8;
            let attacks = MoveGen::compute_bishop_attacks(sq, occupied) & !friendly_occupied;

            let total_mobility = bitboard::popcount(attacks);
            let safe_mobility = bitboard::popcount(attacks & !enemy_pawn_attacks);

            score += total_mobility as i32 * Self::BISHOP_MOBILITY_BONUS;
            score += safe_mobility as i32 * Self::SAFE_MOBILITY_MULTIPLIER;

            bp &= bp - 1;
        }

        score
    }

    fn rook_mobility(
        rooks: Bitboard,
        friendly_occupied: Bitboard,
        occupied: Bitboard,
        enemy_pawn_attacks: Bitboard,
        movegen: &MoveGen,
    ) -> i32 {
        let mut score = 0;
        let mut rk = rooks;

        while rk != 0 {
            let sq = rk.trailing_zeros() as u8;
            let attacks = MoveGen::compute_rook_attacks(sq, occupied) & !friendly_occupied;

            let total_mobility = bitboard::popcount(attacks);
            let safe_mobility = bitboard::popcount(attacks & !enemy_pawn_attacks);

            score += total_mobility as i32 * Self::ROOK_MOBILITY_BONUS;
            score += safe_mobility as i32 * Self::SAFE_MOBILITY_MULTIPLIER;

            rk &= rk - 1;
        }

        score
    }

    fn queen_mobility(
        queens: Bitboard,
        friendly_occupied: Bitboard,
        occupied: Bitboard,
        enemy_pawn_attacks: Bitboard,
        movegen: &MoveGen,
    ) -> i32 {
        let mut score = 0;
        let mut qn = queens;

        while qn != 0 {
            let sq = qn.trailing_zeros() as u8;
            let attacks = (MoveGen::compute_rook_attacks(sq, occupied)
                | MoveGen::compute_bishop_attacks(sq, occupied)) & !friendly_occupied;

            let total_mobility = bitboard::popcount(attacks);
            let safe_mobility = bitboard::popcount(attacks & !enemy_pawn_attacks);

            score += total_mobility as i32 * Self::QUEEN_MOBILITY_BONUS;
            score += safe_mobility as i32 * Self::SAFE_MOBILITY_MULTIPLIER;

            qn &= qn - 1;
        }

        score
    }

    fn pawn_attacks<const WHITE: bool>(pawns: Bitboard) -> Bitboard {
        if WHITE {
            ((pawns << 7) & !0x8080808080808080u64) |
                ((pawns << 9) & !0x0101010101010101u64)
        } else {
            ((pawns >> 7) & !0x0101010101010101u64) |
                ((pawns >> 9) & !0x8080808080808080u64)
        }
    }
}