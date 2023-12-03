use crate::consts::Word;

use super::consts::Byte;
use std::ops::{Index, IndexMut};


const MEMORY_SIZE_KB: usize = 64;
const MEMORY_SIZE: usize = 1024 * MEMORY_SIZE_KB;

pub struct Memory {
    pub data: [Byte; MEMORY_SIZE],
}

impl Memory {}

impl Index<usize> for Memory {
    type Output = Byte;

    fn index(&self, idx: usize) -> &Self::Output {
        return &self.data[idx];
    }
}

impl Index<Word> for Memory {
    type Output = Byte;

    fn index(&self, idx: Word) -> &Self::Output {
        let mem_address: usize = idx.into();
        return &self.data[mem_address];
    }
}

impl IndexMut<Word> for Memory {
    fn index_mut(&mut self, idx: Word) -> &mut Self::Output {
        let mem_address: usize = idx.into();
        return &mut self.data[mem_address];
    }
}

impl Index<Byte> for Memory {
    type Output = Byte;

    fn index(&self, idx: Byte) -> &Self::Output {
        let mem_address: usize = idx.into();
        return &self.data[mem_address];
    }
}

impl From<&[Byte; MEMORY_SIZE]> for Memory {
    fn from(value: &[Byte; MEMORY_SIZE]) -> Self {
        return Memory { data: *value };
    }
}
