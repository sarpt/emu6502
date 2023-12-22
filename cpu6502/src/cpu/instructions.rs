use crate::consts::{Byte, Word};

use super::{AddressingMode, Register, CPU};

pub fn ld(cpu: &mut CPU, addr_mode: AddressingMode, register: Register) {
    let address = cpu.get_address(&addr_mode);

    let value = match addr_mode {
        AddressingMode::AbsoluteY | AddressingMode::IndirectIndexY => {
            cpu.fetch_byte_with_offset(address, cpu.index_register_y)
        }
        AddressingMode::AbsoluteX => cpu.fetch_byte_with_offset(address, cpu.index_register_x),
        _ => {
            cpu.cycle += 1;
            cpu.access_memory(address)
        }
    };

    match register {
        Register::Accumulator => cpu.accumulator = value,
        Register::IndexX => cpu.index_register_x = value,
        Register::IndexY => cpu.index_register_y = value,
    }
    cpu.set_load_status(&register);
}

pub fn lda_im(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Immediate, Register::Accumulator);
}

pub fn lda_zp(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPage, Register::Accumulator);
}

pub fn lda_zpx(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPageX, Register::Accumulator);
}

pub fn lda_a(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Absolute, Register::Accumulator);
}

pub fn lda_a_x(cpu: &mut CPU) {
    ld(cpu, AddressingMode::AbsoluteX, Register::Accumulator);
}

pub fn lda_a_y(cpu: &mut CPU) {
    ld(cpu, AddressingMode::AbsoluteY, Register::Accumulator);
}

pub fn lda_in_x(cpu: &mut CPU) {
    ld(cpu, AddressingMode::IndexIndirectX, Register::Accumulator);
}

pub fn lda_in_y(cpu: &mut CPU) {
    ld(cpu, AddressingMode::IndirectIndexY, Register::Accumulator);
}

pub fn ldy_im(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Immediate, Register::IndexY);
}

pub fn ldy_zp(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPage, Register::IndexY);
}

pub fn ldy_zpx(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPageX, Register::IndexY);
}

pub fn ldy_a(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Absolute, Register::IndexY);
}

pub fn ldy_a_x(cpu: &mut CPU) {
    ld(cpu, AddressingMode::AbsoluteX, Register::IndexY);
}

pub fn ldx_im(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Immediate, Register::IndexX);
}

pub fn ldx_zp(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPage, Register::IndexX);
}

pub fn ldx_zpy(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPageY, Register::IndexX);
}

pub fn ldx_a(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Absolute, Register::IndexX);
}

pub fn ldx_a_y(cpu: &mut CPU) {
    ld(cpu, AddressingMode::AbsoluteY, Register::IndexX);
}

pub fn jsr_a(cpu: &mut CPU) {
    let jump_addr_hi = cpu.fetch_zero_page_address();
    let jump_addr_lo: u16 = cpu.access_memory(cpu.program_counter).into();
    cpu.cycle += 1;

    cpu.push_word_to_stack(cpu.program_counter);

    cpu.program_counter = (jump_addr_lo << 8) | jump_addr_hi;
    cpu.cycle += 1;
}

pub fn rts(cpu: &mut CPU) {
    cpu.access_memory(cpu.program_counter); // fetch and discard
    cpu.cycle += 1;

    cpu.program_counter = cpu.pop_word_from_stack();
    cpu.cycle += 1;
    cpu.increment_program_counter();
}

pub fn jmp(cpu: &mut CPU, addr_mode: AddressingMode) {
    cpu.program_counter = cpu.get_address(&addr_mode);
}

pub fn jmp_a(cpu: &mut CPU) {
    jmp(cpu, AddressingMode::Absolute);
}

pub fn jmp_in(cpu: &mut CPU) {
    jmp(cpu, AddressingMode::Indirect);
}

pub fn beq(cpu: &mut CPU) {
    let operand = cpu.access_memory(cpu.program_counter);
    cpu.increment_program_counter();
    if !cpu.processor_status.get_zero_flag() {
        return;
    }

    let [program_counter_lo, program_counter_hi] = cpu.program_counter.to_le_bytes();
    let negative_offset_direction = 0b10000000 & operand > 0;
    let offset = 0b01111111 & operand;
    let offset_program_counter_lo: Byte;
    let carry: bool;

    if negative_offset_direction {
        (offset_program_counter_lo, carry) = program_counter_lo.overflowing_sub(offset);
    } else {
        (offset_program_counter_lo, carry) = program_counter_lo.overflowing_add(offset);
    }

    cpu.program_counter = Word::from_le_bytes([offset_program_counter_lo, program_counter_hi]);
    cpu.cycle += 1;
    if !carry {
        return;
    }

    let offset_program_counter_hi: Byte;
    if negative_offset_direction {
        offset_program_counter_hi = program_counter_hi.wrapping_sub(1);
    } else {
        offset_program_counter_hi = program_counter_hi.wrapping_add(1);
    }
    cpu.program_counter =
        Word::from_le_bytes([offset_program_counter_lo, offset_program_counter_hi]);
    cpu.cycle += 1;
}

#[cfg(test)]
mod tests;
