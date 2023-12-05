use std::ops::Index;
use crate::consts::{Word, Byte};

struct MemoryMock {
    data: [u8; 5]
}

impl Index<Word> for MemoryMock {
    type Output = Byte;

    fn index(&self, index: Word) -> &Self::Output {
        let addr: usize = index.into();
        return &self.data[addr];
    }
}

#[cfg(test)]
mod new {
    use super::super::*;

    #[test]
    fn should_be_in_reset_state_after_creation() {
        let uut = CPU::new();

        assert_eq!(uut.accumulator, 0);
        assert_eq!(uut.cycle, 0);
        assert_eq!(uut.index_register_x, 0);
        assert_eq!(uut.index_register_y, 0);
        assert_eq!(uut.stack_pointer, 0);
        assert_eq!(uut.processor_status.flags, 0);
        assert_eq!(uut.program_counter, 0xFFFC);
    }
}

#[cfg(test)]
mod reset {
    use super::super::*;

    #[test]
    fn should_set_program_counter_to_fffc_after_reset() {
        let mut uut = CPU::new();
        uut.program_counter = 0xFFFF;

        uut.reset();

        assert_eq!(uut.program_counter, 0xFFFC);
    }

    #[test]
    fn should_set_negative_flag_in_processor_status_to_zero_after_reset() {
        let mut uut = CPU::new();
        uut.processor_status.flags = 0b11111111;

        uut.reset();

        assert_eq!(uut.processor_status.flags, 0b11110111);
    }
}

#[cfg(test)]
mod access_memory {
    use crate::cpu::CPU;
    use crate::consts::Word;
    use super::MemoryMock;

    const MEMORY: MemoryMock = MemoryMock {
        data: [0x44, 0x51, 0x88, 0x42, 0x99]
    };
    const ADDR: Word = 0x0003;

    #[test]
    fn should_return_a_byte() {
        let mut uut = CPU::new();

        let result = uut.access_memory(ADDR, &MEMORY);
        
        assert_eq!(result, 0x42);
    }

    #[test]
    fn should_increase_cycle_counter() {
        let mut uut = CPU::new();
        assert_eq!(uut.cycle, 0);

        uut.access_memory(ADDR, &MEMORY);
        
        assert_eq!(uut.cycle, 1);
    }
}

#[cfg(test)]
mod fetch_byte {
    use crate::cpu::CPU;
    use super::MemoryMock;

    const MEMORY: MemoryMock = MemoryMock {
        data: [0x44, 0x51, 0x88, 0x42, 0x99]
    };

    #[test]
    fn should_return_a_byte_pointed_by_a_program_counter() {
        let mut uut = CPU::new();
        uut.program_counter = 0x0001;

        let result = uut.fetch_byte(&MEMORY);
        
        assert_eq!(result, 0x51);
    }

    #[test]
    fn should_increase_cycle_counter_and_a_program_counter() {
        let mut uut = CPU::new();
        uut.program_counter = 0x0001;

        assert_eq!(uut.cycle, 0);

        uut.fetch_byte(&MEMORY);
        
        assert_eq!(uut.cycle, 1);
        assert_eq!(uut.program_counter, 0x0002);
    }
}

#[cfg(test)]
mod fetch_word {
    use crate::cpu::CPU;
    use super::MemoryMock;

    const MEMORY: MemoryMock = MemoryMock {
        data: [0x44, 0x51, 0x88, 0x42, 0x99]
    };

    #[test]
    fn should_return_a_word_pointed_by_a_program_counter_in_little_endian() {
        let mut uut = CPU::new();
        uut.program_counter = 0x0001;

        let result = uut.fetch_word(&MEMORY);
        
        assert_eq!(result, 0x8851);
    }

    #[test]
    fn should_increase_cycle_counter_and_a_program_counter_twice() {
        let mut uut = CPU::new();
        uut.program_counter = 0x0001;

        assert_eq!(uut.cycle, 0);

        uut.fetch_word(&MEMORY);
        
        assert_eq!(uut.cycle, 2);
        assert_eq!(uut.program_counter, 0x0003);
    }
}

#[cfg(test)]
mod fetch_instruction {
    use crate::cpu::CPU;
    use super::MemoryMock;

    const MEMORY: MemoryMock = MemoryMock {
        data: [0x44, 0x51, 0x88, 0x42, 0x99]
    };

    #[test]
    fn should_return_an_instruction_pointed_by_a_program_counter() {
        let mut uut = CPU::new();
        uut.program_counter = 0x0001;

        let result = uut.fetch_instruction(&MEMORY);
        
        assert_eq!(result, 0x51);
    }

    #[test]
    fn should_increase_cycle_counter_and_a_program_counter() {
        let mut uut = CPU::new();
        uut.program_counter = 0x0001;

        assert_eq!(uut.cycle, 0);

        uut.fetch_instruction(&MEMORY);
        
        assert_eq!(uut.cycle, 1);
        assert_eq!(uut.program_counter, 0x0002);
    }
}