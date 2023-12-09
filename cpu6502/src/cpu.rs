use self::instructions::*;
use super::consts::{Word, Byte};
use crate::memory::Memory;

mod instructions;

type Instruction = Byte;

const INSTRUCTION_LDA_IM: Byte = 0xA9;
const INSTRUCTION_LDA_ZP: Byte = 0xA5;
const INSTRUCTION_LDA_ZPX: Byte = 0xB5;
const INSTRUCTION_LDA_A: Byte = 0xAD;
const INSTRUCTION_JMP_A: Byte = 0x4C;
const INSTRUCTION_JSR_A: Byte = 0x20;

enum Flags {
    Zero = 1,
    DecimalMode = 3,
    Negative = 7
}

struct ProcessorStatus {
    flags: Byte,
} 

impl ProcessorStatus {
    pub fn set_decimal_mode_flag(&mut self, value_set: bool) {
        self.set_flag(Flags::DecimalMode, value_set);
    }

    pub fn set_zero_flag(&mut self, value_set: bool) {
        self.set_flag(Flags::Zero, value_set);
    }

    pub fn set_negative_flag(&mut self, value_set: bool) {
        self.set_flag(Flags::Negative, value_set);
    }

    fn set_flag(&mut self, flag: Flags, value_set: bool) {
        let shift: u8 = flag as u8;
        if value_set {
            self.flags |= 1 << shift;
        } else {
            self.flags &= !(1 << shift);
        }
    }
}

pub struct CPU {
    cycle: u64,
    program_counter: Word,
    stack_pointer: Byte,
    // registers
    accumulator: Byte,
    index_register_x: Byte,
    index_register_y: Byte,
    processor_status: ProcessorStatus,
    memory: Box<dyn Memory>
}

impl CPU {
    pub fn new(memory: Box<dyn Memory>) -> Self {
        return CPU {
            cycle: 0,
            program_counter: 0xFFFC,
            stack_pointer: 0,
            accumulator: 0,
            index_register_x: 0,
            index_register_y: 0,
            processor_status: ProcessorStatus { flags: 0 },
            memory: memory
        };
    }

    pub fn reset(&mut self) -> () {
        self.cycle = 0;
        self.program_counter = 0xFFFC;
        self.stack_pointer = 0x00;
        self.processor_status.set_decimal_mode_flag(false);
        self.accumulator = 0;
        self.index_register_x = 0;
        self.index_register_y = 0;
    }

    fn access_memory(&mut self, addr: Word) -> Byte {
        let data = self.memory[addr];
        self.cycle += 1;

        return data;
    }

    fn fetch_byte(&mut self) -> Byte {
        let data = self.access_memory(self.program_counter);
        self.program_counter = self.program_counter.wrapping_add(1);

        return data;
    }

    fn fetch_word(&mut self) -> Word {
        let msb: Word = self.fetch_byte().into();
        let lsb: Word = self.fetch_byte().into();

        return (lsb << 8) | msb;
    }

    fn fetch_instruction(&mut self) -> Instruction {
        return self.fetch_byte();
    }

    fn set_load_accumulator_status(&mut self) -> () {
        self.processor_status.set_zero_flag(self.accumulator == 0);
        self.processor_status.set_negative_flag((self.accumulator & 0b10000000) > 1);
    }

    fn sum_with_x(&mut self, val: Byte) -> Byte {
        let reg_x = self.index_register_x;
        let res = val.wrapping_add(reg_x);
        self.cycle += 1;

        return res;
    }

    fn push_byte_to_stack(&mut self, val: Byte) {
        let stack_addr: Word = 0x0100 | (self.stack_pointer as u16);
        self.memory[stack_addr] = val;
        self.stack_pointer += 1;
        self.cycle += 1;
    }

    fn push_word_to_stack(&mut self, val: Word) {
        let msb: u8 = (val) as u8;
        let lsb: u8 = (val >> 8) as u8; // change to "to_le_bytes"
        self.push_byte_to_stack(msb);
        self.push_byte_to_stack(lsb);
    }

    pub fn set_memory(&mut self, memory: Box<dyn Memory>) {
        self.memory = memory;
    }

    pub fn execute(&mut self, cycles: u64) -> u64 {
        let cycles_before_execution = self.cycle;
        let stop_cycle = cycles_before_execution + cycles;

        while self.cycle < stop_cycle {
            let instruction = self.fetch_instruction();
            match instruction {
                INSTRUCTION_LDA_IM => {
                    lda_im(self);
                },
                INSTRUCTION_LDA_ZP => {
                    lda_zp(self);
                },
                INSTRUCTION_LDA_ZPX => {
                    lda_zpx(self);
                },
                INSTRUCTION_LDA_A => {
                    lda_a(self);
                },
                INSTRUCTION_JSR_A => {
                    jsr_a(self);
                },
                INSTRUCTION_JMP_A => {
                    jmp_a(self);
                },
                _ => ()
            };
        }

        return stop_cycle;
    }
}

#[cfg(test)]
mod tests;