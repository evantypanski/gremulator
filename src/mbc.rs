use std::fs::File;
use std::io::{Error, Read};

pub struct MBC {
    rom: Vec<u8>,
}

impl MBC {
    pub fn new() -> Result<MBC, Error> {
        Ok(MBC {
            rom: Self::read_rom()?,
        })
    }

    pub fn set_mem_addr(&mut self, addr: u16, val: u8) {
        self.rom[addr as usize] = val;
    }

    pub fn fetch_rom(&self, addr: u16) -> u8 {
        //let mut rom = File::open("roms/test/ld.gb").unwrap();
        self.rom[addr as usize]
    }

    fn read_rom() -> Result<Vec<u8>, Error> {
        let mut file = File::open("roms/test/ld.gb").expect("Unable to open");
        let mut contents = vec![];
        file.read_to_end(&mut contents)?;
        Ok(contents)
    }
}
