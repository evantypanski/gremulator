#![crate_name = "gremulator"]

pub mod cpu;
mod register;
mod mmu;
mod mbc;
mod alu;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let mut cpu = ::cpu::CPU::new();
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn return_cpu() -> ::cpu::CPU {
    ::cpu::CPU::new()
}
