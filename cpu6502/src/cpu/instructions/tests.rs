#[cfg(test)]
mod lda {
    #[cfg(test)]
    mod lda_im {
        use crate::cpu::{instructions::lda_im, tests::MemoryMock, CPU};

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

        #[test]
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
        use crate::cpu::{instructions::lda_zp, tests::MemoryMock, CPU};

        #[test]
        fn should_fetch_byte_from_a_zero_page_address_stored_in_a_place_pointed_by_program_counter_into_accumulator(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0x45])));
            cpu.program_counter = 0x00;
            assert_eq!(cpu.accumulator, 0x0);

            lda_zp(&mut cpu);

            assert_eq!(cpu.accumulator, 0x45);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0xFF])));
            cpu.program_counter = 0x00;

            lda_zp(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0x05])));
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            lda_zp(&mut cpu);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod lda_zpx {
        use crate::cpu::{instructions::lda_zpx, tests::MemoryMock, CPU};

        #[test]
        fn should_fetch_byte_from_an_address_stored_in_program_counter_pointed_place_summed_with_index_register_x_into_accumulator(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x00, 0x00, 0x55])));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.accumulator, 0x0);

            lda_zpx(&mut cpu);

            assert_eq!(cpu.accumulator, 0x55);
        }

        fn should_overflow_over_byte_when_summing_address_from_memory_with_register_x() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0xFF, 0x88, 0x00])));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            lda_zpx(&mut cpu);

            assert_eq!(cpu.accumulator, 0x88);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x00, 0x00, 0xFF])));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            lda_zpx(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x00, 0x00, 0x55])));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            lda_zpx(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod lda_a {
        use crate::cpu::{instructions::lda_a, tests::MemoryMock, CPU};

        #[test]
        fn should_fetch_byte_from_an_absolute_address_stored_in_a_place_pointed_by_program_counter_into_accumulator(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0x45])));
            cpu.program_counter = 0x00;
            assert_eq!(cpu.accumulator, 0x0);

            lda_a(&mut cpu);

            assert_eq!(cpu.accumulator, 0x45);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0xFF])));
            cpu.program_counter = 0x00;

            lda_a(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0x05])));
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            lda_a(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod lda_a_x {
        use crate::{
            consts::Byte,
            cpu::{instructions::lda_a_x, tests::MemoryMock, CPU},
        };

        const ADDRESS_LSB_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_LSB: Byte = 0x03;
        const ADDRESS_MSB: Byte = 0x00;
        const VALUE: Byte = 0xDB;

        #[test]
        fn should_fetch_byte_from_an_absolute_address_offset_by_index_register_x_into_accumulator()
        {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LSB,
                ADDRESS_MSB,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            assert_eq!(cpu.accumulator, 0x0);

            lda_a_x(&mut cpu);

            assert_eq!(cpu.accumulator, VALUE);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LSB,
                ADDRESS_MSB,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;

            lda_a_x(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LSB,
                ADDRESS_MSB,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            cpu.cycle = 0;

            lda_a_x(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LSB_ON_ZERO_PAGE_BOUNDARY,
                ADDRESS_MSB,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            cpu.cycle = 0;

            lda_a_x(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod lda_a_y {
        use crate::{
            consts::Byte,
            cpu::{instructions::lda_a_y, tests::MemoryMock, CPU},
        };

        const ADDRESS_LSB_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_LSB: Byte = 0x03;
        const ADDRESS_MSB: Byte = 0x00;
        const VALUE: Byte = 0xDB;

        #[test]
        fn should_fetch_byte_from_an_absolute_address_offset_by_index_register_y_into_accumulator()
        {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LSB,
                ADDRESS_MSB,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            assert_eq!(cpu.accumulator, 0x0);

            lda_a_y(&mut cpu);

            assert_eq!(cpu.accumulator, VALUE);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LSB,
                ADDRESS_MSB,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;

            lda_a_y(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LSB,
                ADDRESS_MSB,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            cpu.cycle = 0;

            lda_a_y(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LSB_ON_ZERO_PAGE_BOUNDARY,
                ADDRESS_MSB,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            cpu.cycle = 0;

            lda_a_y(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod lda_in_y {
        use crate::{
            consts::Byte,
            cpu::{instructions::lda_in_y, tests::MemoryMock, CPU},
        };

        const INDIRECT_ZERO_PAGE_ADDRESS_PLACE: Byte = 0x01;
        const ADDRESS_LSB: Byte = 0x03;
        const ADDRESS_LSB_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_MSB: Byte = 0x00;
        const VALUE: Byte = 0xDB;

        #[test]
        fn should_fetch_byte_from_an_indirect_adress_stored_in_memory_at_zero_page_and_offset_with_value_from_index_register_y(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LSB,
                ADDRESS_MSB,
                0x45,
                0xAF,
                VALUE,
            ])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.accumulator, 0x0);

            lda_in_y(&mut cpu);

            assert_eq!(cpu.accumulator, VALUE);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LSB,
                ADDRESS_MSB,
                0x45,
                0xAF,
                VALUE,
            ])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            lda_in_y(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_four_cycles_when_summing_indirect_address_with_index_y_does_not_cross_page_flip(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LSB,
                ADDRESS_MSB,
                0x45,
                0xAF,
                VALUE,
            ])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            lda_in_y(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }

        #[test]
        fn should_take_five_cycles_when_summing_indirect_address_with_index_y_crosses_page_flip() {
            let mut memory: [Byte; 512] = [0x00; 512];
            memory[0x0000] = INDIRECT_ZERO_PAGE_ADDRESS_PLACE;
            memory[0x0001] = ADDRESS_LSB_ON_ZERO_PAGE_BOUNDARY;
            memory[0x0002] = ADDRESS_MSB;
            memory[0x0101] = VALUE;

            let mut cpu = CPU::new(Box::new(MemoryMock::new(&memory)));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            lda_in_y(&mut cpu);

            assert_eq!(cpu.cycle, 5);
        }
    }
}

#[cfg(test)]
mod ldx {
    #[cfg(test)]
    mod ldx_im {
        use crate::cpu::{instructions::ldx_im, tests::MemoryMock, CPU};

        #[test]
        fn should_fetch_byte_pointed_by_program_counter_into_index_register_x() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_x, 0x0);

            ldx_im(&mut cpu);

            assert_eq!(cpu.index_register_x, 0x44);
        }

        #[test]
        fn should_set_load_index_register_x_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.program_counter = 0x04;

            ldx_im(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ldx_im(&mut cpu);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod ldx_zp {
        use crate::cpu::{instructions::ldx_zp, tests::MemoryMock, CPU};

        #[test]
        fn should_fetch_byte_from_a_zero_page_address_stored_in_a_place_pointed_by_program_counter_into_index_register_x(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0x45])));
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_x, 0x0);

            ldx_zp(&mut cpu);

            assert_eq!(cpu.index_register_x, 0x45);
        }

        #[test]
        fn should_set_load_index_register_x_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0xFF])));
            cpu.program_counter = 0x00;

            ldx_zp(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0x05])));
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ldx_zp(&mut cpu);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod ldx_zpy {
        use crate::cpu::{instructions::ldx_zpy, tests::MemoryMock, CPU};

        #[test]
        fn should_fetch_byte_from_an_address_stored_in_program_counter_pointed_place_summed_with_index_register_y_into_index_register_x(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x00, 0x00, 0x55])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_x, 0x0);

            ldx_zpy(&mut cpu);

            assert_eq!(cpu.index_register_x, 0x55);
        }

        fn should_overflow_over_byte_when_summing_address_from_memory_with_register_y() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0xFF, 0x88, 0x00])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            ldx_zpy(&mut cpu);

            assert_eq!(cpu.accumulator, 0x88);
        }

        #[test]
        fn should_set_load_index_register_x_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x00, 0x00, 0xFF])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            ldx_zpy(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x00, 0x00, 0x55])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ldx_zpy(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod ldx_a {
        use crate::cpu::{instructions::ldx_a, tests::MemoryMock, CPU};

        #[test]
        fn should_fetch_byte_from_an_absolute_address_stored_in_a_place_pointed_by_program_counter_into_index_register_x(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0x45])));
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_x, 0x0);

            ldx_a(&mut cpu);

            assert_eq!(cpu.index_register_x, 0x45);
        }

        #[test]
        fn should_set_load_index_register_x_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0xFF])));
            cpu.program_counter = 0x00;

            ldx_a(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0x05])));
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ldx_a(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod ldx_a_y {
        use crate::{
            consts::Byte,
            cpu::{instructions::ldx_a_y, tests::MemoryMock, CPU},
        };

        const ADDRESS_LSB_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_LSB: Byte = 0x03;
        const ADDRESS_MSB: Byte = 0x00;
        const VALUE: Byte = 0xDB;

        #[test]
        fn should_fetch_byte_from_an_absolute_address_offset_by_index_register_y_into_index_register_x(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LSB,
                ADDRESS_MSB,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            assert_eq!(cpu.index_register_x, 0x0);

            ldx_a_y(&mut cpu);

            assert_eq!(cpu.index_register_x, VALUE);
        }

        #[test]
        fn should_set_load_index_register_x_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LSB,
                ADDRESS_MSB,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;

            ldx_a_y(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LSB,
                ADDRESS_MSB,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            cpu.cycle = 0;

            ldx_a_y(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LSB_ON_ZERO_PAGE_BOUNDARY,
                ADDRESS_MSB,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            cpu.cycle = 0;

            ldx_a_y(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }
    }
}

#[cfg(test)]
mod jsr_a {
    use super::super::*;
    use crate::cpu::tests::MemoryMock;

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
    use super::super::*;
    use crate::cpu::tests::MemoryMock;

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

#[cfg(test)]
mod jmp_in {
    use super::super::*;
    use crate::cpu::tests::MemoryMock;

    #[test]
    fn should_fetch_indirect_address_from_memory_and_put_in_program_counter() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
        cpu.program_counter = 0x00;

        jmp_in(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_four_cycles() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        jmp_in(&mut cpu);

        assert_eq!(cpu.cycle, 4);
    }

    #[test]
    fn should_incorrectly_interpret_address_pointed_to_by_program_counter_and_take_lsb_from_correct_address_but_wrap_around_page_for_msb(
    ) {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0xFF, 0x00, 0x04, 0x00])));
        cpu.program_counter = 0x00;

        jmp_in(&mut cpu);

        assert_eq!(cpu.program_counter, 0xFF00);
    }
}
