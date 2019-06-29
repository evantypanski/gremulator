use std::fs::File;
use std::io::Read;

pub struct MBC {
    rom: Vec<u8>,
}

impl MBC {
    pub fn new() -> MBC {
        MBC {
            rom: Self::read_rom(),
        }
    }

    pub fn fetch_rom(&self, addr: u16) -> u8 {
        //let mut rom = File::open("roms/test/ld.gb").unwrap();
        self.rom[addr as usize]
    }

    fn read_rom() -> Vec<u8> {
        /*let mut file = File::open("roms/test/add_num.gb").expect("Unable to open");
        let mut contents = vec![];
        file.read_to_end(&mut contents);
        contents*/
        vec![]
    }
}
