use crate::{
    consts::{Byte, Word},
    memory::Memory,
};
use std::ops::{Index, IndexMut};

pub struct MemoryMock {
    data: [u8; 512],
}
impl Memory for MemoryMock {}

impl MemoryMock {
    pub fn new(payload: &[u8]) -> Self {
        let mut mock = MemoryMock { data: [0; 512] };
        mock.data[..payload.len()].copy_from_slice(payload);

        return mock;
    }
}

impl Default for MemoryMock {
    fn default() -> Self {
        const DATA: [u8; 5] = [0x44, 0x51, 0x88, 0x42, 0x99];
        return MemoryMock::new(&DATA);
    }
}

impl Index<Word> for MemoryMock {
    type Output = Byte;

    fn index(&self, index: Word) -> &Self::Output {
        let addr: usize = index.into();
        return &self.data[addr];
    }
}

impl IndexMut<Word> for MemoryMock {
    fn index_mut(&mut self, index: Word) -> &mut Self::Output {
        let addr: usize = index.into();
        return &mut self.data[addr];
    }
}

#[cfg(test)]
mod new {
    use super::super::*;
    use super::MemoryMock;

    #[test]
    fn should_be_in_reset_state_after_creation() {
        let uut = CPU::new(Box::new(MemoryMock::default()));

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
    use super::MemoryMock;

    #[test]
    fn should_set_program_counter_to_fffc_after_reset() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.program_counter = 0xFFFF;

        uut.reset();

        assert_eq!(uut.program_counter, 0xFFFC);
    }

    #[test]
    fn should_set_negative_flag_in_processor_status_to_zero_after_reset() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b11111111;

        uut.reset();

        assert_eq!(uut.processor_status.flags, 0b11110111);
    }
}

#[cfg(test)]
mod access_memory {
    use super::MemoryMock;
    use crate::consts::Word;
    use crate::cpu::CPU;

    const ADDR: Word = 0x0003;

    #[test]
    fn should_return_a_byte() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));

        let result = uut.access_memory(ADDR);

        assert_eq!(result, 0x42);
    }
}

#[cfg(test)]
mod fetch_byte {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_return_a_byte_pointed_by_a_program_counter() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.program_counter = 0x0001;

        let result = uut.fetch_byte();

        assert_eq!(result, 0x51);
    }

    #[test]
    fn should_increase_cycle_counter_and_a_program_counter() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.program_counter = 0x0001;

        assert_eq!(uut.cycle, 0);

        uut.fetch_byte();

        assert_eq!(uut.cycle, 1);
        assert_eq!(uut.program_counter, 0x0002);
    }
}

#[cfg(test)]
mod fetch_word {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_return_a_word_pointed_by_a_program_counter_in_little_endian() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.program_counter = 0x0001;

        let result = uut.fetch_word();

        assert_eq!(result, 0x8851);
    }

    #[test]
    fn should_increase_cycle_counter_and_a_program_counter_twice() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.program_counter = 0x0001;

        assert_eq!(uut.cycle, 0);

        uut.fetch_word();

        assert_eq!(uut.cycle, 2);
        assert_eq!(uut.program_counter, 0x0003);
    }
}

#[cfg(test)]
mod fetch_address {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_return_an_address_pointed_by_a_program_counter_in_little_endian() {
        let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
        uut.program_counter = 0x00;

        let result = uut.fetch_address();

        assert_eq!(result, 0xFF03);
    }

    #[test]
    fn should_increase_cycle_counter_and_a_program_counter_twice() {
        let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
        uut.program_counter = 0x00;

        assert_eq!(uut.cycle, 0);

        uut.fetch_address();

        assert_eq!(uut.cycle, 2);
        assert_eq!(uut.program_counter, 0x0002);
    }
}

#[cfg(test)]
mod fetch_zero_page_address {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_return_a_zero_page_address_pointed_by_a_program_counter_in_little_endian() {
        let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
        uut.program_counter = 0x00;

        let result = uut.fetch_zero_page_address();

        assert_eq!(result, 0x003);
    }

    #[test]
    fn should_increase_cycle_counter_and_a_program_counter_once() {
        let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
        uut.program_counter = 0x00;

        assert_eq!(uut.cycle, 0);

        uut.fetch_zero_page_address();

        assert_eq!(uut.cycle, 1);
        assert_eq!(uut.program_counter, 0x0001);
    }
}

#[cfg(test)]
mod fetch_zero_page_address_with_x_offset {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_return_a_zero_page_address_pointed_by_a_program_counter_summed_with_index_register_x()
    {
        let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
        uut.index_register_x = 0x20;
        uut.program_counter = 0x00;

        let result = uut.fetch_zero_page_address_with_x_offset();

        assert_eq!(result, 0x0023);
    }

    #[test]
    fn should_increase_cycle_counter_two_times() {
        let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
        uut.program_counter = 0x00;

        assert_eq!(uut.cycle, 0);

        uut.fetch_zero_page_address_with_x_offset();

        assert_eq!(uut.cycle, 2);
    }

    fn should_increase_program_counter_once() {
        let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
        uut.program_counter = 0x00;

        assert_eq!(uut.cycle, 0);

        uut.fetch_zero_page_address_with_x_offset();

        assert_eq!(uut.program_counter, 0x0001);
    }
}

#[cfg(test)]
mod fetch_instruction {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_return_an_instruction_pointed_by_a_program_counter() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.program_counter = 0x0001;

        let result = uut.fetch_instruction();

        assert_eq!(result, 0x51);
    }

    #[test]
    fn should_increase_cycle_counter_and_a_program_counter() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.program_counter = 0x0001;

        assert_eq!(uut.cycle, 0);

        uut.fetch_instruction();

        assert_eq!(uut.cycle, 1);
        assert_eq!(uut.program_counter, 0x0002);
    }
}

#[cfg(test)]
mod push_byte_to_stack {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_push_a_byte_to_a_place_to_the_first_page_in_memory_pointed_by_a_stack_pointer() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.stack_pointer = 0x0002;

        let value: u8 = 0xDF;
        uut.push_byte_to_stack(value);

        assert_eq!(uut.memory[0x0102], 0xDF);
    }

    #[test]
    fn should_increase_cycle_counter_and_stack_pointer_by_one() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.stack_pointer = 0x0002;

        assert_eq!(uut.cycle, 0);

        let value: u8 = 0xDF;
        uut.push_byte_to_stack(value);

        assert_eq!(uut.cycle, 1);
        assert_eq!(uut.stack_pointer, 0x0003);
    }
}

#[cfg(test)]
mod push_word_to_stack {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_push_a_byte_to_a_place_to_the_first_page_in_memory_pointed_by_a_stack_pointer() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.stack_pointer = 0x0002;

        let value: u16 = 0x56DF;
        uut.push_word_to_stack(value);

        assert_eq!(uut.memory[0x0102], 0xDF);
        assert_eq!(uut.memory[0x0103], 0x56);
    }

    #[test]
    fn should_increase_cycle_counter_and_stack_pointer_by_two() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.stack_pointer = 0x0002;
        assert_eq!(uut.cycle, 0);

        let value: u16 = 0x56DF;
        uut.push_word_to_stack(value);

        assert_eq!(uut.cycle, 2);
        assert_eq!(uut.stack_pointer, 0x0004);
    }
}

#[cfg(test)]
mod sum_with_x {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_sum_provided_value_with_x_register_contents() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.index_register_x = 0x02;

        let value: u8 = 0x03;
        let result = uut.sum_with_x(value);

        assert_eq!(result, 0x05);
    }

    #[test]
    fn sum_should_wrap_around_byte() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.index_register_x = 0xFF;

        let value: u8 = 0x03;
        let result = uut.sum_with_x(value);

        assert_eq!(result, 0x02);
    }

    #[test]
    fn should_increase_cycle_counter_by_one() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.index_register_x = 0xFF;
        assert_eq!(uut.cycle, 0);

        let value: u8 = 0x03;
        uut.sum_with_x(value);

        assert_eq!(uut.cycle, 1);
    }
}

#[cfg(test)]
mod set_load_status {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_set_zero_flag_on_processor_status_when_accumulator_is_zero() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b00000000;
        uut.accumulator = 0x00;

        uut.set_load_status();

        assert_eq!(uut.processor_status.flags, 0b00000010);
    }

    #[test]
    fn should_unset_zero_flag_on_processor_status_when_accumulator_is_not_zero() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b11111111;
        uut.accumulator = 0xFF;

        uut.set_load_status();

        assert_eq!(uut.processor_status.flags, 0b11111101);
    }

    #[test]
    fn should_set_negative_flag_on_processor_status_when_accumulator_has_bit_7_set() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b00000000;
        uut.accumulator = 0x80;

        uut.set_load_status();

        assert_eq!(uut.processor_status.flags, 0b10000000);
    }

    #[test]
    fn should_unset_negative_flag_on_processor_status_when_accumulator_has_bit_7_unset() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b11111111;
        uut.accumulator = 0x00;

        uut.set_load_status();

        assert_eq!(uut.processor_status.flags, 0b01111111);
    }
}

#[cfg(test)]
mod fetch_byte_with_offset {
    use super::MemoryMock;
    use crate::{consts::Byte, cpu::CPU};

    #[test]
    fn should_fetch_byte_from_address_pointed_by_program_counter_with_added_provided_offset() {
        let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
        uut.program_counter = 0x0001;

        let offset: Byte = 0x02;
        let result = uut.fetch_byte_with_offset(offset);

        assert_eq!(result, 0x52);
    }

    #[test]
    fn should_take_one_cycle_when_adding_offset_does_not_cross_page_flip() {
        let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
        uut.program_counter = 0x0001;
        uut.cycle = 0;

        let offset: Byte = 0x02;
        uut.fetch_byte_with_offset(offset);

        assert_eq!(uut.cycle, 1);
    }

    #[test]
    fn should_fetch_byte_from_address_pointed_by_program_counter_with_added_provided_offset_when_adding_crosses_page_flip(
    ) {
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x0101] = 0x52;
        let mut uut = CPU::new(Box::new(MemoryMock::new(&memory)));
        uut.program_counter = 0x00FF;

        let offset: Byte = 0x02;
        let result = uut.fetch_byte_with_offset(offset);

        assert_eq!(result, 0x52);
    }

    #[test]
    fn should_take_two_cycles_when_adding_offset_crosses_page_flip() {
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x0101] = 0x52;
        let mut uut = CPU::new(Box::new(MemoryMock::new(&memory)));
        uut.program_counter = 0x00FF;
        uut.cycle = 0;

        let offset: Byte = 0x02;
        uut.fetch_byte_with_offset(offset);

        assert_eq!(uut.cycle, 2);
    }
}
