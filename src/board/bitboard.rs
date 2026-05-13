
// https://doc.rust-lang.org/book/appendix-02-operators.html
pub type Bitboard = u64;

pub const EMPTY: Bitboard = 0;

#[inline]
pub fn square(file: u8, rank: u8) -> u8 {
    rank * 8 + file
}

#[inline]
pub fn bit(square: u8) -> Bitboard {
    1u64 << square
}

#[inline]
pub fn set(bb: Bitboard, sq: u8) -> Bitboard {
    bb | bit(sq)
}

#[inline]
pub fn unset(bb: Bitboard, sq: u8) -> Bitboard {
    bb & !bit(sq)
}

#[inline]
pub fn popcount(bb: Bitboard) -> u32 {
    bb.count_ones()
}