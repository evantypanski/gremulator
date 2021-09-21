use std::convert::TryInto;

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

    // Include set functions to make things easier for combined registers.
    pub fn set_bc(&mut self, num: u16) {
        let b_num: u8 = (num >> 8).try_into().unwrap();
        self.b = b_num;
        let c_num: u8 = (num & 0xFF).try_into().unwrap();
        self.c = c_num;
    }

    pub fn set_de(&mut self, num: u16) {
        let d_num: u8 = (num >> 8).try_into().unwrap();
        self.d = d_num;
        let e_num: u8 = (num & 0xFF).try_into().unwrap();
        self.e = e_num;
    }

    pub fn set_hl(&mut self, num: u16) {
        let h_num: u8 = (num >> 8).try_into().unwrap();
        self.h = h_num;
        let l_num: u8 = (num & 0xFF).try_into().unwrap();
        self.l = l_num;
    }
}
