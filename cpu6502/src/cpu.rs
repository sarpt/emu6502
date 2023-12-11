use self::instructions::*;
use super::consts::{Byte, Word};
use crate::memory::Memory;

mod instructions;

type Instruction = Byte;

const INSTRUCTION_LDA_IM: Byte = 0xA9;
const INSTRUCTION_LDA_ZP: Byte = 0xA5;
const INSTRUCTION_LDA_ZPX: Byte = 0xB5;
const INSTRUCTION_LDA_A: Byte = 0xAD;
const INSTRUCTION_LDA_A_X: Byte = 0xBD;
const INSTRUCTION_LDA_A_Y: Byte = 0xB9;
const INSTRUCTION_LDA_IN_X: Byte = 0xA1;
const INSTRUCTION_LDA_IN_Y: Byte = 0xB1;
const INSTRUCTION_LDY_IM: Byte = 0xA0;
const INSTRUCTION_LDY_ZP: Byte = 0xA4;
const INSTRUCTION_LDY_ZPX: Byte = 0xB4;
const INSTRUCTION_LDY_A: Byte = 0xAC;
const INSTRUCTION_LDY_A_X: Byte = 0xBC;
const INSTRUCTION_LDX_IM: Byte = 0xA2;
const INSTRUCTION_LDX_ZP: Byte = 0xA6;
const INSTRUCTION_LDX_ZPY: Byte = 0xB6;
const INSTRUCTION_LDX_A: Byte = 0xAE;
const INSTRUCTION_LDX_A_Y: Byte = 0xBE;
const INSTRUCTION_JMP_A: Byte = 0x4C;
const INSTRUCTION_JMP_IN: Byte = 0x6C;
const INSTRUCTION_JSR_A: Byte = 0x20;

enum Flags {
    Zero = 1,
    DecimalMode = 3,
    Negative = 7,
}

struct ProcessorStatus {
    flags: Byte,
}

enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndexIndirectX,
    IndirectIndexY,
}

enum Register {
    Accumulator,
    IndexX,
    IndexY,
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
    memory: Box<dyn Memory>,
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
            memory: memory,
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

        return data;
    }

    fn increment_program_counter(&mut self) {
        self.program_counter = self.program_counter.wrapping_add(1);
        self.cycle += 1;
    }

    fn decrement_program_counter(&mut self) {
        self.program_counter = self.program_counter.wrapping_sub(1);
        self.cycle += 1;
    }

    fn increment_stack_pointer(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
    }

    fn fetch_byte(&mut self) -> Byte {
        let data = self.access_memory(self.program_counter);
        self.increment_program_counter();

        return data;
    }

    fn fetch_byte_with_offset(&mut self, offset: Byte) -> Byte {
        let lsb: u8 = (self.program_counter) as u8;
        let mut msb: u8 = (self.program_counter >> 8) as u8; // change to "to_le_bytes"

        let (new_lsb, carry) = lsb.overflowing_add(offset);
        self.program_counter = ((msb as u16) << 8) | new_lsb as u16;
        self.cycle += 1;
        if !carry {
            return self.access_memory(self.program_counter);
        };

        msb = msb.wrapping_add(1);
        self.program_counter = ((msb as u16) << 8) | new_lsb as u16;
        self.cycle += 1;
        return self.access_memory(self.program_counter);
    }

    fn fetch_word(&mut self) -> Word {
        let lsb: Word = self.fetch_byte().into();
        let msb: Word = self.fetch_byte().into();

        return (msb << 8) | lsb;
    }

    fn fetch_instruction(&mut self) -> Instruction {
        return self.fetch_byte();
    }

    fn fetch_address(&mut self) -> Word {
        return self.fetch_word();
    }

    fn fetch_address_from(&mut self, addr: Word) -> Word {
        self.program_counter = addr;
        return self.fetch_word();
    }

    fn fetch_zero_page_address(&mut self) -> Word {
        return self.fetch_byte().into();
    }

    fn fetch_zero_page_address_with_y_offset(&mut self) -> Word {
        let zero_page_addr = self.fetch_byte();
        return self.sum_with_y(zero_page_addr).into();
    }

    fn fetch_zero_page_address_with_x_offset(&mut self) -> Word {
        let zero_page_addr = self.fetch_byte();
        return self.sum_with_x(zero_page_addr).into();
    }

    fn set_load_status(&mut self, register: &Register) {
        let target_register = match register {
            Register::Accumulator => self.accumulator,
            Register::IndexX => self.index_register_x,
            Register::IndexY => self.index_register_y,
        };

        self.processor_status.set_zero_flag(target_register == 0);
        self.processor_status
            .set_negative_flag((target_register & 0b10000000) > 1);
    }

    fn sum_with_x(&mut self, val: Byte) -> Byte {
        let reg_x = self.index_register_x;
        let res = val.wrapping_add(reg_x);
        self.cycle += 1;

        return res;
    }

    fn sum_with_y(&mut self, val: Byte) -> Byte {
        let reg_y = self.index_register_y;
        let res = val.wrapping_add(reg_y);
        self.cycle += 1;

        return res;
    }

    fn push_byte_to_stack(&mut self, val: Byte) {
        let stack_addr: Word = 0x0100 | (self.stack_pointer as u16);
        self.memory[stack_addr] = val;
        self.increment_stack_pointer();
        self.increment_program_counter();
    }

    fn push_word_to_stack(&mut self, val: Word) {
        let lsb: u8 = (val) as u8;
        let msb: u8 = (val >> 8) as u8; // change to "to_le_bytes"
        self.push_byte_to_stack(lsb);
        self.push_byte_to_stack(msb);
    }

    pub fn set_memory(&mut self, memory: Box<dyn Memory>) {
        self.memory = memory;
    }

    fn prepare_program_counter(&mut self, addr_mode: &AddressingMode) {
        match addr_mode {
            AddressingMode::ZeroPage | AddressingMode::IndirectIndexY => {
                self.program_counter = self.fetch_zero_page_address();
            }
            AddressingMode::ZeroPageY => {
                self.program_counter = self.fetch_zero_page_address_with_y_offset();
            }
            AddressingMode::ZeroPageX | AddressingMode::IndexIndirectX => {
                self.program_counter = self.fetch_zero_page_address_with_x_offset();
            }
            _ => {}
        }

        match addr_mode {
            AddressingMode::Absolute
            | AddressingMode::AbsoluteX
            | AddressingMode::AbsoluteY
            | AddressingMode::IndexIndirectX
            | AddressingMode::IndirectIndexY => {
                self.program_counter = self.fetch_address();
            }
            _ => {}
        }
    }

    pub fn execute(&mut self, cycles: u64) -> u64 {
        let cycles_before_execution = self.cycle;
        let stop_cycle = cycles_before_execution + cycles;

        while self.cycle < stop_cycle {
            let instruction = self.fetch_instruction();
            match instruction {
                INSTRUCTION_LDA_IM => {
                    lda_im(self);
                }
                INSTRUCTION_LDA_ZP => {
                    lda_zp(self);
                }
                INSTRUCTION_LDA_ZPX => {
                    lda_zpx(self);
                }
                INSTRUCTION_LDA_A => {
                    lda_a(self);
                }
                INSTRUCTION_LDA_A_X => {
                    lda_a_x(self);
                }
                INSTRUCTION_LDA_A_Y => {
                    lda_a_y(self);
                }
                INSTRUCTION_LDA_IN_X => {
                    lda_in_x(self);
                }
                INSTRUCTION_LDA_IN_Y => {
                    lda_in_y(self);
                }
                INSTRUCTION_LDY_IM => {
                    ldy_im(self);
                }
                INSTRUCTION_LDY_ZP => {
                    ldy_zp(self);
                }
                INSTRUCTION_LDY_ZPX => {
                    ldy_zpx(self);
                }
                INSTRUCTION_LDY_A => {
                    ldy_a(self);
                }
                INSTRUCTION_LDY_A_X => {
                    ldy_a_x(self);
                }
                INSTRUCTION_LDX_IM => {
                    ldx_im(self);
                }
                INSTRUCTION_LDX_ZP => {
                    ldx_zp(self);
                }
                INSTRUCTION_LDX_ZPY => {
                    ldx_zpy(self);
                }
                INSTRUCTION_LDX_A => {
                    ldx_a(self);
                }
                INSTRUCTION_LDX_A_Y => {
                    ldx_a_y(self);
                }
                INSTRUCTION_JSR_A => {
                    jsr_a(self);
                }
                INSTRUCTION_JMP_A => {
                    jmp_a(self);
                }
                INSTRUCTION_JMP_IN => {
                    jmp_in(self);
                }
                _ => (),
            };
        }

        return stop_cycle;
    }
}

#[cfg(test)]
mod tests;
