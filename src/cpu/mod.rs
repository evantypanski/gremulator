mod alu;

pub struct CPU {
    registers:  ::register::Registers,
    mmu:        ::mmu::MMU,
    halted:     bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers:  ::register::Registers::new(),
            mmu:        ::mmu::MMU::new(),
            halted:     false,
        }
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.mmu.fetch(self.registers.pc);
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

    // All operations
    pub fn ops(&mut self, opcode: u8) -> u8 {
        // Copy registers to avoid immutable/mutable reference with ALU ops
        let copy_registers = self.registers;
        match opcode {
            // NOP
            0x00 => {
                1
            },
            // LD B,n
            0x06 => {
                self.registers.b = self.fetch_byte();
                1
            },
            // LD C,n
            0x0E => {
                self.registers.c = self.fetch_byte();
                1
            },
            // LD D,n
            0x16 => {
                self.registers.d = self.fetch_byte();
                1
            },
            // LD E,n
            0x1E => {
                self.registers.e = self.fetch_byte();
                1
            },
            // LD H,n
            0x26 => {
                self.registers.h = self.fetch_byte();
                1
            },
            // LD L,n
            0x2E => {
                self.registers.l = self.fetch_byte();
                1
            },
            // IMPLEMENT LD (HL),n
            0x36 => {
                self.registers.h = self.registers.l;
                1
            },
            // LD B,B
            0x40 => {
                self.registers.b = self.registers.b;
                1
            },
            // LD B,C
            0x41 => {
                self.registers.b = self.registers.c;
                1
            },
            // LD B,D
            0x42 => {
                self.registers.b = self.registers.d;
                1
            },
            // LD B,E
            0x43 => {
                self.registers.b = self.registers.e;
                1
            },
            // LD B,H
            0x44 => {
                self.registers.b = self.registers.h;
                1
            },
            // LD B,L
            0x45 => {
                self.registers.b = self.registers.l;
                1
            },
            // IMPLEMENT LD B,(HL)
            0x46 => {
                self.registers.b = self.registers.l;
                1
            },
            // LD C,B
            0x48 => {
                self.registers.c = self.registers.b;
                1
            },
            // LD C,C
            0x49 => {
                self.registers.c = self.registers.c;
                1
            },
            // LD C,D
            0x4A => {
                self.registers.c = self.registers.d;
                1
            },
            // LD C,E
            0x4B => {
                self.registers.c = self.registers.e;
                1
            },
            // LD C,H
            0x4C => {
                self.registers.c = self.registers.h;
                1
            },
            // LD C,L
            0x4D => {
                self.registers.c = self.registers.l;
                1
            },
            // IMPLEMENT LD C,(HL)
            0x4E => {
                self.registers.c = self.registers.l;
                1
            },
            // LD D,B
            0x50 => {
                self.registers.d = self.registers.b;
                1
            },
            // LD D,C
            0x51 => {
                self.registers.d = self.registers.c;
                1
            },
            // LD D,D
            0x52 => {
                self.registers.d = self.registers.d;
                1
            },
            // LD D,E
            0x53 => {
                self.registers.d = self.registers.e;
                1
            },
            // LD D,H
            0x54 => {
                self.registers.d = self.registers.h;
                1
            },
            // LD D,L
            0x55 => {
                self.registers.d = self.registers.l;
                1
            },
            // IMPLEMENT LD D,(HL)
            0x56 => {
                self.registers.d = self.registers.l;
                1
            },
            // LD E,B
            0x58 => {
                self.registers.e = self.registers.b;
                1
            },
            // LD E,C
            0x59 => {
                self.registers.e = self.registers.c;
                1
            },
            // LD E,D
            0x5A => {
                self.registers.e = self.registers.d;
                1
            },
            // LD E,E
            0x5B => {
                self.registers.e = self.registers.e;
                1
            },
            // LD E,H
            0x5C => {
                self.registers.e = self.registers.h;
                1
            },
            // LD E,L
            0x5D => {
                self.registers.e = self.registers.l;
                1
            },
            // IMPLEMENT LD E,(HL)
            0x5E => {
                self.registers.e = self.registers.l;
                1
            },
            // LD H,B
            0x60 => {
                self.registers.h = self.registers.b;
                1
            },
            // LD H,C
            0x61 => {
                self.registers.h = self.registers.c;
                1
            },
            // LD H,D
            0x62 => {
                self.registers.h = self.registers.d;
                1
            },
            // LD H,E
            0x63 => {
                self.registers.h = self.registers.e;
                1
            },
            // LD H,H
            0x64 => {
                self.registers.h = self.registers.h;
                1
            },
            // LD H,L
            0x65 => {
                self.registers.h = self.registers.l;
                1
            },
            // IMPLEMENT LD H,(HL)
            0x66 => {
                self.registers.h = self.registers.l;
                1
            },
            // LD L,B
            0x68 => {
                self.registers.l = self.registers.b;
                1
            },
            // LD L,C
            0x69 => {
                self.registers.l = self.registers.c;
                1
            },
            // LD L,D
            0x6A => {
                self.registers.l = self.registers.d;
                1
            },
            // LD L,E
            0x6B => {
                self.registers.l = self.registers.e;
                1
            },
            // LD L,H
            0x6C => {
                self.registers.l = self.registers.h;
                1
            },
            // LD L,L
            0x6D => {
                self.registers.l = self.registers.l;
                1
            },
            // IMPLEMENT LD L,(HL)
            0x6E => {
                self.registers.l = self.registers.l;
                1
            },
            // IMPLEMENT LD (HL),B
            0x70 => {
                self.registers.h = self.registers.b;
                1
            },
            // IMPLEMENT LD (HL),C
            0x71 => {
                self.registers.h = self.registers.c;
                1
            },
            // IMPLEMENT LD (HL),D
            0x72 => {
                self.registers.h = self.registers.d;
                1
            },
            // IMPLEMENT LD (HL),E
            0x73 => {
                self.registers.h = self.registers.e;
                1
            },
            // IMPLEMENT LD (HL),H
            0x74 => {
                self.registers.h = self.registers.h;
                1
            },
            // IMPLEMENT LD (HL),L
            0x75 => {
                self.registers.h = self.registers.l;
                1
            },
            // IMPLEMENT HALT
            0x76 => {
                66
            },
            // LD A,B
            0x78 => {
                self.registers.a = self.registers.b;
                1
            },
            // LD A,C
            0x79 => {
                self.registers.a = self.registers.c;
                1
            },
            // LD A,D
            0x7A => {
                self.registers.a = self.registers.d;
                1
            },
            // LD A,E
            0x7B => {
                self.registers.a = self.registers.e;
                1
            },
            // LD A,H
            0x7C => {
                self.registers.a = self.registers.h;
                1
            },
            // LD A,L
            0x7D => {
                self.registers.a = self.registers.l;
                1
            },
            // IMPLEMENT LD A,(HL)
            0x7E => {
                self.registers.a = self.registers.l;
                1
            },
            // LD A,A
            0x7F => {
                self.registers.a = self.registers.a;
                1
            },
            // ADD A,B
            0x80 => {
                ::cpu::alu::add(&mut self.registers.a, Some(self.registers.b), &mut self.registers.f);
                1
            },
            // ADD A,C
            0x81 => {
                ::cpu::alu::add(&mut self.registers.a, Some(self.registers.c), &mut self.registers.f);
                1
            },
            // ADD A,D
            0x82 => {
                ::cpu::alu::add(&mut self.registers.a, Some(self.registers.d), &mut self.registers.f);
                1
            },
            // ADD A,E
            0x83 => {
                ::cpu::alu::add(&mut self.registers.a, Some(self.registers.e), &mut self.registers.f);
                1
            },
            // ADD A,H
            0x84 => {
                ::cpu::alu::add(&mut self.registers.a, Some(self.registers.h), &mut self.registers.f);
                1
            },
            // ADD A,L
            0x85 => {
                ::cpu::alu::add(&mut self.registers.a, Some(self.registers.l), &mut self.registers.f);
                1
            },
            // ADD A,[HL]
            0x86 => {
                let mem_val = self.mmu.fetch(copy_registers.hl());
                ::cpu::alu::add(&mut self.registers.a, Some(mem_val), &mut self.registers.f);
                1
            },
            // ADD A,A
            0x87 => {
                ::cpu::alu::add(&mut self.registers.a, None, &mut self.registers.f);
                1
            },
            // SUB A,B
            0x90 => {
                ::cpu::alu::sub(&mut self.registers.a, Some(self.registers.b), &mut self.registers.f);
                1
            },
            // SUB A,C
            0x91 => {
                ::cpu::alu::sub(&mut self.registers.a, Some(self.registers.c), &mut self.registers.f);
                1
            },
            // SUB A,D
            0x92 => {
                ::cpu::alu::sub(&mut self.registers.a, Some(self.registers.d), &mut self.registers.f);
                1
            },
            // SUB A,E
            0x93 => {
                ::cpu::alu::sub(&mut self.registers.a, Some(self.registers.e), &mut self.registers.f);
                1
            },
            // SUB A,H
            0x94 => {
                ::cpu::alu::sub(&mut self.registers.a, Some(self.registers.h), &mut self.registers.f);
                1
            },
            // SUB A,L
            0x95 => {
                ::cpu::alu::sub(&mut self.registers.a, Some(self.registers.l), &mut self.registers.f);
                1
            },
            // SUB A,[HL]
            0x96 => {
                let mem_val = self.mmu.fetch(copy_registers.hl());
                ::cpu::alu::sub(&mut self.registers.a, Some(mem_val), &mut self.registers.f);
                1
            },
            // SUB A,A
            0x97 => {
                ::cpu::alu::sub(&mut self.registers.a, None, &mut self.registers.f);
                1
            },
            // AND A,B
            0xA0 => {
                ::cpu::alu::and(&mut self.registers.a, Some(self.registers.b), &mut self.registers.f);
                1
            },
            // AND A,C
            0xA1 => {
                ::cpu::alu::and(&mut self.registers.a, Some(self.registers.c), &mut self.registers.f);
                1
            },
            // AND A,D
            0xA2 => {
                ::cpu::alu::and(&mut self.registers.a, Some(self.registers.d), &mut self.registers.f);
                1
            },
            // AND A,E
            0xA3 => {
                ::cpu::alu::and(&mut self.registers.a, Some(self.registers.e), &mut self.registers.f);
                1
            },
            // AND A,H
            0xA4 => {
                ::cpu::alu::and(&mut self.registers.a, Some(self.registers.h), &mut self.registers.f);
                1
            },
            // AND A,L
            0xA5 => {
                ::cpu::alu::and(&mut self.registers.a, Some(self.registers.l), &mut self.registers.f);
                1
            },
            // AND A,[HL]
            0xA6 => {
                let mem_val = self.mmu.fetch(copy_registers.hl());
                ::cpu::alu::and(&mut self.registers.a, Some(mem_val), &mut self.registers.f);
                1
            },
            // AND A,A
            0xA7 => {
                ::cpu::alu::and(&mut self.registers.a, None, &mut self.registers.f);
                1
            },
            // XOR A,B
            0xA8 => {
                ::cpu::alu::xor(&mut self.registers.a, Some(self.registers.b), &mut self.registers.f);
                1
            },
            // XOR A,C
            0xA9 => {
                ::cpu::alu::xor(&mut self.registers.a, Some(self.registers.c), &mut self.registers.f);
                1
            },
            // XOR A,D
            0xAA => {
                ::cpu::alu::xor(&mut self.registers.a, Some(self.registers.d), &mut self.registers.f);
                1
            },
            // XOR A,E
            0xAB => {
                ::cpu::alu::xor(&mut self.registers.a, Some(self.registers.e), &mut self.registers.f);
                1
            },
            // XOR A,H
            0xAC => {
                ::cpu::alu::xor(&mut self.registers.a, Some(self.registers.h), &mut self.registers.f);
                1
            },
            // XOR A,L
            0xAD => {
                ::cpu::alu::xor(&mut self.registers.a, Some(self.registers.l), &mut self.registers.f);
                1
            },
            // XOR A,[HL]
            0xAE => {
                let mem_val = self.mmu.fetch(copy_registers.hl());
                ::cpu::alu::xor(&mut self.registers.a, Some(mem_val), &mut self.registers.f);
                1
            },
            // XOR A,A
            0xAF => {
                ::cpu::alu::xor(&mut self.registers.a, None, &mut self.registers.f);
                1
            },
            // OR A,B
            0xB0 => {
                ::cpu::alu::or(&mut self.registers.a, Some(self.registers.b), &mut self.registers.f);
                1
            },
            // OR A,C
            0xB1 => {
                ::cpu::alu::or(&mut self.registers.a, Some(self.registers.c), &mut self.registers.f);
                1
            },
            // OR A,D
            0xB2 => {
                ::cpu::alu::or(&mut self.registers.a, Some(self.registers.d), &mut self.registers.f);
                1
            },
            // OR A,E
            0xB3 => {
                ::cpu::alu::or(&mut self.registers.a, Some(self.registers.e), &mut self.registers.f);
                1
            },
            // OR A,H
            0xB4 => {
                ::cpu::alu::or(&mut self.registers.a, Some(self.registers.h), &mut self.registers.f);
                1
            },
            // OR A,L
            0xB5 => {
                ::cpu::alu::or(&mut self.registers.a, Some(self.registers.l), &mut self.registers.f);
                1
            },
            // OR A,[HL]
            0xB6 => {
                let mem_val = self.mmu.fetch(copy_registers.hl());
                ::cpu::alu::or(&mut self.registers.a, Some(mem_val), &mut self.registers.f);
                1
            },
            // OR A,A
            0xB7 => {
                ::cpu::alu::or(&mut self.registers.a, None, &mut self.registers.f);
                1
            },
            // CP A,B
            0xB8 => {
                ::cpu::alu::cp(self.registers.a, Some(self.registers.b), &mut self.registers.f);
                1
            },
            // CP A,C
            0xB9 => {
                ::cpu::alu::cp(self.registers.a, Some(self.registers.c), &mut self.registers.f);
                1
            },
            // CP A,D
            0xBA => {
                ::cpu::alu::cp(self.registers.a, Some(self.registers.d), &mut self.registers.f);
                1
            },
            // CP A,E
            0xBB => {
                ::cpu::alu::cp(self.registers.a, Some(self.registers.e), &mut self.registers.f);
                1
            },
            // CP A,H
            0xBC => {
                ::cpu::alu::cp(self.registers.a, Some(self.registers.h), &mut self.registers.f);
                1
            },
            // CP A,L
            0xBD => {
                ::cpu::alu::cp(self.registers.a, Some(self.registers.l), &mut self.registers.f);
                1
            },
            // CP A,[HL]
            0xBE => {
                let mem_val = self.mmu.fetch(copy_registers.hl());
                ::cpu::alu::cp(self.registers.a, Some(mem_val), &mut self.registers.f);
                1
            },
            // CP A,A
            0xBF => {
                ::cpu::alu::cp(self.registers.a, None, &mut self.registers.f);
                1
            },
            // ADD A,#
            0xC6 => {
                let byte = self.fetch_byte();
                ::cpu::alu::add(&mut self.registers.a, Some(byte), &mut self.registers.f);
                1
            },
            // SUB A,#
            0xD6 => {
                let byte = self.fetch_byte();
                ::cpu::alu::sub(&mut self.registers.a, Some(byte), &mut self.registers.f);
                1
            },
            // AND A,#
            0xE6 => {
                let byte = self.fetch_byte();
                ::cpu::alu::and(&mut self.registers.a, Some(byte), &mut self.registers.f);
                1
            },
            // XOR A,#
            0xEE => {
                let byte = self.fetch_byte();
                ::cpu::alu::xor(&mut self.registers.a, Some(byte), &mut self.registers.f);
                1
            },
            // OR A,#
            0xF6 => {
                let byte = self.fetch_byte();
                ::cpu::alu::or(&mut self.registers.a, Some(byte), &mut self.registers.f);
                1
            },
            // CP A,#
            0xFE => {
                let byte = self.fetch_byte();
                ::cpu::alu::cp(self.registers.a, Some(byte), &mut self.registers.f);
                1
            },
            other => panic!("Instruction {} not implemented!", other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_creates_properly() {
        let cpu = CPU::new();
        assert!(!cpu.halted);
    }
}
