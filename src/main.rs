extern crate gremulator;

use gremulator::cpu::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();
    while cpu.cycle() != 66 {
        println!("{}", cpu.get_a());
    }
}
