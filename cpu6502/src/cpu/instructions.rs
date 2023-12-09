use crate::consts::Word;
use super::CPU;

pub fn lda_im(cpu: &mut CPU) {
    cpu.accumulator = cpu.fetch_byte();
    cpu.set_load_accumulator_status();
}

pub fn lda_zp(cpu: &mut CPU) {
    let address: Word = cpu.fetch_byte().into();
    cpu.accumulator = cpu.access_memory(address);
    cpu.set_load_accumulator_status();
}

pub fn lda_zpx(cpu: &mut CPU) {
    let zero_page_addr = cpu.fetch_byte();
    let final_addr: Word = cpu.sum_with_x(zero_page_addr).into();
    cpu.accumulator = cpu.access_memory(final_addr);
    cpu.set_load_accumulator_status();
}

pub fn lda_a(cpu: &mut CPU) {
    cpu.set_load_accumulator_status();
}

pub fn jsr_a(cpu: &mut CPU) {
    let jump_addr = cpu.fetch_word();
    let saved_return_addr = cpu.program_counter - 1;
    cpu.cycle += 1;
    cpu.push_word_to_stack(saved_return_addr);
    cpu.program_counter = jump_addr;
}

pub fn jmp_a(cpu: &mut CPU) {
    cpu.program_counter = cpu.fetch_word();
}
