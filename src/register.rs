#[derive(Copy, Clone)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub f: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 5,
            b: 9,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 1,
            pc: 0,
            sp: 0xFFFE,
            f: 0,
        }
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) + self.l as u16
    }
}
