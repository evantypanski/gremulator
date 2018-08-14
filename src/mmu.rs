pub struct MMU {
    a: u8,
    mbc: ::mbc::MBC,
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            a: 0,
            mbc: ::mbc::MBC::new(),
        }
    }

    pub fn fetch(&self, addr: u16) -> u8 {
        self.mbc.fetch_rom(addr)
    }
}
