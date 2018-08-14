extern crate gremulator;

fn main() {
    let mut cpu = gremulator::cpu::CPU::new();
    while cpu.cycle() != 66 {
        println!("{}", cpu.get_a());
    }
}
