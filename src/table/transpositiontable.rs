use crate::movegen::r#move::Move;
use std::collections::LinkedList;

#[derive(Clone, Copy)]
pub struct TranspositionTableEntry { // fuck saftey
    pub zobristHash: u64,
    pub score: i32,
    pub depth: u8,
    pub flag: u8, // 0=EXACT, 1=LOWER_BOUND(alpha), 2=UPPER_BOUND(beta)
    pub bestMove: Move,
    pub age: u8,
}

impl TranspositionTableEntry {

    pub const EXACT: u8 = 0;
    pub const LOWER_BOUND: u8 = 1;
    pub const UPPER_BOUND: u8 = 2;

    pub fn new(
        zobristHash: u64,
        score: i32,
        depth: u8,
        flag: u8,
        bestMove: Move,
        age: u8,
    ) -> Self {
        Self {
            zobristHash,
            score,
            depth,
            flag,
            bestMove,
            age,
        }
    }
}

pub struct TranspositionTable {
    table: Vec<Option<TranspositionTableEntry>>,
    size: usize,
}

impl TranspositionTable {

    pub const EXACT: u8 = 0;
    pub const LOWER: u8 = 1;
    pub const UPPER: u8 = 2;

    pub fn new(size: usize) -> Self {
        Self {
            table: vec![None; size],
            size,
        }
    }

    #[inline]
    pub fn probe(&self, zobristHash: u64)
        -> Option<&TranspositionTableEntry> {

        let index = (zobristHash as usize) % self.size;

        match &self.table[index] {
            Some(entry) if entry.zobristHash == zobristHash => {
                Some(entry)
            }
            _ => None,
        }
    }

    #[inline]
    pub fn store(
        &mut self,
        zobristHash: u64,
        score: i32,
        depth: u8,
        flag: u8,
        bestMove: Move,
        age: u8,
    ) {

        let index = (zobristHash as usize) % self.size;

        let replace = match self.table[index] {
            Some(existing) => depth >= existing.depth,
            None => true,
        };

        if replace {
            self.table[index] = Some(
                TranspositionTableEntry::new(
                    zobristHash,
                    score,
                    depth,
                    flag,
                    bestMove,
                    age,
                )
            );
        }

    }

    #[inline]
    pub fn clear(&mut self) {
        self.table.fill(None);
    }

}