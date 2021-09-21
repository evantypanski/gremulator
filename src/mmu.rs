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
}
