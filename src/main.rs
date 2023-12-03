mod cpu;
mod machine;
mod memory;
mod consts;

fn main() {
    let machine = machine::Machine::new();
    println!("Hello, world!");
}
