use super::{AddressingMode, Registers, CPU};

fn ld(cpu: &mut CPU, addr_mode: AddressingMode, register: Registers) {
    let address = match cpu.get_address(addr_mode) {
        Some(address) => address,
        None => panic!("ld used with incorrect address mode"),
    };

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
    match cpu.get_address(addr_mode) {
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
    let address = match cpu.get_address(addr_mode) {
        Some(address) => address,
        None => panic!("compare used with incorrect address mode"),
    };

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

fn increment(cpu: &mut CPU, addr_mode: AddressingMode, register: Registers) {
    match register {
        Registers::IndexX | Registers::IndexY => {
            cpu.increment_register(register);
            cpu.cycle += 1;
        }
        Registers::Accumulator => {
            let address = match cpu.get_address(addr_mode) {
                Some(address) => address,
                None => panic!("accumulator increment used with incorrect address mode"),
            };
        }
        _ => panic!("increment used with incorrect register"),
    }
}

pub fn inc_zp(cpu: &mut CPU) {
    increment(cpu, AddressingMode::ZeroPage, Registers::Accumulator);
}

pub fn inc_zpx(cpu: &mut CPU) {
    increment(cpu, AddressingMode::ZeroPageX, Registers::Accumulator);
}

pub fn inc_a_x(cpu: &mut CPU) {
    increment(cpu, AddressingMode::AbsoluteX, Registers::Accumulator);
}

pub fn inc_a_y(cpu: &mut CPU) {
    increment(cpu, AddressingMode::AbsoluteY, Registers::Accumulator);
}

pub fn inx(cpu: &mut CPU) {
    increment(cpu, AddressingMode::Implicit, Registers::IndexY);
}

pub fn iny(cpu: &mut CPU) {
    increment(cpu, AddressingMode::Implicit, Registers::IndexY);
}

#[cfg(test)]
mod tests;
