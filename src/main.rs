use cpu5602::machine;

fn main() {
    let program: &[(u16, u8)] = &[
        (0xFFFC, 0x4C),
        (0xFFFD, 0x34),
        (0xFFFE, 0x12),
        (0x1234, 0xB5),
        (0x1235, 0xAB),
        (0x00AB, 0x42),
        (0x1236, 0x20),
        (0x1237, 0x00),
        (0x1238, 0x03),
        (0x0300, 0xA9),
        (0x0301, 0xFF)
    ];
    let mut machine = machine::Machine::new();
    let cycles = 14;
    machine.execute_cycles(program, cycles);
}
