use super::{AddressingMode, Register, CPU};

fn ld(cpu: &mut CPU, addr_mode: AddressingMode, register: Register) {
    let address = match cpu.get_address(&addr_mode) {
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

fn jmp(cpu: &mut CPU, addr_mode: AddressingMode) {
    match cpu.get_address(&addr_mode) {
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

#[cfg(test)]
mod tests;
