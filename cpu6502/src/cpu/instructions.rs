use super::{AddressingMode, MemoryModifications, Registers, CPU};

fn ld(cpu: &mut CPU, addr_mode: AddressingMode, register: Registers) {
    let value = match cpu.read_memory(addr_mode) {
        Some(value) => value,
        None => panic!("ld used with incorrect address mode"),
    };

    cpu.set_register(register, value);
    cpu.set_load_status(register);
}

pub fn lda_im(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Immediate, Registers::Accumulator);
}

pub fn lda_zp(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPage, Registers::Accumulator);
}

pub fn lda_zpx(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPageX, Registers::Accumulator);
}

pub fn lda_a(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Absolute, Registers::Accumulator);
}

pub fn lda_a_x(cpu: &mut CPU) {
    ld(cpu, AddressingMode::AbsoluteX, Registers::Accumulator);
}

pub fn lda_a_y(cpu: &mut CPU) {
    ld(cpu, AddressingMode::AbsoluteY, Registers::Accumulator);
}

pub fn lda_in_x(cpu: &mut CPU) {
    ld(cpu, AddressingMode::IndexIndirectX, Registers::Accumulator);
}

pub fn lda_in_y(cpu: &mut CPU) {
    ld(cpu, AddressingMode::IndirectIndexY, Registers::Accumulator);
}

pub fn ldy_im(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Immediate, Registers::IndexY);
}

pub fn ldy_zp(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPage, Registers::IndexY);
}

pub fn ldy_zpx(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPageX, Registers::IndexY);
}

pub fn ldy_a(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Absolute, Registers::IndexY);
}

pub fn ldy_a_x(cpu: &mut CPU) {
    ld(cpu, AddressingMode::AbsoluteX, Registers::IndexY);
}

pub fn ldx_im(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Immediate, Registers::IndexX);
}

pub fn ldx_zp(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPage, Registers::IndexX);
}

pub fn ldx_zpy(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPageY, Registers::IndexX);
}

pub fn ldx_a(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Absolute, Registers::IndexX);
}

pub fn ldx_a_y(cpu: &mut CPU) {
    ld(cpu, AddressingMode::AbsoluteY, Registers::IndexX);
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

fn jmp(cpu: &mut CPU, addr_mode: AddressingMode) {
    match cpu.get_address(addr_mode, super::MemoryOperation::Read) {
        Some(address) => cpu.program_counter = address,
        None => panic!("jmp used with incorrect addressing mode"),
    }
}

pub fn jmp_a(cpu: &mut CPU) {
    jmp(cpu, AddressingMode::Absolute);
}

pub fn jmp_in(cpu: &mut CPU) {
    jmp(cpu, AddressingMode::Indirect);
}

fn branch(cpu: &mut CPU, condition: fn(&CPU) -> bool) {
    let operand = cpu.access_memory(cpu.program_counter);
    cpu.increment_program_counter();
    if !condition(cpu) {
        return;
    }

    cpu.offset_program_counter(operand)
}

pub fn bcc(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return !cpu.processor_status.get_carry_flag();
    });
}

pub fn bcs(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return cpu.processor_status.get_carry_flag();
    });
}

pub fn bne(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return !cpu.processor_status.get_zero_flag();
    });
}

pub fn beq(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return cpu.processor_status.get_zero_flag();
    });
}

fn compare(cpu: &mut CPU, addr_mode: AddressingMode, register: Registers) {
    let value = match cpu.read_memory(addr_mode) {
        Some(value) => value,
        None => panic!("compare used with incorrect address mode"),
    };

    cpu.set_cmp_status(register, value);
}

pub fn cmp_im(cpu: &mut CPU) {
    compare(cpu, AddressingMode::Immediate, Registers::Accumulator);
}

pub fn cmp_zp(cpu: &mut CPU) {
    compare(cpu, AddressingMode::ZeroPage, Registers::Accumulator);
}

pub fn cmp_zpx(cpu: &mut CPU) {
    compare(cpu, AddressingMode::ZeroPageX, Registers::Accumulator);
}

pub fn cmp_a(cpu: &mut CPU) {
    compare(cpu, AddressingMode::Absolute, Registers::Accumulator);
}

pub fn cmp_a_x(cpu: &mut CPU) {
    compare(cpu, AddressingMode::AbsoluteX, Registers::Accumulator);
}

pub fn cmp_a_y(cpu: &mut CPU) {
    compare(cpu, AddressingMode::AbsoluteY, Registers::Accumulator);
}

pub fn cmp_in_x(cpu: &mut CPU) {
    compare(cpu, AddressingMode::IndexIndirectX, Registers::Accumulator);
}

pub fn cmp_in_y(cpu: &mut CPU) {
    compare(cpu, AddressingMode::IndirectIndexY, Registers::Accumulator);
}

pub fn cpx_im(cpu: &mut CPU) {
    compare(cpu, AddressingMode::Immediate, Registers::IndexX);
}

pub fn cpx_zp(cpu: &mut CPU) {
    compare(cpu, AddressingMode::ZeroPage, Registers::IndexX);
}

pub fn cpx_a(cpu: &mut CPU) {
    compare(cpu, AddressingMode::Absolute, Registers::IndexX);
}

pub fn cpy_im(cpu: &mut CPU) {
    compare(cpu, AddressingMode::Immediate, Registers::IndexY);
}

pub fn cpy_zp(cpu: &mut CPU) {
    compare(cpu, AddressingMode::ZeroPage, Registers::IndexY);
}

pub fn cpy_a(cpu: &mut CPU) {
    compare(cpu, AddressingMode::Absolute, Registers::IndexY);
}

fn decrement_register(cpu: &mut CPU, register: Registers) {
    match register {
        Registers::IndexX | Registers::IndexY => {
            cpu.decrement_register(register);
        }
        _ => panic!("decrement_register used with incorrect register"),
    }
}

pub fn dec_zp(cpu: &mut CPU) {
    cpu.modify_memory(AddressingMode::ZeroPage, MemoryModifications::Decrement);
}

pub fn dec_zpx(cpu: &mut CPU) {
    cpu.modify_memory(AddressingMode::ZeroPageX, MemoryModifications::Decrement);
}

pub fn dec_a(cpu: &mut CPU) {
    cpu.modify_memory(AddressingMode::Absolute, MemoryModifications::Decrement);
}

pub fn dec_a_x(cpu: &mut CPU) {
    cpu.modify_memory(AddressingMode::AbsoluteX, MemoryModifications::Decrement);
}

pub fn dex_im(cpu: &mut CPU) {
    decrement_register(cpu, Registers::IndexX);
}

pub fn dey_im(cpu: &mut CPU) {
    decrement_register(cpu, Registers::IndexY);
}

fn increment_register(cpu: &mut CPU, register: Registers) {
    match register {
        Registers::IndexX | Registers::IndexY => {
            cpu.increment_register(register);
        }
        _ => panic!("increment_register used with incorrect register"),
    }
}

pub fn inc_zp(cpu: &mut CPU) {
    cpu.modify_memory(AddressingMode::ZeroPage, MemoryModifications::Increment);
}

pub fn inc_zpx(cpu: &mut CPU) {
    cpu.modify_memory(AddressingMode::ZeroPageX, MemoryModifications::Increment);
}

pub fn inc_a(cpu: &mut CPU) {
    cpu.modify_memory(AddressingMode::Absolute, MemoryModifications::Increment);
}

pub fn inc_a_x(cpu: &mut CPU) {
    cpu.modify_memory(AddressingMode::AbsoluteX, MemoryModifications::Increment);
}

pub fn inx_im(cpu: &mut CPU) {
    increment_register(cpu, Registers::IndexX);
}

pub fn iny_im(cpu: &mut CPU) {
    increment_register(cpu, Registers::IndexY);
}

pub fn store(cpu: &mut CPU, addr_mode: AddressingMode, register: Registers) {
    let value = cpu.get_register(register);
    match cpu.write_memory(addr_mode, value) {
        Some(()) => (),
        None => panic!("store_in_memory used with incorrect address mode"),
    }
}

pub fn sta_zp(cpu: &mut CPU) {
    store(cpu, AddressingMode::ZeroPage, Registers::Accumulator);
}

pub fn sta_zpx(cpu: &mut CPU) {
    store(cpu, AddressingMode::ZeroPageX, Registers::Accumulator);
}

pub fn sta_a(cpu: &mut CPU) {
    store(cpu, AddressingMode::Absolute, Registers::Accumulator);
}

pub fn sta_a_x(cpu: &mut CPU) {
    store(cpu, AddressingMode::AbsoluteX, Registers::Accumulator);
}

pub fn sta_a_y(cpu: &mut CPU) {
    store(cpu, AddressingMode::AbsoluteY, Registers::Accumulator);
}

pub fn sta_in_x(cpu: &mut CPU) {
    store(cpu, AddressingMode::IndexIndirectX, Registers::Accumulator);
}

pub fn sta_in_y(cpu: &mut CPU) {
    store(cpu, AddressingMode::IndirectIndexY, Registers::Accumulator);
}

pub fn stx_zp(cpu: &mut CPU) {
    store(cpu, AddressingMode::ZeroPage, Registers::IndexX);
}

pub fn stx_zpy(cpu: &mut CPU) {
    store(cpu, AddressingMode::ZeroPageY, Registers::IndexX);
}

pub fn stx_a(cpu: &mut CPU) {
    store(cpu, AddressingMode::Absolute, Registers::IndexX);
}

pub fn sty_zp(cpu: &mut CPU) {
    store(cpu, AddressingMode::ZeroPage, Registers::IndexY);
}

pub fn sty_zpx(cpu: &mut CPU) {
    store(cpu, AddressingMode::ZeroPageX, Registers::IndexY);
}

pub fn sty_a(cpu: &mut CPU) {
    store(cpu, AddressingMode::Absolute, Registers::IndexY);
}

#[cfg(test)]
mod tests;
