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

    #[test]
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
        uut.stack_pointer = 0xFF;

        let value: u8 = 0xDF;
        uut.push_byte_to_stack(value);

        assert_eq!(uut.memory[0x01FF], 0xDF);
    }

    #[test]
    fn should_increase_cycle_counter_and_decrease_stack_pointer_by_one() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.stack_pointer = 0xFF;

        assert_eq!(uut.cycle, 0);

        let value: u8 = 0xDF;
        uut.push_byte_to_stack(value);

        assert_eq!(uut.cycle, 1);
        assert_eq!(uut.stack_pointer, 0xFE);
    }
}

#[cfg(test)]
mod push_word_to_stack {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_push_a_byte_to_a_place_to_the_first_page_in_memory_pointed_by_a_stack_pointer() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.stack_pointer = 0xFF;

        let value: u16 = 0x56DF;
        uut.push_word_to_stack(value);

        assert_eq!(uut.memory[0x01FF], 0xDF);
        assert_eq!(uut.memory[0x01FE], 0x56);
    }

    #[test]
    fn should_increase_cycle_counter_and_decrease_stack_pointer_by_two() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.stack_pointer = 0xFF;
        assert_eq!(uut.cycle, 0);

        let value: u16 = 0x56DF;
        uut.push_word_to_stack(value);

        assert_eq!(uut.cycle, 2);
        assert_eq!(uut.stack_pointer, 0xFD);
    }
}

#[cfg(test)]
mod pop_byte_from_stack {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_pop_byte_from_stack() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.memory[0x01FF] = 0xDF;
        uut.memory[0x01FE] = 0x48;
        uut.stack_pointer = 0xFD;

        let value = uut.pop_byte_from_stack();

        assert_eq!(value, 0x48);
    }

    #[test]
    fn should_increment_cycle_count_and_stack_pointer_once() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.memory[0x01FF] = 0xDF;
        uut.memory[0x01FE] = 0x48;
        uut.stack_pointer = 0xFD;

        assert_eq!(uut.cycle, 0);

        uut.pop_byte_from_stack();

        assert_eq!(uut.cycle, 1);
        assert_eq!(uut.stack_pointer, 0xFE);
    }
}

#[cfg(test)]
mod pop_word_from_stack {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_pop_word_from_stack() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.memory[0x01FF] = 0xDF;
        uut.memory[0x01FE] = 0x48;
        uut.stack_pointer = 0xFD;

        let val = uut.pop_word_from_stack();

        assert_eq!(val, 0xDF48);
    }

    #[test]
    fn should_increment_cycle_count_and_stack_pointer_twice() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.memory[0x01FF] = 0xDF;
        uut.memory[0x01FE] = 0x48;
        uut.stack_pointer = 0xFD;
        assert_eq!(uut.cycle, 0);

        uut.pop_word_from_stack();

        assert_eq!(uut.cycle, 2);
        assert_eq!(uut.stack_pointer, 0xFF);
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
    use crate::cpu::{Register, CPU};

    #[test]
    fn should_set_zero_flag_on_processor_status_when_register_is_zero() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b00000000;
        uut.accumulator = 0x00;

        let register = Register::Accumulator;
        uut.set_load_status(&register);

        assert_eq!(uut.processor_status.flags, 0b00000010);
    }

    #[test]
    fn should_unset_zero_flag_on_processor_status_when_register_is_not_zero() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b11111111;
        uut.accumulator = 0xFF;

        let register = Register::Accumulator;
        uut.set_load_status(&register);

        assert_eq!(uut.processor_status.flags, 0b11111101);
    }

    #[test]
    fn should_set_negative_flag_on_processor_status_when_register_has_bit_7_set() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b00000000;
        uut.accumulator = 0x80;

        let register = Register::Accumulator;
        uut.set_load_status(&register);

        assert_eq!(uut.processor_status.flags, 0b10000000);
    }

    #[test]
    fn should_unset_negative_flag_on_processor_status_when_register_has_bit_7_unset() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b11111111;
        uut.accumulator = 0x00;

        let register = Register::Accumulator;
        uut.set_load_status(&register);

        assert_eq!(uut.processor_status.flags, 0b01111111);
    }
}

#[cfg(test)]
mod fetch_byte_with_offset {
    use super::MemoryMock;
    use crate::{consts::Byte, cpu::CPU};

    #[test]
    fn should_fetch_byte_from_address_with_added_provided_offset() {
        let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));

        let addr = 0x0001;
        let offset: Byte = 0x02;
        let result = uut.fetch_byte_with_offset(addr, offset);

        assert_eq!(result, 0x52);
    }

    #[test]
    fn should_take_one_cycle_when_adding_offset_does_not_cross_page_flip() {
        let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
        uut.cycle = 0;

        let addr = 0x0001;
        let offset: Byte = 0x02;
        uut.fetch_byte_with_offset(addr, offset);

        assert_eq!(uut.cycle, 1);
    }

    #[test]
    fn should_fetch_byte_from_address_with_added_provided_offset_when_adding_crosses_page_flip() {
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x0101] = 0x52;
        let mut uut = CPU::new(Box::new(MemoryMock::new(&memory)));

        let addr = 0x00FF;
        let offset: Byte = 0x02;
        let result = uut.fetch_byte_with_offset(addr, offset);

        assert_eq!(result, 0x52);
    }

    #[test]
    fn should_take_two_cycles_when_adding_offset_crosses_page_flip() {
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x0101] = 0x52;
        let mut uut = CPU::new(Box::new(MemoryMock::new(&memory)));
        uut.cycle = 0;

        let addr = 0x00FF;
        let offset: Byte = 0x02;
        uut.fetch_byte_with_offset(addr, offset);

        assert_eq!(uut.cycle, 2);
    }
}

#[cfg(test)]
mod get_address {

    #[cfg(test)]
    mod immediate_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, CPU};

        #[test]
        fn should_return_program_counter_address() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0xCB;

            let result = uut.get_address(&AddressingMode::Immediate);

            assert_eq!(result, 0xCB);
        }

        #[test]
        fn should_not_change_program_counter() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0xCB;

            uut.get_address(&AddressingMode::Immediate);

            assert_eq!(uut.program_counter, 0xCB);
        }

        #[test]
        fn should_not_take_any_cycles() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0xCB;
            uut.cycle = 0;

            uut.get_address(&AddressingMode::Immediate);

            assert_eq!(uut.cycle, 0);
        }
    }

    #[cfg(test)]
    mod absolute_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, CPU};

        #[test]
        fn should_return_address_from_next_word_in_memory_relative_to_program_counter() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x01;

            let result = uut.get_address(&AddressingMode::Absolute);

            assert_eq!(result, 0xCBFF);
        }

        #[test]
        fn should_advance_program_counter_twice() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x01;

            uut.get_address(&AddressingMode::Absolute);

            assert_eq!(uut.program_counter, 0x03);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x01;
            uut.cycle = 0;

            uut.get_address(&AddressingMode::Absolute);

            assert_eq!(uut.cycle, 2);
        }
    }

    #[cfg(test)]
    mod absolute_x_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, CPU};

        #[test]
        fn should_return_address_from_next_word_in_memory_relative_to_program_counter() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;

            let result = uut.get_address(&AddressingMode::AbsoluteX);

            assert_eq!(result, 0x52CB);
        }

        #[test]
        fn should_advance_program_counter_twice() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;

            uut.get_address(&AddressingMode::AbsoluteX);

            assert_eq!(uut.program_counter, 0x04);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.cycle = 0;

            uut.get_address(&AddressingMode::AbsoluteX);

            assert_eq!(uut.cycle, 2);
        }
    }

    #[cfg(test)]
    mod absolute_y_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, CPU};

        #[test]
        fn should_return_address_from_next_word_in_memory_relative_to_program_counter() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;

            let result = uut.get_address(&AddressingMode::AbsoluteY);

            assert_eq!(result, 0x52CB);
        }

        #[test]
        fn should_advance_program_counter_twice() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;

            uut.get_address(&AddressingMode::AbsoluteY);

            assert_eq!(uut.program_counter, 0x04);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.cycle = 0;

            uut.get_address(&AddressingMode::AbsoluteY);

            assert_eq!(uut.cycle, 2);
        }
    }

    #[cfg(test)]
    mod zero_page_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, CPU};

        #[test]
        fn should_return_address_in_zero_page_from_next_byte_in_memory_relative_to_program_counter()
        {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;

            let result = uut.get_address(&AddressingMode::ZeroPage);

            assert_eq!(result, 0x00CB);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;

            uut.get_address(&AddressingMode::ZeroPage);

            assert_eq!(uut.program_counter, 0x03);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.cycle = 0;

            uut.get_address(&AddressingMode::ZeroPage);

            assert_eq!(uut.cycle, 1);
        }
    }

    #[cfg(test)]
    mod zero_page_x_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, CPU};

        #[test]
        fn should_return_address_in_zero_page_from_next_byte_in_memory_relative_to_program_counter_summed_with_index_register_x(
        ) {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_x = 0x03;

            let result = uut.get_address(&AddressingMode::ZeroPageX);

            assert_eq!(result, 0x00CE);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_x = 0x03;

            uut.get_address(&AddressingMode::ZeroPageX);

            assert_eq!(uut.program_counter, 0x03);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_x = 0x03;
            uut.cycle = 0;

            uut.get_address(&AddressingMode::ZeroPageX);

            assert_eq!(uut.cycle, 2);
        }
    }

    #[cfg(test)]
    mod zero_page_y_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, CPU};

        #[test]
        fn should_return_address_in_zero_page_from_next_byte_in_memory_relative_to_program_counter_summed_with_index_register_y(
        ) {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x03;
            uut.index_register_y = 0x03;

            let result = uut.get_address(&AddressingMode::ZeroPageY);

            assert_eq!(result, 0x0055);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_y = 0x03;

            uut.get_address(&AddressingMode::ZeroPageY);

            assert_eq!(uut.program_counter, 0x03);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_y = 0x03;
            uut.cycle = 0;

            uut.get_address(&AddressingMode::ZeroPageY);

            assert_eq!(uut.cycle, 2);
        }
    }

    #[cfg(test)]
    mod index_indirect_x_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, CPU};

        #[test]
        fn should_return_address_stored_in_place_pointed_by_zero_page_address_in_next_byte_relative_to_program_counter_summed_with_index_register_x(
        ) {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x01, 0xFF, 0x03, 0xDD, 0x25])));
            uut.program_counter = 0x00;
            uut.index_register_x = 0x01;

            let result = uut.get_address(&AddressingMode::IndexIndirectX);

            assert_eq!(result, 0xDD03);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x01, 0xFF, 0x03, 0xDD, 0x25])));
            uut.program_counter = 0x00;
            uut.index_register_x = 0x01;

            uut.get_address(&AddressingMode::IndexIndirectX);

            assert_eq!(uut.program_counter, 0x01);
        }

        #[test]
        fn should_take_four_cycles() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x01, 0xFF, 0x03, 0xDD, 0x25])));
            uut.program_counter = 0x00;
            uut.index_register_x = 0x01;
            uut.cycle = 0;

            uut.get_address(&AddressingMode::IndexIndirectX);

            assert_eq!(uut.cycle, 4);
        }
    }

    #[cfg(test)]
    mod indirect_index_y_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, CPU};

        #[test]
        fn should_return_address_stored_in_place_pointed_by_zero_page_address_in_next_byte_relative_to_program_counter(
        ) {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25])));
            uut.program_counter = 0x00;

            let result = uut.get_address(&AddressingMode::IndirectIndexY);

            assert_eq!(result, 0xDD03);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25])));
            uut.program_counter = 0x00;

            uut.get_address(&AddressingMode::IndirectIndexY);

            assert_eq!(uut.program_counter, 0x01);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25])));
            uut.program_counter = 0x00;
            uut.cycle = 0;

            uut.get_address(&AddressingMode::IndirectIndexY);

            assert_eq!(uut.cycle, 3);
        }
    }

    #[cfg(test)]
    mod indirect_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, CPU};

        #[test]
        fn should_return_address_from_place_in_memory_stored_in_next_word_relative_to_program_counter(
        ) {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
            uut.program_counter = 0x00;

            let result = uut.get_address(&AddressingMode::Indirect);

            assert_eq!(result, 0x0001);
        }

        #[test]
        fn should_advance_program_counter_twice() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
            uut.program_counter = 0x00;

            uut.get_address(&AddressingMode::Indirect);

            assert_eq!(uut.program_counter, 0x02);
        }

        #[test]
        fn should_take_four_cycles() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
            uut.program_counter = 0x02;
            uut.cycle = 0;

            uut.get_address(&AddressingMode::Indirect);

            assert_eq!(uut.cycle, 4);
        }

        #[test]
        fn should_incorrectly_interpret_address_stored_in_next_word_when_it_points_to_page_edge_and_take_lsb_from_correct_address_but_wrap_around_page_for_msb(
        ) {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0xFF, 0x00, 0x04, 0x00])));
            uut.program_counter = 0x00;

            let result = uut.get_address(&AddressingMode::Indirect);

            assert_eq!(result, 0xFF00);
        }
    }
}
