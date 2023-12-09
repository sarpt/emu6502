#[cfg(test)]
mod lda_im {
    use crate::cpu::tests::MemoryMock;
    use super::super::*;

    #[test]
    fn should_fetch_byte_pointed_by_program_counter_into_accumulator() {
        let mut cpu = CPU::new(Box::new(MemoryMock::default()));
        cpu.program_counter = 0x00;
        assert_eq!(cpu.accumulator, 0x0);

        lda_im(&mut cpu);

        assert_eq!(cpu.accumulator, 0x44);
    }

    #[test]
    fn should_set_load_accumulator_processor_status() {
        let mut cpu = CPU::new(Box::new(MemoryMock::default()));
        cpu.program_counter = 0x04;

        lda_im(&mut cpu);

        assert_eq!(cpu.processor_status.flags, 0b10000000);
    }

    fn should_take_one_cycle() {
        let mut cpu = CPU::new(Box::new(MemoryMock::default()));
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        lda_im(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }
}

#[cfg(test)]
mod lda_zp {
    use crate::cpu::tests::MemoryMock;
    use super::super::*;

    #[test]
    fn should_fetch_byte_from_a_zero_page_address_stored_in_a_place_pointed_by_program_counter_into_accumulator() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03,0xFF,0x00,0x45])));
        cpu.program_counter = 0x00;

        lda_zp(&mut cpu);

        assert_eq!(cpu.accumulator, 0x45);
    }

    #[test]
    fn should_set_load_accumulator_processor_status() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03,0xFF,0x00,0xFF])));
        cpu.program_counter = 0x00;

        lda_zp(&mut cpu);

        assert_eq!(cpu.processor_status.flags, 0b10000000);
    }

    fn should_take_two_cycles() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03,0xFF,0x00,0x05])));
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        lda_zp(&mut cpu);

        assert_eq!(cpu.cycle, 2);
    }
}

#[cfg(test)]
mod lda_zpx {
    use crate::cpu::tests::MemoryMock;
    use super::super::*;

    #[test]
    fn should_fetch_byte_from_an_address_stored_in_program_counter_pointed_place_summed_with_index_register_x_into_accumulator() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01,0x00,0x00,0x55])));
        cpu.index_register_x = 0x02;
        cpu.program_counter = 0x00;

        lda_zpx(&mut cpu);

        assert_eq!(cpu.accumulator, 0x55);
    }

    fn should_overflow_over_byte_when_summing_address_from_memory_with_register_x() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0xFF,0x88,0x00])));
        cpu.index_register_x = 0x02;
        cpu.program_counter = 0x00;

        lda_zpx(&mut cpu);

        assert_eq!(cpu.accumulator, 0x88);
    }

    #[test]
    fn should_set_load_accumulator_processor_status() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01,0x00,0x00,0xFF])));
        cpu.index_register_x = 0x02;
        cpu.program_counter = 0x00;

        lda_zpx(&mut cpu);

        assert_eq!(cpu.processor_status.flags, 0b10000000);
    }

    fn should_take_three_cycles() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01,0x00,0x00,0x55])));
        cpu.index_register_x = 0x02;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        lda_zpx(&mut cpu);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod lda_a {
    use crate::cpu::tests::MemoryMock;
    use super::super::*;

    #[test]
    fn should_fetch_byte_from_an_absolute_address_stored_in_a_place_pointed_by_program_counter_into_accumulator() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03,0x00,0x00,0x45])));
        cpu.program_counter = 0x00;

        lda_a(&mut cpu);

        assert_eq!(cpu.accumulator, 0x45);
    }

    #[test]
    fn should_set_load_accumulator_processor_status() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03,0x00,0x00,0xFF])));
        cpu.program_counter = 0x00;

        lda_a(&mut cpu);

        assert_eq!(cpu.processor_status.flags, 0b10000000);
    }

    fn should_take_three_cycles() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03,0x00,0x00,0x05])));
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        lda_zp(&mut cpu);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod jsr_a {
    use crate::cpu::tests::MemoryMock;
    use super::super::*;

    #[test]
    fn should_fetch_address_pointed_by_program_counter_and_put_in_program_counter() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x44, 0x51, 0x88])));
        cpu.program_counter = 0x00;

        jsr_a(&mut cpu);

        assert_eq!(cpu.program_counter, 0x5144);
    }

    fn should_save_program_counter_after_fetching_new_adress_minus_one_into_stack_pointer() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x44, 0x51, 0x88])));
        cpu.stack_pointer = 0x00;

        jsr_a(&mut cpu);

        assert_eq!(cpu.stack_pointer, 0x02);
    }

    #[test]
    fn should_take_six_cycles() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x44, 0x51, 0x88])));
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        jsr_a(&mut cpu);

        assert_eq!(cpu.cycle, 5);
    }
}

#[cfg(test)]
mod jmp_a {
    use crate::cpu::tests::MemoryMock;
    use super::super::*;

    #[test]
    fn should_put_address_stored_in_memory_at_program_counter_as_a_new_program_counter() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x44, 0x51, 0x88])));
        cpu.program_counter = 0x00;

        jmp_a(&mut cpu);

        assert_eq!(cpu.program_counter, 0x5144);
    }

    #[test]
    fn should_take_two_cycles() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x44, 0x51, 0x88])));
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        jmp_a(&mut cpu);

        assert_eq!(cpu.cycle, 2);
    }
}