use std::io::Error;

pub struct MMU {
    mbc: ::mbc::MBC,
}

impl MMU {
    pub fn new() -> Result<MMU, Error> {
        Ok(MMU {
            mbc: ::mbc::MBC::new()?,
        })
    }

    pub fn fetch(&self, addr: u16) -> u8 {
        self.mbc.fetch_rom(addr)
    }

    pub fn set_mem_addr(&mut self, addr: u16, val: u8) {
        self.mbc.set_mem_addr(addr, val);
    }
}
