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

#[cfg(test)]
mod tests;