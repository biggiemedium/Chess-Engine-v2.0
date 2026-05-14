use crate::board::bitboard::{self, Bitboard};
use crate::board::board::Board;
use crate::movegen::r#move::{Move, MoveFlags};

pub struct MoveGen {
    // Pre-computed attack tables for sliding pieces
    knight_attacks: [Bitboard; 64],
    king_attacks: [Bitboard; 64],
}

impl MoveGen {
    pub fn new() -> Self {
        let mut movegen = Self {
            knight_attacks: [0; 64],
            king_attacks: [0; 64],
        };
        movegen.init_tables();
        movegen
    }

    fn init_tables(&mut self) {
        for sq in 0..64 {
            self.knight_attacks[sq as usize] = Self::compute_knight_attacks(sq);
            self.king_attacks[sq as usize] = Self::compute_king_attacks(sq);
        }
    }

    #[inline]
    fn compute_knight_attacks(sq: u8) -> Bitboard {
        let bb = bitboard::bit(sq);
        let mut attacks = 0u64;

        // Knight moves: 2 squares in one direction, 1 in perpendicular
        attacks |= (bb << 17) & !0x0101010101010101u64; // up 2, right 1
        attacks |= (bb << 15) & !0x8080808080808080u64; // up 2, left 1
        attacks |= (bb << 10) & !0x0303030303030303u64; // up 1, right 2
        attacks |= (bb << 6) & !0xC0C0C0C0C0C0C0C0u64;  // up 1, left 2
        attacks |= (bb >> 17) & !0x8080808080808080u64; // down 2, left 1
        attacks |= (bb >> 15) & !0x0101010101010101u64; // down 2, right 1
        attacks |= (bb >> 10) & !0xC0C0C0C0C0C0C0C0u64; // down 1, left 2
        attacks |= (bb >> 6) & !0x0303030303030303u64;  // down 1, right 2

        attacks
    }

    #[inline]
    fn compute_king_attacks(sq: u8) -> Bitboard {
        let bb = bitboard::bit(sq);
        let mut attacks = 0u64;

        attacks |= (bb << 8);                           // up
        attacks |= (bb >> 8);                           // down
        attacks |= (bb << 1) & !0x0101010101010101u64;  // right
        attacks |= (bb >> 1) & !0x8080808080808080u64;  // left
        attacks |= (bb << 9) & !0x0101010101010101u64;  // up-right
        attacks |= (bb << 7) & !0x8080808080808080u64;  // up-left
        attacks |= (bb >> 9) & !0x8080808080808080u64;  // down-left
        attacks |= (bb >> 7) & !0x0101010101010101u64;  // down-right

        attacks
    }

    #[inline]
    fn compute_rook_attacks(sq: u8, occupied: Bitboard) -> Bitboard {
        let mut attacks = 0u64;
        let file = sq % 8;
        let rank = sq / 8;

        // North
        for r in (rank + 1)..8 {
            let target = r * 8 + file;
            attacks |= bitboard::bit(target);
            if occupied & bitboard::bit(target) != 0 { break; }
        }
        // South
        for r in (0..rank).rev() {
            let target = r * 8 + file;
            attacks |= bitboard::bit(target);
            if occupied & bitboard::bit(target) != 0 { break; }
        }
        // East
        for f in (file + 1)..8 {
            let target = rank * 8 + f;
            attacks |= bitboard::bit(target);
            if occupied & bitboard::bit(target) != 0 { break; }
        }
        // West
        for f in (0..file).rev() {
            let target = rank * 8 + f;
            attacks |= bitboard::bit(target);
            if occupied & bitboard::bit(target) != 0 { break; }
        }

        attacks
    }

    #[inline]
    pub fn compute_bishop_attacks(sq: u8, occupied: Bitboard) -> Bitboard {
        let mut attacks = 0u64;
        let file = sq % 8;
        let rank = sq / 8;

        // North east
        let mut r = rank + 1;
        let mut f = file + 1;
        while r < 8 && f < 8 {
            let target = r * 8 + f;
            attacks |= bitboard::bit(target);
            if occupied & bitboard::bit(target) != 0 {
                break;
            }
            r += 1;
            f += 1;
        }

        // North west
        r = rank + 1;
        f = file.wrapping_sub(1); // = file - 1
        while r < 8 && f < 8 {
            let target = r * 8 + f;
            attacks |= bitboard::bit(target);
            if occupied & bitboard::bit(target) != 0 {
                break;
            }
            r += 1;
            f = f.wrapping_sub(1);
        }

        // South east
        r = rank.wrapping_sub(1);
        f = file + 1;
        while r < 8 && f < 8 {
            let target = r * 8 + f;
            attacks |= bitboard::bit(target);
            if occupied & bitboard::bit(target) != 0 {
                break;
            }
            r = r.wrapping_sub(1);
            f += 1;
        }

        // south west
        r = rank.wrapping_sub(1);
        f = file.wrapping_sub(1);
        while r < 8 && f < 8 {
            let target = r * 8 + f;
            attacks |= bitboard::bit(target);
            if occupied & bitboard::bit(target) != 0 {
                break;
            }
            r = r.wrapping_sub(1);
            f = f.wrapping_sub(1);
        }
        r = rank.wrapping_sub(1);
        attacks
    }

    pub fn generate_capture_moves(&self, board: &Board, white_to_move: bool) -> Vec<Move> {
        let mut moves = Vec::new();

        let (friendly, enemy) = if white_to_move {
            (board.white_occupancy(), board.black_occupancy())
        } else {
            (board.black_occupancy(), board.white_occupancy())
        };

        let occupied = friendly | enemy;

        if white_to_move {
            self.generate_pawn_captures::<true>(board, enemy, &mut moves);
            self.generate_knight_captures(board.white_knights, friendly, enemy, &mut moves);
            self.generate_king_captures(board.white_king, friendly, enemy, &mut moves);
            self.generate_rook_captures(board.white_rooks, occupied, friendly, enemy, &mut moves);
            self.generate_bishop_captures(board.white_bishops, occupied, friendly, enemy, &mut moves);
            self.generate_queen_captures(board.white_queens, occupied, friendly, enemy, &mut moves);
        } else {
            self.generate_pawn_captures::<false>(board, enemy, &mut moves);
            self.generate_knight_captures(board.black_knights, friendly, enemy, &mut moves);
            self.generate_king_captures(board.black_king, friendly, enemy, &mut moves);
            self.generate_rook_captures(board.black_rooks, occupied, friendly, enemy, &mut moves);
            self.generate_bishop_captures(board.black_bishops, occupied, friendly, enemy, &mut moves);
            self.generate_queen_captures(board.black_queens, occupied, friendly, enemy, &mut moves);
        }

        moves
    }

    #[inline]
    pub fn is_king_in_check(&self, board: &Board, white_king: bool) -> bool {
        let (king_bb, enemy_knights, enemy_bishops, enemy_rooks, enemy_queens, enemy_pawns, enemy_king) =
            if white_king {
                (board.white_king, board.black_knights, board.black_bishops,
                 board.black_rooks, board.black_queens, board.black_pawns, board.black_king)
            } else {
                (board.black_king, board.white_knights, board.white_bishops,
                 board.white_rooks, board.white_queens, board.white_pawns, board.white_king)
            };

        if king_bb == 0 { return false; }
        let king_sq = king_bb.trailing_zeros() as u8;
        let occupied = board.white_occupancy() | board.black_occupancy();

        // Check for enemy knights
        if self.knight_attacks[king_sq as usize] & enemy_knights != 0 { return true; }

        // Check for enemy king (adjacent squares)
        if self.king_attacks[king_sq as usize] & enemy_king != 0 { return true; }

        // Check for enemy bishops/queens on diagonals
        let bishop_attacks = Self::compute_bishop_attacks(king_sq, occupied);
        if bishop_attacks & (enemy_bishops | enemy_queens) != 0 { return true; }

        // Check for enemy rooks/queens on ranks/files
        let rook_attacks = Self::compute_rook_attacks(king_sq, occupied);
        if rook_attacks & (enemy_rooks | enemy_queens) != 0 { return true; }

        // Check for enemy pawns
        let pawn_attacks = if white_king {
            ((bitboard::bit(king_sq) << 7) & !0x8080808080808080u64) |
                ((bitboard::bit(king_sq) << 9) & !0x0101010101010101u64)
        } else {
            ((bitboard::bit(king_sq) >> 7) & !0x0101010101010101u64) |
                ((bitboard::bit(king_sq) >> 9) & !0x8080808080808080u64)
        };
        pawn_attacks & enemy_pawns != 0
    }

    #[inline]
    fn generate_pawn_captures<const WHITE: bool>(
        &self,
        board: &Board,
        enemy: Bitboard,
        moves: &mut Vec<Move>,
    ) {
        let pawns = if WHITE { board.white_pawns } else { board.black_pawns };

        if WHITE {
            let left_captures = ((pawns << 7) & !0x8080808080808080u64) & enemy;
            let right_captures = ((pawns << 9) & !0x0101010101010101u64) & enemy;

            let promotion_rank = 0xFF00000000000000;
            self.serialize_promotions::<true>(pawns, left_captures & promotion_rank, 7, true, moves);
            self.serialize_promotions::<true>(pawns, right_captures & promotion_rank, 9, true, moves);
            self.serialize_moves(pawns, left_captures & !promotion_rank, 7, MoveFlags::CAPTURE, moves);
            self.serialize_moves(pawns, right_captures & !promotion_rank, 9, MoveFlags::CAPTURE, moves);
        } else {
            let left_captures = ((pawns >> 9) & !0x8080808080808080u64) & enemy;
            let right_captures = ((pawns >> 7) & !0x0101010101010101u64) & enemy;

            let promotion_rank = 0x00000000000000FF;
            self.serialize_promotions::<false>(pawns, left_captures & promotion_rank, 9, true, moves);
            self.serialize_promotions::<false>(pawns, right_captures & promotion_rank, 7, true, moves);
            self.serialize_moves_backward(pawns, left_captures & !promotion_rank, 9, MoveFlags::CAPTURE, moves);
            self.serialize_moves_backward(pawns, right_captures & !promotion_rank, 7, MoveFlags::CAPTURE, moves);
        }
    }

    #[inline]
    fn generate_knight_captures(
        &self,
        knights: Bitboard,
        friendly: Bitboard,
        enemy: Bitboard,
        moves: &mut Vec<Move>,
    ) {
        let mut k = knights;
        while k != 0 {
            let from = k.trailing_zeros() as u8;
            let captures = self.knight_attacks[from as usize] & enemy;
            self.serialize_moves_from_square(from, captures, MoveFlags::CAPTURE, moves);
            k &= k - 1;
        }
    }

    #[inline]
    fn generate_king_captures(
        &self,
        king: Bitboard,
        friendly: Bitboard,
        enemy: Bitboard,
        moves: &mut Vec<Move>,
    ) {
        if king == 0 { return; }
        let from = king.trailing_zeros() as u8;
        let captures = self.king_attacks[from as usize] & enemy;
        self.serialize_moves_from_square(from, captures, MoveFlags::CAPTURE, moves);
    }

    #[inline]
    fn generate_rook_captures(
        &self,
        rooks: Bitboard,
        occupied: Bitboard,
        friendly: Bitboard,
        enemy: Bitboard,
        moves: &mut Vec<Move>,
    ) {
        let mut r = rooks;
        while r != 0 {
            let from = r.trailing_zeros() as u8;
            let captures = Self::compute_rook_attacks(from, occupied) & enemy;
            self.serialize_moves_from_square(from, captures, MoveFlags::CAPTURE, moves);
            r &= r - 1;
        }
    }

    #[inline]
    fn generate_bishop_captures(
        &self,
        bishops: Bitboard,
        occupied: Bitboard,
        friendly: Bitboard,
        enemy: Bitboard,
        moves: &mut Vec<Move>,
    ) {
        let mut b = bishops;
        while b != 0 {
            let from = b.trailing_zeros() as u8;
            let captures = Self::compute_bishop_attacks(from, occupied) & enemy;
            self.serialize_moves_from_square(from, captures, MoveFlags::CAPTURE, moves);
            b &= b - 1;
        }
    }

    #[inline]
    fn generate_queen_captures(
        &self,
        queens: Bitboard,
        occupied: Bitboard,
        friendly: Bitboard,
        enemy: Bitboard,
        moves: &mut Vec<Move>,
    ) {
        let mut q = queens;
        while q != 0 {
            let from = q.trailing_zeros() as u8;
            let captures = (Self::compute_rook_attacks(from, occupied)
                | Self::compute_bishop_attacks(from, occupied)) & enemy;
            self.serialize_moves_from_square(from, captures, MoveFlags::CAPTURE, moves);
            q &= q - 1;
        }
    }

    pub fn generate_moves(&self, board: &Board, white_to_move: bool, moves: &mut Vec<Move>) {
        moves.clear();

        let (friendly, enemy) = if white_to_move {
            (board.white_occupancy(), board.black_occupancy())
        } else {
            (board.black_occupancy(), board.white_occupancy())
        };

        let occupied = friendly | enemy;

        if white_to_move {
            self.generate_pawn_moves::<true>(board, friendly, enemy, moves);
            self.generate_knight_moves(board.white_knights, friendly, enemy, moves);
            self.generate_king_moves(board.white_king, friendly, enemy, moves);
            self.generate_rook_moves(board.white_rooks, occupied, friendly, enemy, moves);
            self.generate_bishop_moves(board.white_bishops, occupied, friendly, enemy, moves);
            self.generate_queen_moves(board.white_queens, occupied, friendly, enemy, moves);
        } else {
            self.generate_pawn_moves::<false>(board, friendly, enemy, moves);
            self.generate_knight_moves(board.black_knights, friendly, enemy, moves);
            self.generate_king_moves(board.black_king, friendly, enemy, moves);
            self.generate_rook_moves(board.black_rooks, occupied, friendly, enemy, moves);
            self.generate_bishop_moves(board.black_bishops, occupied, friendly, enemy, moves);
            self.generate_queen_moves(board.black_queens, occupied, friendly, enemy, moves);
        }
    }

    #[inline]
    fn generate_pawn_moves<const WHITE: bool>(
        &self,
        board: &Board,
        friendly: Bitboard,
        enemy: Bitboard,
        moves: &mut Vec<Move>,
    ) {
        let pawns = if WHITE { board.white_pawns } else { board.black_pawns };
        let occupied = friendly | enemy;

        if WHITE {
            // Single push
            let push_targets = (pawns << 8) & !occupied;
            self.add_pawn_pushes::<WHITE>(pawns, push_targets, 8, moves);

            // Double push
            let double_push_targets = ((push_targets & 0x0000000000FF0000) << 8) & !occupied;
            self.serialize_moves(pawns & 0x000000000000FF00, double_push_targets, 16, MoveFlags::DOUBLE_PAWN_PUSH, moves);

            // Captures
            let left_captures = ((pawns << 7) & !0x8080808080808080u64) & enemy;
            let right_captures = ((pawns << 9) & !0x0101010101010101u64) & enemy;
            self.add_pawn_captures::<WHITE>(pawns, left_captures, 7, moves);
            self.add_pawn_captures::<WHITE>(pawns, right_captures, 9, moves);
        } else {
            // Single push
            let push_targets = (pawns >> 8) & !occupied;
            self.add_pawn_pushes::<WHITE>(pawns, push_targets, 8, moves);

            // Double push
            let double_push_targets = ((push_targets & 0x0000FF0000000000) >> 8) & !occupied;
            self.serialize_moves(pawns & 0x00FF000000000000, double_push_targets, 16, MoveFlags::DOUBLE_PAWN_PUSH, moves);

            // Captures
            let left_captures = ((pawns >> 9) & !0x8080808080808080u64) & enemy;
            let right_captures = ((pawns >> 7) & !0x0101010101010101u64) & enemy;
            self.add_pawn_captures::<WHITE>(pawns, left_captures, 9, moves);
            self.add_pawn_captures::<WHITE>(pawns, right_captures, 7, moves);
        }
    }

    #[inline]
    fn add_pawn_pushes<const WHITE: bool>(
        &self,
        pawns: Bitboard,
        targets: Bitboard,
        shift: u8,
        moves: &mut Vec<Move>,
    ) {
        let promotion_rank = if WHITE { 0xFF00000000000000 } else { 0x00000000000000FF };
        let promotions = targets & promotion_rank;
        let normal = targets & !promotion_rank;

        if WHITE {
            self.serialize_promotions::<true>(pawns, promotions, shift, false, moves);
            self.serialize_moves(pawns, normal, shift, MoveFlags::NORMAL, moves);
        } else {
            self.serialize_promotions::<false>(pawns, promotions, shift, false, moves);
            self.serialize_moves_backward(pawns, normal, shift, MoveFlags::NORMAL, moves);
        }
    }

    #[inline]
    fn add_pawn_captures<const WHITE: bool>(
        &self,
        pawns: Bitboard,
        targets: Bitboard,
        shift: u8,
        moves: &mut Vec<Move>,
    ) {
        let promotion_rank = if WHITE { 0xFF00000000000000 } else { 0x00000000000000FF };
        let promotions = targets & promotion_rank;
        let normal = targets & !promotion_rank;

        if WHITE {
            self.serialize_promotions::<true>(pawns, promotions, shift, true, moves);
            self.serialize_moves(pawns, normal, shift, MoveFlags::CAPTURE, moves);
        } else {
            self.serialize_promotions::<true>(pawns, promotions, shift, true, moves);
            self.serialize_moves_backward(pawns, normal, shift, MoveFlags::CAPTURE, moves);
        }
    }

    #[inline]
    fn serialize_promotions<const WHITE: bool>(
        &self,
        sources: Bitboard,
        targets: Bitboard,
        shift: u8,
        is_capture: bool,
        moves: &mut Vec<Move>,
    ) {
        let base_flag = if is_capture {
            MoveFlags::PROMOTION_CAPTURE_KNIGHT.value()
        } else {
            MoveFlags::PROMOTION_KNIGHT.value()
        };

        let mut tgt = targets;
        while tgt != 0 {
            let to = tgt.trailing_zeros() as u8;

            let from = if WHITE {
                to - shift
            } else {
                to + shift
            };

            if sources & bitboard::bit(from) != 0 {
                for i in 0..4 {
                    moves.push(Move::new(from, to, MoveFlags::new(base_flag + i)));
                }
            }
            tgt &= tgt - 1;
        }
    }

    #[inline]
    fn generate_knight_moves(
        &self,
        knights: Bitboard,
        friendly: Bitboard,
        enemy: Bitboard,
        moves: &mut Vec<Move>,
    ) {
        let mut k = knights;
        while k != 0 {
            let from = k.trailing_zeros() as u8;
            let attacks = self.knight_attacks[from as usize] & !friendly;
            let captures = attacks & enemy;
            let quiet = attacks & !enemy;

            self.serialize_moves_from_square(from, captures, MoveFlags::CAPTURE, moves);
            self.serialize_moves_from_square(from, quiet, MoveFlags::NORMAL, moves);

            k &= k - 1;
        }
    }

    #[inline]
    fn generate_king_moves(
        &self,
        king: Bitboard,
        friendly: Bitboard,
        enemy: Bitboard,
        moves: &mut Vec<Move>,
    ) {
        if king == 0 { return; }
        let from = king.trailing_zeros() as u8;
        let attacks = self.king_attacks[from as usize] & !friendly;
        let captures = attacks & enemy;
        let quiet = attacks & !enemy;

        self.serialize_moves_from_square(from, captures, MoveFlags::CAPTURE, moves);
        self.serialize_moves_from_square(from, quiet, MoveFlags::NORMAL, moves);
    }

    #[inline]
    fn generate_rook_moves(
        &self,
        rooks: Bitboard,
        occupied: Bitboard,
        friendly: Bitboard,
        enemy: Bitboard,
        moves: &mut Vec<Move>,
    ) {
        let mut r = rooks;
        while r != 0 {
            let from = r.trailing_zeros() as u8;
            let attacks = Self::compute_rook_attacks(from, occupied) & !friendly;
            let captures = attacks & enemy;
            let quiet = attacks & !enemy;

            self.serialize_moves_from_square(from, captures, MoveFlags::CAPTURE, moves);
            self.serialize_moves_from_square(from, quiet, MoveFlags::NORMAL, moves);

            r &= r - 1;
        }
    }

    #[inline]
    fn generate_bishop_moves(
        &self,
        bishops: Bitboard,
        occupied: Bitboard,
        friendly: Bitboard,
        enemy: Bitboard,
        moves: &mut Vec<Move>,
    ) {
        let mut b = bishops;
        while b != 0 {
            let from = b.trailing_zeros() as u8;
            let attacks = Self::compute_bishop_attacks(from, occupied) & !friendly;
            let captures = attacks & enemy;
            let quiet = attacks & !enemy;

            self.serialize_moves_from_square(from, captures, MoveFlags::CAPTURE, moves);
            self.serialize_moves_from_square(from, quiet, MoveFlags::NORMAL, moves);

            b &= b - 1;
        }
    }

    #[inline]
    fn generate_queen_moves(
        &self,
        queens: Bitboard,
        occupied: Bitboard,
        friendly: Bitboard,
        enemy: Bitboard,
        moves: &mut Vec<Move>,
    ) {
        let mut q = queens;
        while q != 0 {
            let from = q.trailing_zeros() as u8;
            let attacks = (Self::compute_rook_attacks(from, occupied)
                | Self::compute_bishop_attacks(from, occupied)) & !friendly;
            let captures = attacks & enemy;
            let quiet = attacks & !enemy;

            self.serialize_moves_from_square(from, captures, MoveFlags::CAPTURE, moves);
            self.serialize_moves_from_square(from, quiet, MoveFlags::NORMAL, moves);

            q &= q - 1;
        }
    }

    #[inline]
    fn serialize_moves_from_square(
        &self,
        from: u8,
        targets: Bitboard,
        flags: MoveFlags,
        moves: &mut Vec<Move>,
    ) {
        let mut tgt = targets;
        while tgt != 0 {
            let to = tgt.trailing_zeros() as u8;
            moves.push(Move::new(from, to, flags));
            tgt &= tgt - 1;
        }
    }

    #[inline]
    fn serialize_moves(
        &self,
        sources: Bitboard,
        targets: Bitboard,
        shift: u8,
        flags: MoveFlags,
        moves: &mut Vec<Move>,
    ) {
        let mut tgt = targets;
        while tgt != 0 {
            let to = tgt.trailing_zeros() as u8;
            let from = to - shift;
            if sources & bitboard::bit(from) != 0 {
                moves.push(Move::new(from, to, flags));
            }
            tgt &= tgt - 1;
        }
    }

    #[inline]
    fn serialize_moves_backward(
        &self,
        sources: Bitboard,
        targets: Bitboard,
        shift: u8,
        flags: MoveFlags,
        moves: &mut Vec<Move>,
    ) {
        let mut tgt = targets;
        while tgt != 0 {
            let to = tgt.trailing_zeros() as u8;
            let from = to + shift;
            if sources & bitboard::bit(from) != 0 {
                moves.push(Move::new(from, to, flags));
            }
            tgt &= tgt - 1;
        }
    }

}

