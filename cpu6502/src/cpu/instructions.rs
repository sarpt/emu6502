use super::{AddressingMode, Register, CPU};
use crate::consts::Word;

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
    // TODO: this one is incorrect, stack should decrement not incremenet
    let jump_addr = cpu.fetch_address();
    cpu.decrement_program_counter();
    cpu.push_word_to_stack(cpu.program_counter);
    cpu.program_counter = jump_addr;
}

pub fn jmp_a(cpu: &mut CPU) {
    cpu.program_counter = cpu.fetch_address();
}

pub fn jmp_in(cpu: &mut CPU) {
    let address_of_jmp_address = cpu.fetch_address();
    // 6502 had a bug in indirect jump when indirect address was on a page flip
    // The bug was fixed with 65SC02
    // http://www.6502.org/users/obelisk/6502/reference.html#JMP
    // TODO: maybe make this an optional behavior
    let should_incorrectly_jump = address_of_jmp_address & 0x00FF == 0x00FF;
    if should_incorrectly_jump {
        let lsb: Word = cpu.access_memory(address_of_jmp_address).into();
        let msb: Word = cpu.access_memory(address_of_jmp_address & 0x1100).into();
        let incorrect_jmp_address = (msb << 8) | lsb;

        cpu.program_counter = incorrect_jmp_address;
        return;
    }

    cpu.program_counter = cpu.fetch_address_from(address_of_jmp_address);
}

#[cfg(test)]
mod tests;
