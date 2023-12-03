use crate::memory::Memory;

use super::cpu::CPU;

pub struct Machine {
    memory: Memory,
    cpu: CPU,
}

impl Machine {
    pub fn new() -> Self {
        let mut payload: [u8; 64 * 1024] = [0; 64 * 1024]; // change to Boxed array
        let mut memory: Memory = (&payload).into();
        memory.data[0xFFFC] = 0x4C;
        memory.data[0xFFFD] = 0x34;
        memory.data[0xFFFE] = 0x12;
        memory.data[0x1234] = 0xB5;
        memory.data[0x1235] = 0xAB;
        memory.data[0x00AB] = 0x42;
        memory.data[0x1236] = 0x20;
        memory.data[0x1237] = 0x00;
        memory.data[0x1238] = 0x03;
        memory.data[0x0300] = 0xA9;
        memory.data[0x0301] = 0xFF;

        let mut machine = Machine{
            memory,
            cpu: CPU::new()
        };

        payload[0xFFFC] = 0xA9;
        machine.cpu.reset();
        const CYCLES: u64 = 14;
        machine.cpu.execute(CYCLES, &mut machine.memory);

        return machine;
    }
}
