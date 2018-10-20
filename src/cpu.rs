pub struct CPU {
    registers: ::register::Registers,
    mmu:       ::mmu::MMU,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: ::register::Registers::new(),
            mmu:       ::mmu::MMU::new(),
        }
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.mmu.fetch(self.registers.pc);     // Fetch byte before incrementing pc
        self.registers.pc += 1;
        byte
    }

    pub fn get_a(&self) -> u8 {
        self.registers.a
    }

    pub fn get_b(&self) -> u8 {
        self.registers.b
    }

    pub fn get_c(&self) -> u8 {
        self.registers.c
    }

    pub fn cycle(&mut self) -> u8 {
        let opcode = self.fetch_byte();
        self.ops(opcode)
    }

    // ALU

    fn add(&mut self, val: u8) {
        self.registers.a = self.registers.a + val;
    }

    fn sub(&mut self, val: u8) {
        self.registers.a = self.registers.a - val
    }

    fn and(&mut self, val: u8) {
        self.registers.a = self.registers.a & val
    }

    fn or(&mut self, val: u8) {
        self.registers.a = self.registers.a | val
    }

    fn xor(&mut self, val: u8) {
        self.registers.a = self.registers.a ^ val
    }

    fn cp(&mut self, val: u8) {
        // TODO: Manipulate flags
    }

    // All operations

    pub fn ops(&mut self, opcode: u8) -> u8 {
        // Copy registers to avoid immutable/mutable reference with ALU ops
        let copy_registers = self.registers;
        match opcode {
            0x06 => { self.registers.b = self.fetch_byte(); 1 },    // LD B,n
            0x0E => { self.registers.c = self.fetch_byte(); 1 },    // LD C,n
            0x16 => { self.registers.d = self.fetch_byte(); 1 },    // LD D,n
            0x1E => { self.registers.e = self.fetch_byte(); 1 },    // LD E,n
            0x26 => { self.registers.h = self.fetch_byte(); 1 },    // LD H,n
            0x2E => { self.registers.l = self.fetch_byte(); 1 },    // LD L,n
            0x36 => { self.registers.h = self.registers.l; 1  },    // IMPLEMENT LD (HL),n
            0x40 => { self.registers.b = self.registers.b; 1  },    // LD B,B
            0x41 => { self.registers.b = self.registers.c; 1  },    // LD B,C
            0x42 => { self.registers.b = self.registers.d; 1  },    // LD B,D
            0x43 => { self.registers.b = self.registers.e; 1  },    // LD B,E
            0x44 => { self.registers.b = self.registers.h; 1  },    // LD B,H
            0x45 => { self.registers.b = self.registers.l; 1  },    // LD B,L
            0x46 => { self.registers.b = self.registers.l; 1  },    // IMPLEMENT LD B,(HL)
            0x48 => { self.registers.c = self.registers.b; 1  },    // LD C,B
            0x49 => { self.registers.c = self.registers.c; 1  },    // LD C,C
            0x4A => { self.registers.c = self.registers.d; 1  },    // LD C,D
            0x4B => { self.registers.c = self.registers.e; 1  },    // LD C,E
            0x4C => { self.registers.c = self.registers.h; 1  },    // LD C,H
            0x4D => { self.registers.c = self.registers.l; 1  },    // LD C,L
            0x4E => { self.registers.c = self.registers.l; 1  },    // IMPLEMENT LD C,(HL)
            0x50 => { self.registers.d = self.registers.b; 1  },    // LD D,B
            0x51 => { self.registers.d = self.registers.c; 1  },    // LD D,C
            0x52 => { self.registers.d = self.registers.d; 1  },    // LD D,D
            0x53 => { self.registers.d = self.registers.e; 1  },    // LD D,E
            0x54 => { self.registers.d = self.registers.h; 1  },    // LD D,H
            0x55 => { self.registers.d = self.registers.l; 1  },    // LD D,L
            0x56 => { self.registers.d = self.registers.l; 1  },    // IMPLEMENT LD D,(HL)
            0x58 => { self.registers.e = self.registers.b; 1  },    // LD E,B
            0x59 => { self.registers.e = self.registers.c; 1  },    // LD E,C
            0x5A => { self.registers.e = self.registers.d; 1  },    // LD E,D
            0x5B => { self.registers.e = self.registers.e; 1  },    // LD E,E
            0x5C => { self.registers.e = self.registers.h; 1  },    // LD E,H
            0x5D => { self.registers.e = self.registers.l; 1  },    // LD E,L
            0x5E => { self.registers.e = self.registers.l; 1  },    // IMPLEMENT LD E,(HL)
            0x60 => { self.registers.h = self.registers.b; 1  },    // LD H,B
            0x61 => { self.registers.h = self.registers.c; 1  },    // LD H,C
            0x62 => { self.registers.h = self.registers.d; 1  },    // LD H,D
            0x63 => { self.registers.h = self.registers.e; 1  },    // LD H,E
            0x64 => { self.registers.h = self.registers.h; 1  },    // LD H,H
            0x65 => { self.registers.h = self.registers.l; 1  },    // LD H,L
            0x66 => { self.registers.h = self.registers.l; 1  },    // IMPLEMENT LD H,(HL)
            0x68 => { self.registers.l = self.registers.b; 1  },    // LD L,B
            0x69 => { self.registers.l = self.registers.c; 1  },    // LD L,C
            0x6A => { self.registers.l = self.registers.d; 1  },    // LD L,D
            0x6B => { self.registers.l = self.registers.e; 1  },    // LD L,E
            0x6C => { self.registers.l = self.registers.h; 1  },    // LD L,H
            0x6D => { self.registers.l = self.registers.l; 1  },    // LD L,L
            0x6E => { self.registers.l = self.registers.l; 1  },    // IMPLEMENT LD L,(HL)
            0x70 => { self.registers.h = self.registers.b; 1  },    // IMPLEMENT LD (HL),B
            0x71 => { self.registers.h = self.registers.c; 1  },    // IMPLEMENT LD (HL),C
            0x72 => { self.registers.h = self.registers.d; 1  },    // IMPLEMENT LD (HL),D
            0x73 => { self.registers.h = self.registers.e; 1  },    // IMPLEMENT LD (HL),E
            0x74 => { self.registers.h = self.registers.h; 1  },    // IMPLEMENT LD (HL),H
            0x75 => { self.registers.h = self.registers.l; 1  },    // IMPLEMENT LD (HL),L
            0x76 => { 66 },                                         // IMPLEMENT HALT
            0x78 => { self.registers.a = self.registers.b; 1  },    // LD A,B
            0x79 => { self.registers.a = self.registers.c; 1  },    // LD A,C
            0x7A => { self.registers.a = self.registers.d; 1  },    // LD A,D
            0x7B => { self.registers.a = self.registers.e; 1  },    // LD A,E
            0x7C => { self.registers.a = self.registers.h; 1  },    // LD A,H
            0x7D => { self.registers.a = self.registers.l; 1  },    // LD A,L
            0x7E => { self.registers.a = self.registers.l; 1  },    // IMPLEMENT LD A,(HL)
            0x7F => { self.registers.a = self.registers.a; 1  },    // LD A,A
            0x80 => { self.add(copy_registers.b); 1 },              // ADD A,B
            0x81 => { self.add(copy_registers.c); 1 },              // ADD A,C
            0x82 => { self.add(copy_registers.d); 1 },              // ADD A,D
            0x83 => { self.add(copy_registers.e); 1 },              // ADD A,E
            0x84 => { self.add(copy_registers.h); 1 },              // ADD A,H
            0x85 => { self.add(copy_registers.l); 1 },              // ADD A,L
            0x86 => { let mem_val = self.mmu.fetch(copy_registers.hl()); self.add(mem_val); 1 },    // ADD A,[HL]
            0x87 => { self.add(copy_registers.a); 1 },              // ADD A,A
            0x90 => { self.sub(copy_registers.b); 1 },              // SUB A,B
            0x91 => { self.sub(copy_registers.c); 1 },              // SUB A,C
            0x92 => { self.sub(copy_registers.d); 1 },              // SUB A,D
            0x93 => { self.sub(copy_registers.e); 1 },              // SUB A,E
            0x94 => { self.sub(copy_registers.h); 1 },              // SUB A,H
            0x95 => { self.sub(copy_registers.l); 1 },              // SUB A,L
            0x96 => { let mem_val = self.mmu.fetch(copy_registers.hl()); self.sub(mem_val); 1 },    // SUB A,[HL]
            0x97 => { self.sub(copy_registers.a); 1 },              // SUB A,A
            0xA0 => { self.and(copy_registers.b); 1 },              // AND A,B
            0xA1 => { self.and(copy_registers.c); 1 },              // AND A,C
            0xA2 => { self.and(copy_registers.d); 1 },              // AND A,D
            0xA3 => { self.and(copy_registers.e); 1 },              // AND A,E
            0xA4 => { self.and(copy_registers.h); 1 },              // AND A,H
            0xA5 => { self.and(copy_registers.l); 1 },              // AND A,L
            0xA6 => { let mem_val = self.mmu.fetch(copy_registers.hl()); self.and(mem_val); 1 },    // AND A,[HL]
            0xA7 => { self.and(copy_registers.a); 1 },              // AND A,A
            0xA8 => { self.xor(copy_registers.b); 1 },              // XOR A,B
            0xA9 => { self.xor(copy_registers.c); 1 },              // XOR A,C
            0xAA => { self.xor(copy_registers.d); 1 },              // XOR A,D
            0xAB => { self.xor(copy_registers.e); 1 },              // XOR A,E
            0xAC => { self.xor(copy_registers.h); 1 },              // XOR A,H
            0xAD => { self.xor(copy_registers.l); 1 },              // XOR A,L
            0xAE => { let mem_val = self.mmu.fetch(copy_registers.hl()); self.xor(mem_val); 1 },    // XOR A,[HL]
            0xAF => { self.xor(copy_registers.a); 1 },              // XOR A,A
            0xB0 => { self.or(copy_registers.b); 1  },              // OR A,B
            0xB1 => { self.or(copy_registers.c); 1  },              // OR A,C
            0xB2 => { self.or(copy_registers.d); 1  },              // OR A,D
            0xB3 => { self.or(copy_registers.e); 1  },              // OR A,E
            0xB4 => { self.or(copy_registers.h); 1  },              // OR A,H
            0xB5 => { self.or(copy_registers.l); 1  },              // OR A,L
            0xB6 => { let mem_val = self.mmu.fetch(copy_registers.hl()); self.or(mem_val); 1 },     // OR A,[HL]
            0xB7 => { self.or(copy_registers.a); 1  },              // OR A,A
            0xB8 => { self.cp(copy_registers.b); 1  },              // CP A,B
            0xB9 => { self.cp(copy_registers.c); 1  },              // CP A,C
            0xBA => { self.cp(copy_registers.d); 1  },              // CP A,D
            0xBB => { self.cp(copy_registers.e); 1  },              // CP A,E
            0xBC => { self.cp(copy_registers.h); 1  },              // CP A,H
            0xBD => { self.cp(copy_registers.l); 1  },              // CP A,L
            0xBE => { let mem_val = self.mmu.fetch(copy_registers.hl()); self.cp(mem_val); 1 },     // CP A,[HL]
            0xBF => { self.cp(copy_registers.a); 1  },              // CP A,A
            0xC6 => { let byte = self.fetch_byte(); self.add(byte); 1 },                            // ADD A,#
            0xD6 => { let byte = self.fetch_byte(); self.sub(byte); 1 },                            // SUB A,#
            0xE6 => { let byte = self.fetch_byte(); self.and(byte); 1 },                            // AND A,#
            0xEE => { let byte = self.fetch_byte(); self.xor(byte); 1 },                            // XOR A,#
            0xF6 => { let byte = self.fetch_byte(); self.or(byte);  1  },                           // OR A,#
            0xFE => { let byte = self.fetch_byte(); self.cp(byte);  1  },                           // CP A,#
            other => panic!("Instruction not implemented!"),
        }
    }
}
