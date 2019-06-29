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

static Z_FLAG_OFFSET: u8 = 7;
static N_FLAG_OFFSET: u8 = 6;
static H_FLAG_OFFSET: u8 = 5;
static C_FLAG_OFFSET: u8 = 4;

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

    pub fn z_flag(&mut self, set: bool) {
        self.flag(set, Z_FLAG_OFFSET);
    }

    pub fn n_flag(&mut self, set: bool) {
        self.flag(set, N_FLAG_OFFSET);
    }

    pub fn h_flag(&mut self, set: bool) {
        self.flag(set, H_FLAG_OFFSET);
    }

    pub fn c_flag(&mut self, set: bool) {
        self.flag(set, C_FLAG_OFFSET);
    }

    fn flag(&mut self, set: bool, offset: u8) {
        if (set) { self.f |= (1 << offset);  }
        else     { self.f &= !(1 << offset); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_set_properly() {
        let mut reg = Registers::new();
        reg.f = 0b01010000;
        reg.z_flag(true);
        assert_eq!(format!("{:b}", reg.f), "11010000");

        reg.n_flag(false);
        assert_eq!(format!("{:b}", reg.f), "10010000");

        reg.h_flag(true);
        assert_eq!(format!("{:b}", reg.f), "10110000");

        reg.c_flag(false);
        assert_eq!(format!("{:b}", reg.f), "10100000");
    }
}
