use crate::memory::VecMemory;

use super::cpu::CPU;

pub struct Machine {
    memory: VecMemory,
    cpu: CPU,
}

impl Machine {
    pub fn new() -> Self {
        return Machine{
            memory: VecMemory::new(),
            cpu: CPU::new()
        };
    }

    pub fn execute_cycles(&mut self, program: &[(u16, u8)], cycles: u64) {
        self.memory.store(program);
        self.cpu.execute(cycles, &mut self.memory);
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }
}
