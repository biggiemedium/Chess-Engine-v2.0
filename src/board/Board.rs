use crate::board::bitboard::Bitboard;

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
}