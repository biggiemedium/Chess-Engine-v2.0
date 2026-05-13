pub type Bitboard = u64;

// https://doc.rust-lang.org/book/appendix-02-operators.html
pub mod bitboard {

    use super::Bitboard;

    pub const EMPTY: Bitboard = 0;

    // (file, index) -> square position
    #[inline]
    pub fn square(file: u8, rank: u8) -> u8 {
        // move up (rank) rows -> add by file amount
        rank * 8 + file
    }

    #[inline]
    pub fn bit(square: u8) -> Bitboard {
        1u64 << square // expr << expr -> Left-shift
    }

    // set a bit
    #[inline]
    pub fn set(bb: Bitboard, sq: u8) -> Bitboard {
        // pat | pat -> Pattern alternatives
        // expr | expr -> Bitwise OR

        bb | bit(sq) // Bitwise OR
    }

    #[inline]
    pub fn unset(bb: Bitboard, sq: u8) -> Bitboard {
        bb & !bit(sq)
    }

    // Count set bits
    #[inline]
    pub fn popcount(bb: Bitboard) -> u32 {
        bb.count_ones()
    }


}