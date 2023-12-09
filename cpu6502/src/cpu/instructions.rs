use crate::consts::Word;
use super::CPU;

pub fn lda_im(cpu: &mut CPU) {
    cpu.accumulator = cpu.fetch_byte();
    cpu.set_load_accumulator_status();
}

pub fn lda_zp(cpu: &mut CPU) {
    let address: Word = cpu.fetch_zero_page_address();
    cpu.accumulator = cpu.access_memory(address);
    cpu.set_load_accumulator_status();
}

pub fn lda_zpx(cpu: &mut CPU) {
    let address: Word = cpu.fetch_zero_page_with_x_offset();
    cpu.accumulator = cpu.access_memory(address);
    cpu.set_load_accumulator_status();
}

pub fn lda_a(cpu: &mut CPU) {
    let address = cpu.fetch_address();
    cpu.accumulator = cpu.access_memory(address);
    cpu.set_load_accumulator_status();
}

pub fn jsr_a(cpu: &mut CPU) {
    let jump_addr = cpu.fetch_address();
    let saved_return_addr = cpu.program_counter - 1;
    cpu.cycle += 1;
    cpu.push_word_to_stack(saved_return_addr);
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
        cpu.program_counter = address_of_jmp_address;
        let lsb: Word= cpu.fetch_byte().into();
        cpu.program_counter = address_of_jmp_address & 0x1100;
        let msb: Word = cpu.fetch_byte().into();
        let incorrect_jmp_address = (msb << 8) | lsb;

        cpu.program_counter = incorrect_jmp_address;
        return
    }

    let jmp_address = cpu.fetch_address_from(address_of_jmp_address);
    cpu.program_counter = jmp_address;
}

#[cfg(test)]
mod tests;