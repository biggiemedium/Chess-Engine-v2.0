use crate::board::bitboard::{self, Bitboard};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveFlags(u8);

impl MoveFlags {
    pub const NORMAL: MoveFlags = MoveFlags(0);
    pub const CAPTURE: MoveFlags = MoveFlags(1);
    pub const DOUBLE_PAWN_PUSH: MoveFlags = MoveFlags(2);
    pub const EN_PASSANT: MoveFlags = MoveFlags(3);
    pub const CASTLE_KING: MoveFlags = MoveFlags(4);
    pub const CASTLE_QUEEN: MoveFlags = MoveFlags(5);
    pub const PROMOTION_KNIGHT: MoveFlags = MoveFlags(6);
    pub const PROMOTION_BISHOP: MoveFlags = MoveFlags(7);
    pub const PROMOTION_ROOK: MoveFlags = MoveFlags(8);
    pub const PROMOTION_QUEEN: MoveFlags = MoveFlags(9);
    pub const PROMOTION_CAPTURE_KNIGHT: MoveFlags = MoveFlags(10);
    pub const PROMOTION_CAPTURE_BISHOP: MoveFlags = MoveFlags(11);
    pub const PROMOTION_CAPTURE_ROOK: MoveFlags = MoveFlags(12);
    pub const PROMOTION_CAPTURE_QUEEN: MoveFlags = MoveFlags(13);

    #[inline]
    pub const fn new(value: u8) -> Self {
        MoveFlags(value)
    }

    #[inline]
    pub const fn value(&self) -> u8 {
        self.0
    }

    #[inline]
    pub const fn is_promotion_knight(&self) -> bool {
        self.0 == Self::PROMOTION_KNIGHT.0 || self.0 == Self::PROMOTION_CAPTURE_KNIGHT.0
    }

    #[inline]
    pub const fn is_promotion_bishop(&self) -> bool {
        self.0 == Self::PROMOTION_BISHOP.0 || self.0 == Self::PROMOTION_CAPTURE_BISHOP.0
    }

    #[inline]
    pub const fn is_promotion_rook(&self) -> bool {
        self.0 == Self::PROMOTION_ROOK.0 || self.0 == Self::PROMOTION_CAPTURE_ROOK.0
    }

    #[inline]
    pub const fn is_promotion_queen(&self) -> bool {
        self.0 == Self::PROMOTION_QUEEN.0 || self.0 == Self::PROMOTION_CAPTURE_QUEEN.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub flags: MoveFlags,
}

impl Move {
    #[inline]
    pub fn new(from: u8, to: u8, flags: MoveFlags) -> Self {
        Self { from, to, flags }
    }

    #[inline]
    pub fn is_capture(&self) -> bool {
        self.flags.0 == MoveFlags::CAPTURE.0
            || (self.flags.0 >= MoveFlags::PROMOTION_CAPTURE_KNIGHT.0
            && self.flags.0 <= MoveFlags::PROMOTION_CAPTURE_QUEEN.0)
            || self.flags.0 == MoveFlags::EN_PASSANT.0
    }

    #[inline]
    pub fn is_promotion(&self) -> bool {
        self.flags.0 >= MoveFlags::PROMOTION_KNIGHT.0
    }
}