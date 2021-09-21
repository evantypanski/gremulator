extern crate env_logger;
extern crate gremulator;
extern crate log;

use log::{info, trace};
use std::io::Error;

use gremulator::cpu::cpu::CPU;

fn main() -> Result<(), Error> {
    env_logger::init();
    info!("Gremulator successfully started");
    let mut cpu = CPU::new()?;
    while !cpu.halted {
        cpu.cycle();
        // Useful to debug for now.
        trace!("Register A after cycle: {}", cpu.get_a());
    }
    info!("Gremulator halted! Exiting...");
    Ok(())
}
