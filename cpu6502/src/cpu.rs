use std::collections::HashMap;

use self::instructions::*;
use super::consts::{Byte, Word};
use crate::{consts::STACK_PAGE_HI, memory::Memory};

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
const INSTRUCTION_RTS: Byte = 0x60;
const INSTRUCTION_BEQ: Byte = 0xF0;
const INSTRUCTION_BCC: Byte = 0x90;
const INSTRUCTION_BCS: Byte = 0xB0;
const INSTRUCTION_BNE: Byte = 0xD0;
const INSTRUCTION_CMP_IM: Byte = 0xC9;
const INSTRUCTION_CMP_ZP: Byte = 0xC5;
const INSTRUCTION_CMP_ZPX: Byte = 0xD5;
const INSTRUCTION_CMP_A: Byte = 0xCD;
const INSTRUCTION_CMP_A_X: Byte = 0xDD;
const INSTRUCTION_CMP_A_Y: Byte = 0xD9;
const INSTRUCTION_CMP_IN_X: Byte = 0xC1;
const INSTRUCTION_CMP_IN_Y: Byte = 0xD1;
const INSTRUCTION_CPX_IM: Byte = 0xE0;
const INSTRUCTION_CPX_ZP: Byte = 0xE4;
const INSTRUCTION_CPX_A: Byte = 0xEC;
const INSTRUCTION_CPY_IM: Byte = 0xC0;
const INSTRUCTION_CPY_ZP: Byte = 0xC4;
const INSTRUCTION_CPY_A: Byte = 0xCC;

enum Flags {
    Carry = 0,
    Zero = 1,
    DecimalMode = 3,
    Negative = 7,
}

struct ProcessorStatus {
    flags: Byte,
}

enum AddressingMode {
    Immediate,
    Indirect,
    Implicit,
    Relative,
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

    pub fn set_carry_flag(&mut self, value_set: bool) {
        self.set_flag(Flags::Carry, value_set);
    }

    pub fn get_carry_flag(&self) -> bool {
        return self.get_flag(Flags::Carry);
    }

    pub fn get_zero_flag(&self) -> bool {
        return self.get_flag(Flags::Zero);
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

    fn get_flag(&self, flag: Flags) -> bool {
        let shift: u8 = flag as u8;
        return (self.flags & (1 << shift)) > 0;
    }
}

type OpcodeHandler = fn(&mut CPU) -> ();

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
    opcode_handlers: HashMap<Byte, OpcodeHandler>,
}

impl CPU {
    pub fn new(memory: Box<dyn Memory>) -> Self {
        let opcode_handlers: HashMap<Byte, OpcodeHandler> = HashMap::from([
            (INSTRUCTION_LDA_IM, lda_im as OpcodeHandler),
            (INSTRUCTION_LDA_ZP, lda_zp as OpcodeHandler),
            (INSTRUCTION_LDA_ZPX, lda_zpx as OpcodeHandler),
            (INSTRUCTION_LDA_A, lda_a as OpcodeHandler),
            (INSTRUCTION_LDA_A_X, lda_a_x as OpcodeHandler),
            (INSTRUCTION_LDA_A_Y, lda_a_y as OpcodeHandler),
            (INSTRUCTION_LDA_IN_X, lda_in_x as OpcodeHandler),
            (INSTRUCTION_LDA_IN_Y, lda_in_y as OpcodeHandler),
            (INSTRUCTION_LDY_IM, ldy_im as OpcodeHandler),
            (INSTRUCTION_LDY_ZP, ldy_zp as OpcodeHandler),
            (INSTRUCTION_LDY_ZPX, ldy_zpx as OpcodeHandler),
            (INSTRUCTION_LDY_A, ldy_a as OpcodeHandler),
            (INSTRUCTION_LDY_A_X, ldy_a_x as OpcodeHandler),
            (INSTRUCTION_LDX_IM, ldx_im as OpcodeHandler),
            (INSTRUCTION_LDX_ZP, ldx_zp as OpcodeHandler),
            (INSTRUCTION_LDX_ZPY, ldx_zpy as OpcodeHandler),
            (INSTRUCTION_LDX_A, ldx_a as OpcodeHandler),
            (INSTRUCTION_LDX_A_Y, ldx_a_y as OpcodeHandler),
            (INSTRUCTION_JMP_A, jmp_a as OpcodeHandler),
            (INSTRUCTION_JMP_IN, jmp_in as OpcodeHandler),
            (INSTRUCTION_JSR_A, jsr_a as OpcodeHandler),
            (INSTRUCTION_RTS, rts as OpcodeHandler),
            (INSTRUCTION_BCC, bcc as OpcodeHandler),
            (INSTRUCTION_BCS, bcs as OpcodeHandler),
            (INSTRUCTION_BEQ, beq as OpcodeHandler),
            (INSTRUCTION_BNE, bne as OpcodeHandler),
            (INSTRUCTION_CMP_IM, cmp_im as OpcodeHandler),
            (INSTRUCTION_CMP_ZP, cmp_zp as OpcodeHandler),
            (INSTRUCTION_CMP_ZPX, cmp_zpx as OpcodeHandler),
            (INSTRUCTION_CMP_A, cmp_a as OpcodeHandler),
            (INSTRUCTION_CMP_A_X, cmp_a_x as OpcodeHandler),
            (INSTRUCTION_CMP_A_Y, cmp_a_y as OpcodeHandler),
            (INSTRUCTION_CMP_IN_X, cmp_in_x as OpcodeHandler),
            (INSTRUCTION_CMP_IN_Y, cmp_in_y as OpcodeHandler),
            (INSTRUCTION_CPX_IM, cpx_im as OpcodeHandler),
            (INSTRUCTION_CPX_ZP, cpx_zp as OpcodeHandler),
            (INSTRUCTION_CPX_A, cpx_a as OpcodeHandler),
            (INSTRUCTION_CPY_IM, cpy_im as OpcodeHandler),
            (INSTRUCTION_CPY_ZP, cpy_zp as OpcodeHandler),
            (INSTRUCTION_CPY_A, cpy_a as OpcodeHandler),
        ]);

        return CPU {
            cycle: 0,
            program_counter: 0xFFFC,
            stack_pointer: 0,
            accumulator: 0,
            index_register_x: 0,
            index_register_y: 0,
            processor_status: ProcessorStatus { flags: 0 },
            memory: memory,
            opcode_handlers,
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
        return self.memory[addr];
    }

    fn increment_program_counter(&mut self) {
        self.program_counter = self.program_counter.wrapping_add(1);
        self.cycle += 1;
    }

    fn increment_stack_pointer(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
    }

    fn decrement_stack_pointer(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn fetch_byte_with_offset(&mut self, addr: Word, offset: Byte) -> Byte {
        let [lo, mut hi] = addr.to_le_bytes();
        let (new_lo, carry) = lo.overflowing_add(offset);
        let mut address = Word::from_le_bytes([new_lo, hi]);
        self.cycle += 1;
        if !carry {
            return self.access_memory(address);
        };

        hi = hi.wrapping_add(1);
        address = ((hi as u16) << 8) | new_lo as u16;
        self.cycle += 1;
        return self.access_memory(address);
    }

    fn fetch_instruction(&mut self) -> Instruction {
        let opcode = self.access_memory(self.program_counter);
        self.increment_program_counter();

        return opcode;
    }

    fn fetch_address(&mut self) -> Word {
        let lo = self.access_memory(self.program_counter);
        self.increment_program_counter();
        let hi = self.access_memory(self.program_counter);
        self.increment_program_counter();

        return Word::from_le_bytes([lo, hi]);
    }

    fn fetch_address_from(&mut self, addr: Word) -> Word {
        let lo = self.access_memory(addr);
        self.cycle += 1;
        let hi = self.access_memory(addr + 1);
        self.cycle += 1;

        return Word::from_le_bytes([lo, hi]);
    }

    fn fetch_zero_page_address(&mut self) -> Word {
        let address: Word = self.access_memory(self.program_counter).into();
        self.increment_program_counter();

        return address;
    }

    fn fetch_zero_page_address_lsb(&mut self) -> Byte {
        let address: Byte = self.access_memory(self.program_counter);
        self.increment_program_counter();

        return address;
    }

    fn fetch_zero_page_address_with_y_offset(&mut self) -> Word {
        let zero_page_addr = self.fetch_zero_page_address_lsb();
        return self.sum_with_y(zero_page_addr).into();
    }

    fn fetch_zero_page_address_with_x_offset(&mut self) -> Word {
        let zero_page_addr = self.fetch_zero_page_address_lsb();
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

    fn set_cmp_status(&mut self, register: &Register, value: Byte) {
        let target_register = match register {
            Register::Accumulator => self.accumulator,
            Register::IndexX => self.index_register_x,
            Register::IndexY => self.index_register_y,
        };

        self.processor_status
            .set_carry_flag(target_register >= value);
        self.processor_status
            .set_zero_flag(target_register == value);
        self.processor_status
            .set_negative_flag(((target_register.wrapping_sub(value)) & 0b10000000) > 1);
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
        let stack_addr: Word = STACK_PAGE_HI | (self.stack_pointer as u16);
        self.memory[stack_addr] = val;
        self.decrement_stack_pointer();
        self.cycle += 1;
    }

    fn push_word_to_stack(&mut self, val: Word) {
        let [lo, hi] = val.to_le_bytes();
        self.push_byte_to_stack(lo);
        self.push_byte_to_stack(hi);
    }

    fn pop_byte_from_stack(&mut self) -> Byte {
        self.increment_stack_pointer();
        let stack_addr: Word = STACK_PAGE_HI | (self.stack_pointer as u16);
        self.cycle += 1;
        let val = self.memory[stack_addr];

        return val;
    }

    fn pop_word_from_stack(&mut self) -> Word {
        let lo = self.pop_byte_from_stack();
        let hi = self.pop_byte_from_stack();

        return Word::from_le_bytes([lo, hi]);
    }

    pub fn set_memory(&mut self, memory: Box<dyn Memory>) {
        self.memory = memory;
    }

    pub fn offset_program_counter(&mut self, offset: u8) {
        let [program_counter_lo, program_counter_hi] = self.program_counter.to_le_bytes();
        let negative_offset_direction = 0b10000000 & offset > 0;
        let offset = 0b01111111 & offset;
        let offset_program_counter_lo: Byte;
        let carry: bool;

        if negative_offset_direction {
            (offset_program_counter_lo, carry) = program_counter_lo.overflowing_sub(offset);
        } else {
            (offset_program_counter_lo, carry) = program_counter_lo.overflowing_add(offset);
        }

        self.program_counter = Word::from_le_bytes([offset_program_counter_lo, program_counter_hi]);
        self.cycle += 1;
        if !carry {
            return;
        }

        let offset_program_counter_hi: Byte;
        if negative_offset_direction {
            offset_program_counter_hi = program_counter_hi.wrapping_sub(1);
        } else {
            offset_program_counter_hi = program_counter_hi.wrapping_add(1);
        }
        self.program_counter =
            Word::from_le_bytes([offset_program_counter_lo, offset_program_counter_hi]);
        self.cycle += 1;
    }

    fn get_address(&mut self, addr_mode: &AddressingMode) -> Option<Word> {
        match addr_mode {
            AddressingMode::ZeroPage => {
                return Some(self.fetch_zero_page_address());
            }
            AddressingMode::IndexIndirectX => {
                let address = self.fetch_zero_page_address_with_x_offset();
                return Some(self.fetch_address_from(address));
            }
            AddressingMode::IndirectIndexY => {
                let address = self.fetch_zero_page_address();
                return Some(self.fetch_address_from(address));
            }
            AddressingMode::ZeroPageY => {
                return Some(self.fetch_zero_page_address_with_y_offset());
            }
            AddressingMode::ZeroPageX => {
                return Some(self.fetch_zero_page_address_with_x_offset());
            }
            AddressingMode::Absolute | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => {
                return Some(self.fetch_address());
            }
            AddressingMode::Indirect => {
                let address = self.fetch_address();
                let should_incorrectly_jump = address & 0x00FF == 0x00FF;
                if !should_incorrectly_jump {
                    return Some(self.fetch_address_from(address));
                };

                let hi = self.access_memory(address);
                let lo = self.access_memory(address & 0x1100);
                let incorrect_jmp_address = Word::from_le_bytes([hi, lo]);

                return Some(incorrect_jmp_address);
            }
            AddressingMode::Immediate => {
                return Some(self.program_counter);
            }
            _ => None,
        }
    }

    pub fn execute(&mut self, cycles: u64) -> u64 {
        let cycles_before_execution = self.cycle;
        let stop_cycle = cycles_before_execution + cycles;

        while self.cycle < stop_cycle {
            let opcode = self.fetch_instruction();
            let handler = self.opcode_handlers.get(&opcode);
            match handler {
                Some(cb) => cb(self),
                None => panic!("illegal opcode found: {opcode}"),
            }
        }

        return stop_cycle;
    }
}

#[cfg(test)]
mod tests;
