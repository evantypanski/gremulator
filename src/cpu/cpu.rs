extern crate log;

use std::io::Error;

use self::log::{info, trace};

pub struct CPU {
    pub registers: ::register::Registers,
    mmu: ::mmu::MMU,
    pub halted: bool,
}

impl CPU {
    pub fn new() -> Result<CPU, Error> {
        info!("Created new CPU");
        let mmu = ::mmu::MMU::new()?;
        Ok(CPU {
            registers: ::register::Registers::new(),
            mmu: mmu,
            halted: false,
        })
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.mmu.fetch(self.registers.pc);
        self.registers.pc += 1;
        byte
    }

    pub fn cycle(&mut self) -> u8 {
        let opcode = self.fetch_byte();
        trace!("Cycle on opcode {}", opcode);
        self.ops(opcode)
    }

    // All operations
    pub fn ops(&mut self, opcode: u8) -> u8 {
        // Copy registers to avoid immutable/mutable reference with ALU ops
        let copy_registers = self.registers;
        match opcode {
            // NOP
            0x00 => 4,
            // LD BC,u16
            0x01 => {
                let immediate = ((self.fetch_byte() as u16) << 8) + (self.fetch_byte() as u16);
                self.registers.set_bc(immediate);
                12
            }
            // LD (BC), A
            0x02 => {
                // TODO
                8
            }
            // INC BC
            0x03 => {
                // Does not update flags, so don't need to go through ALU.
                self.registers.set_bc(self.registers.bc().wrapping_add(1));
                8
            }
            // INC B
            0x04 => {
                ::cpu::alu::inc(&mut self.registers.b, &mut self.registers.f);
                4
            }
            // DEC B
            0x05 => {
                ::cpu::alu::dec(&mut self.registers.b, &mut self.registers.f);
                4
            }
            // LD B,n
            0x06 => {
                self.registers.b = self.fetch_byte();
                8
            }
            // RLCA
            0x07 => {
                ::cpu::alu::rlc(&mut self.registers.a, &mut self.registers.f);
                4
            }
            // LD (u16), SP
            0x08 => {
                // TODO
                20
            }
            // ADD HL, BC
            0x09 => {
                // TODO
                8
            }
            // LD A,(BC)
            0x0A => {
                // TODO
                8
            }
            // DEC BC
            0x0B => {
                // Does not update flags, so don't need to go through ALU.
                self.registers.set_bc(self.registers.bc().wrapping_sub(1));
                8
            }
            // INC C
            0x0C => {
                ::cpu::alu::inc(&mut self.registers.c, &mut self.registers.f);
                4
            }
            // DEC C
            0x0D => {
                ::cpu::alu::dec(&mut self.registers.c, &mut self.registers.f);
                4
            }
            // LD C,n
            0x0E => {
                self.registers.c = self.fetch_byte();
                8
            }
            // RRCA
            0x0F => {
                ::cpu::alu::rrc(&mut self.registers.a, &mut self.registers.f);
                4
            }
            // LD DE,u16
            0x11 => {
                let immediate = ((self.fetch_byte() as u16) << 8) + (self.fetch_byte() as u16);
                self.registers.set_de(immediate);
                12
            }
            // LD (DE),A
            0x12 => {
                // TODO
                8
            }
            // INC DE
            0x13 => {
                // Does not update flags, so don't need to go through ALU.
                self.registers.set_de(self.registers.de().wrapping_add(1));
                8
            }
            // INC D
            0x14 => {
                ::cpu::alu::inc(&mut self.registers.d, &mut self.registers.f);
                4
            }
            // DEC D
            0x15 => {
                ::cpu::alu::dec(&mut self.registers.d, &mut self.registers.f);
                4
            }
            // LD D,n
            0x16 => {
                self.registers.d = self.fetch_byte();
                8
            }
            // RLA
            0x17 => {
                ::cpu::alu::rl(&mut self.registers.a, &mut self.registers.f);
                4
            }
            // JR i8
            0x18 => {
                // TODO
                12
            }
            // ADD HL, DE
            0x19 => {
                // TODO
                8
            }
            // LD A,(DE)
            0x1A => {
                // TODO
                8
            }
            // DEC BC
            0x1B => {
                // Does not update flags, so don't need to go through ALU.
                self.registers.set_de(self.registers.de().wrapping_sub(1));
                8
            }
            // INC E
            0x1C => {
                ::cpu::alu::inc(&mut self.registers.e, &mut self.registers.f);
                4
            }
            // DEC E
            0x1D => {
                ::cpu::alu::dec(&mut self.registers.e, &mut self.registers.f);
                4
            }
            // LD E,n
            0x1E => {
                self.registers.e = self.fetch_byte();
                8
            }
            // RRA
            0x1F => {
                ::cpu::alu::rr(&mut self.registers.a, &mut self.registers.f);
                4
            }
            // LD HL,u16
            0x21 => {
                let immediate = ((self.fetch_byte() as u16) << 8) + (self.fetch_byte() as u16);
                self.registers.set_hl(immediate);
                12
            }
            // LD (HL+),A
            0x22 => {
                // TODO
                8
            }
            // INC HL
            0x23 => {
                // Does not update flags, so don't need to go through ALU.
                self.registers.set_hl(self.registers.hl().wrapping_add(1));
                8
            }
            // INC H
            0x24 => {
                ::cpu::alu::inc(&mut self.registers.h, &mut self.registers.f);
                4
            }
            // DEC H
            0x25 => {
                ::cpu::alu::dec(&mut self.registers.h, &mut self.registers.f);
                4
            }
            // LD H,n
            0x26 => {
                self.registers.h = self.fetch_byte();
                8
            }
            // DAA
            0x27 => {
                ::cpu::alu::daa(&mut self.registers.a, &mut self.registers.f);
                4
            }
            // JR Z,i8
            0x28 => {
                // TODO
                // Note: time is 8-12 - determine which
                8
            }
            // ADD HL, HL
            0x29 => {
                // TODO
                8
            }
            // LD A,(HL+)
            0x2A => {
                // TODO
                8
            }
            // DEC HL
            0x2B => {
                // Does not update flags, so don't need to go through ALU.
                self.registers.set_hl(self.registers.hl().wrapping_sub(1));
                8
            }
            // INC L
            0x2C => {
                ::cpu::alu::inc(&mut self.registers.l, &mut self.registers.f);
                4
            }
            // DEC L
            0x2D => {
                ::cpu::alu::dec(&mut self.registers.e, &mut self.registers.f);
                4
            }
            // LD L,n
            0x2E => {
                self.registers.l = self.fetch_byte();
                8
            }
            // CPL
            0x2F => {
                ::cpu::alu::cpl(&mut self.registers.a, &mut self.registers.f);
                4
            }
            // LD SP,u16
            0x31 => {
                let immediate = ((self.fetch_byte() as u16) << 8) + (self.fetch_byte() as u16);
                self.registers.sp = immediate;
                12
            }
            // LD (HL-),A
            0x32 => {
                // TODO
                8
            }
            // INC SP
            0x33 => {
                // Does not update flags, so don't need to go through ALU.
                self.registers.sp = self.registers.sp.wrapping_add(1);
                8
            }
            // INC (HL)
            0x34 => {
                // TODO: Is there a better way?
                let mem_addr = self.registers.hl();
                let mut res = self.mmu.fetch(mem_addr);
                ::cpu::alu::inc(&mut res, &mut self.registers.f);
                self.mmu.set_mem_addr(mem_addr, res);
                12
            }
            // DEC (HL)
            0x35 => {
                // TODO: Is there a better way?
                let mem_addr = self.registers.hl();
                let mut res = self.mmu.fetch(mem_addr);
                ::cpu::alu::dec(&mut res, &mut self.registers.f);
                self.mmu.set_mem_addr(mem_addr, res);
                12
            }
            // LD (HL),n
            0x36 => {
                let val = self.fetch_byte();
                self.mmu.set_mem_addr(self.registers.hl(), val);
                12
            }
            // SCF
            0x37 => {
                ::cpu::alu::scf(&mut self.registers.f);
                4
            }
            // JR C,i8
            0x38 => {
                // TODO
                // Note: time is 8-12 - determine which
                8
            }
            // ADD HL, SP
            0x39 => {
                // TODO
                8
            }
            // LD A,(HL-)
            0x3A => {
                // TODO
                8
            }
            // DEC SP
            0x3B => {
                // Does not update flags, so don't need to go through ALU.
                self.registers.sp = self.registers.sp.wrapping_sub(1);
                8
            }
            // INC A
            0x3C => {
                ::cpu::alu::inc(&mut self.registers.a, &mut self.registers.f);
                4
            }
            // DEC A
            0x3D => {
                ::cpu::alu::dec(&mut self.registers.a, &mut self.registers.f);
                4
            }
            // LD A,n
            0x3E => {
                self.registers.a = self.fetch_byte();
                8
            }
            // CCF
            0x3F => {
                ::cpu::alu::ccf(&mut self.registers.f);
                4
            }
            // LD B,B
            0x40 => {
                self.registers.b = self.registers.b;
                4
            }
            // LD B,C
            0x41 => {
                self.registers.b = self.registers.c;
                4
            }
            // LD B,D
            0x42 => {
                self.registers.b = self.registers.d;
                4
            }
            // LD B,E
            0x43 => {
                self.registers.b = self.registers.e;
                4
            }
            // LD B,H
            0x44 => {
                self.registers.b = self.registers.h;
                4
            }
            // LD B,L
            0x45 => {
                self.registers.b = self.registers.l;
                4
            }
            // LD B,(HL)
            0x46 => {
                self.registers.b = self.mmu.fetch(self.registers.hl());
                8
            }
            // LD C,B
            0x48 => {
                self.registers.c = self.registers.b;
                4
            }
            // LD C,C
            0x49 => {
                self.registers.c = self.registers.c;
                4
            }
            // LD C,D
            0x4A => {
                self.registers.c = self.registers.d;
                4
            }
            // LD C,E
            0x4B => {
                self.registers.c = self.registers.e;
                4
            }
            // LD C,H
            0x4C => {
                self.registers.c = self.registers.h;
                4
            }
            // LD C,L
            0x4D => {
                self.registers.c = self.registers.l;
                4
            }
            // LD C,(HL)
            0x4E => {
                self.registers.c = self.mmu.fetch(self.registers.hl());
                8
            }
            // LD D,B
            0x50 => {
                self.registers.d = self.registers.b;
                4
            }
            // LD D,C
            0x51 => {
                self.registers.d = self.registers.c;
                4
            }
            // LD D,D
            0x52 => {
                self.registers.d = self.registers.d;
                4
            }
            // LD D,E
            0x53 => {
                self.registers.d = self.registers.e;
                4
            }
            // LD D,H
            0x54 => {
                self.registers.d = self.registers.h;
                4
            }
            // LD D,L
            0x55 => {
                self.registers.d = self.registers.l;
                4
            }
            // LD D,(HL)
            0x56 => {
                self.registers.d = self.mmu.fetch(self.registers.hl());
                8
            }
            // LD E,B
            0x58 => {
                self.registers.e = self.registers.b;
                4
            }
            // LD E,C
            0x59 => {
                self.registers.e = self.registers.c;
                4
            }
            // LD E,D
            0x5A => {
                self.registers.e = self.registers.d;
                4
            }
            // LD E,E
            0x5B => {
                self.registers.e = self.registers.e;
                4
            }
            // LD E,H
            0x5C => {
                self.registers.e = self.registers.h;
                4
            }
            // LD E,L
            0x5D => {
                self.registers.e = self.registers.l;
                4
            }
            // LD E,(HL)
            0x5E => {
                self.registers.e = self.mmu.fetch(self.registers.hl());
                8
            }
            // LD H,B
            0x60 => {
                self.registers.h = self.registers.b;
                4
            }
            // LD H,C
            0x61 => {
                self.registers.h = self.registers.c;
                4
            }
            // LD H,D
            0x62 => {
                self.registers.h = self.registers.d;
                4
            }
            // LD H,E
            0x63 => {
                self.registers.h = self.registers.e;
                4
            }
            // LD H,H
            0x64 => {
                self.registers.h = self.registers.h;
                4
            }
            // LD H,L
            0x65 => {
                self.registers.h = self.registers.l;
                4
            }
            // LD H,(HL)
            0x66 => {
                self.registers.h = self.mmu.fetch(self.registers.hl());
                8
            }
            // LD L,B
            0x68 => {
                self.registers.l = self.registers.b;
                4
            }
            // LD L,C
            0x69 => {
                self.registers.l = self.registers.c;
                4
            }
            // LD L,D
            0x6A => {
                self.registers.l = self.registers.d;
                4
            }
            // LD L,E
            0x6B => {
                self.registers.l = self.registers.e;
                4
            }
            // LD L,H
            0x6C => {
                self.registers.l = self.registers.h;
                4
            }
            // LD L,L
            0x6D => {
                self.registers.l = self.registers.l;
                4
            }
            // LD L,(HL)
            0x6E => {
                self.registers.l = self.mmu.fetch(self.registers.hl());
                8
            }
            // LD (HL),B
            0x70 => {
                self.mmu.set_mem_addr(self.registers.hl(), self.registers.b);
                8
            }
            // LD (HL),C
            0x71 => {
                self.mmu.set_mem_addr(self.registers.hl(), self.registers.c);
                8
            }
            // LD (HL),D
            0x72 => {
                self.mmu.set_mem_addr(self.registers.hl(), self.registers.d);
                self.registers.h = self.registers.d;
                8
            }
            // LD (HL),E
            0x73 => {
                self.mmu.set_mem_addr(self.registers.hl(), self.registers.e);
                8
            }
            // LD (HL),H
            0x74 => {
                self.mmu.set_mem_addr(self.registers.hl(), self.registers.h);
                8
            }
            // LD (HL),L
            0x75 => {
                self.mmu.set_mem_addr(self.registers.hl(), self.registers.l);
                8
            }
            // HALT
            0x76 => {
                info!("CPU halting");
                self.halted = true;
                4
            }
            // LD A,B
            0x78 => {
                self.registers.a = self.registers.b;
                4
            }
            // LD A,C
            0x79 => {
                self.registers.a = self.registers.c;
                4
            }
            // LD A,D
            0x7A => {
                self.registers.a = self.registers.d;
                4
            }
            // LD A,E
            0x7B => {
                self.registers.a = self.registers.e;
                4
            }
            // LD A,H
            0x7C => {
                self.registers.a = self.registers.h;
                4
            }
            // LD A,L
            0x7D => {
                self.registers.a = self.registers.l;
                4
            }
            // LD A,(HL)
            0x7E => {
                self.registers.a = self.mmu.fetch(self.registers.hl());
                8
            }
            // LD A,A
            0x7F => {
                self.registers.a = self.registers.a;
                4
            }
            // ADD A,B
            0x80 => {
                ::cpu::alu::add(
                    &mut self.registers.a,
                    Some(self.registers.b),
                    &mut self.registers.f,
                );
                4
            }
            // ADD A,C
            0x81 => {
                ::cpu::alu::add(
                    &mut self.registers.a,
                    Some(self.registers.c),
                    &mut self.registers.f,
                );
                4
            }
            // ADD A,D
            0x82 => {
                ::cpu::alu::add(
                    &mut self.registers.a,
                    Some(self.registers.d),
                    &mut self.registers.f,
                );
                4
            }
            // ADD A,E
            0x83 => {
                ::cpu::alu::add(
                    &mut self.registers.a,
                    Some(self.registers.e),
                    &mut self.registers.f,
                );
                4
            }
            // ADD A,H
            0x84 => {
                ::cpu::alu::add(
                    &mut self.registers.a,
                    Some(self.registers.h),
                    &mut self.registers.f,
                );
                4
            }
            // ADD A,L
            0x85 => {
                ::cpu::alu::add(
                    &mut self.registers.a,
                    Some(self.registers.l),
                    &mut self.registers.f,
                );
                4
            }
            // ADD A,[HL]
            0x86 => {
                let mem_val = self.mmu.fetch(copy_registers.hl());
                ::cpu::alu::add(&mut self.registers.a, Some(mem_val), &mut self.registers.f);
                8
            }
            // ADD A,A
            0x87 => {
                ::cpu::alu::add(&mut self.registers.a, None, &mut self.registers.f);
                4
            }
            // ADC A,B
            0x88 => {
                ::cpu::alu::add_carry(
                    &mut self.registers.a,
                    Some(self.registers.b),
                    &mut self.registers.f,
                );
                4
            }
            // ADC A,C
            0x89 => {
                ::cpu::alu::add_carry(
                    &mut self.registers.a,
                    Some(self.registers.c),
                    &mut self.registers.f,
                );
                4
            }
            // ADC A,D
            0x8A => {
                ::cpu::alu::add_carry(
                    &mut self.registers.a,
                    Some(self.registers.d),
                    &mut self.registers.f,
                );
                4
            }
            // ADC A,E
            0x8B => {
                ::cpu::alu::add_carry(
                    &mut self.registers.a,
                    Some(self.registers.e),
                    &mut self.registers.f,
                );
                4
            }
            // ADC A,H
            0x8C => {
                ::cpu::alu::add_carry(
                    &mut self.registers.a,
                    Some(self.registers.h),
                    &mut self.registers.f,
                );
                4
            }
            // ADC A,L
            0x8D => {
                ::cpu::alu::add_carry(
                    &mut self.registers.a,
                    Some(self.registers.l),
                    &mut self.registers.f,
                );
                4
            }
            // ADC A,[HL]
            0x8E => {
                let mem_val = self.mmu.fetch(copy_registers.hl());
                ::cpu::alu::add_carry(&mut self.registers.a, Some(mem_val), &mut self.registers.f);
                8
            }
            // ADC A,A
            0x8F => {
                ::cpu::alu::add_carry(&mut self.registers.a, None, &mut self.registers.f);
                4
            }
            // SUB A,B
            0x90 => {
                ::cpu::alu::sub(
                    &mut self.registers.a,
                    Some(self.registers.b),
                    &mut self.registers.f,
                );
                4
            }
            // SUB A,C
            0x91 => {
                ::cpu::alu::sub(
                    &mut self.registers.a,
                    Some(self.registers.c),
                    &mut self.registers.f,
                );
                4
            }
            // SUB A,D
            0x92 => {
                ::cpu::alu::sub(
                    &mut self.registers.a,
                    Some(self.registers.d),
                    &mut self.registers.f,
                );
                4
            }
            // SUB A,E
            0x93 => {
                ::cpu::alu::sub(
                    &mut self.registers.a,
                    Some(self.registers.e),
                    &mut self.registers.f,
                );
                4
            }
            // SUB A,H
            0x94 => {
                ::cpu::alu::sub(
                    &mut self.registers.a,
                    Some(self.registers.h),
                    &mut self.registers.f,
                );
                4
            }
            // SUB A,L
            0x95 => {
                ::cpu::alu::sub(
                    &mut self.registers.a,
                    Some(self.registers.l),
                    &mut self.registers.f,
                );
                4
            }
            // SUB A,[HL]
            0x96 => {
                let mem_val = self.mmu.fetch(copy_registers.hl());
                ::cpu::alu::sub(&mut self.registers.a, Some(mem_val), &mut self.registers.f);
                8
            }
            // SUB A,A
            0x97 => {
                ::cpu::alu::sub(&mut self.registers.a, None, &mut self.registers.f);
                4
            }
            // TODO: SBC
            // AND A,B
            0xA0 => {
                ::cpu::alu::and(
                    &mut self.registers.a,
                    Some(self.registers.b),
                    &mut self.registers.f,
                );
                4
            }
            // AND A,C
            0xA1 => {
                ::cpu::alu::and(
                    &mut self.registers.a,
                    Some(self.registers.c),
                    &mut self.registers.f,
                );
                4
            }
            // AND A,D
            0xA2 => {
                ::cpu::alu::and(
                    &mut self.registers.a,
                    Some(self.registers.d),
                    &mut self.registers.f,
                );
                4
            }
            // AND A,E
            0xA3 => {
                ::cpu::alu::and(
                    &mut self.registers.a,
                    Some(self.registers.e),
                    &mut self.registers.f,
                );
                4
            }
            // AND A,H
            0xA4 => {
                ::cpu::alu::and(
                    &mut self.registers.a,
                    Some(self.registers.h),
                    &mut self.registers.f,
                );
                4
            }
            // AND A,L
            0xA5 => {
                ::cpu::alu::and(
                    &mut self.registers.a,
                    Some(self.registers.l),
                    &mut self.registers.f,
                );
                4
            }
            // AND A,[HL]
            0xA6 => {
                let mem_val = self.mmu.fetch(copy_registers.hl());
                ::cpu::alu::and(&mut self.registers.a, Some(mem_val), &mut self.registers.f);
                8
            }
            // AND A,A
            0xA7 => {
                ::cpu::alu::and(&mut self.registers.a, None, &mut self.registers.f);
                4
            }
            // XOR A,B
            0xA8 => {
                ::cpu::alu::xor(
                    &mut self.registers.a,
                    Some(self.registers.b),
                    &mut self.registers.f,
                );
                4
            }
            // XOR A,C
            0xA9 => {
                ::cpu::alu::xor(
                    &mut self.registers.a,
                    Some(self.registers.c),
                    &mut self.registers.f,
                );
                4
            }
            // XOR A,D
            0xAA => {
                ::cpu::alu::xor(
                    &mut self.registers.a,
                    Some(self.registers.d),
                    &mut self.registers.f,
                );
                4
            }
            // XOR A,E
            0xAB => {
                ::cpu::alu::xor(
                    &mut self.registers.a,
                    Some(self.registers.e),
                    &mut self.registers.f,
                );
                4
            }
            // XOR A,H
            0xAC => {
                ::cpu::alu::xor(
                    &mut self.registers.a,
                    Some(self.registers.h),
                    &mut self.registers.f,
                );
                4
            }
            // XOR A,L
            0xAD => {
                ::cpu::alu::xor(
                    &mut self.registers.a,
                    Some(self.registers.l),
                    &mut self.registers.f,
                );
                4
            }
            // XOR A,[HL]
            0xAE => {
                let mem_val = self.mmu.fetch(copy_registers.hl());
                ::cpu::alu::xor(&mut self.registers.a, Some(mem_val), &mut self.registers.f);
                8
            }
            // XOR A,A
            0xAF => {
                ::cpu::alu::xor(&mut self.registers.a, None, &mut self.registers.f);
                4
            }
            // OR A,B
            0xB0 => {
                ::cpu::alu::or(
                    &mut self.registers.a,
                    Some(self.registers.b),
                    &mut self.registers.f,
                );
                4
            }
            // OR A,C
            0xB1 => {
                ::cpu::alu::or(
                    &mut self.registers.a,
                    Some(self.registers.c),
                    &mut self.registers.f,
                );
                4
            }
            // OR A,D
            0xB2 => {
                ::cpu::alu::or(
                    &mut self.registers.a,
                    Some(self.registers.d),
                    &mut self.registers.f,
                );
                4
            }
            // OR A,E
            0xB3 => {
                ::cpu::alu::or(
                    &mut self.registers.a,
                    Some(self.registers.e),
                    &mut self.registers.f,
                );
                4
            }
            // OR A,H
            0xB4 => {
                ::cpu::alu::or(
                    &mut self.registers.a,
                    Some(self.registers.h),
                    &mut self.registers.f,
                );
                4
            }
            // OR A,L
            0xB5 => {
                ::cpu::alu::or(
                    &mut self.registers.a,
                    Some(self.registers.l),
                    &mut self.registers.f,
                );
                4
            }
            // OR A,[HL]
            0xB6 => {
                let mem_val = self.mmu.fetch(copy_registers.hl());
                ::cpu::alu::or(&mut self.registers.a, Some(mem_val), &mut self.registers.f);
                8
            }
            // OR A,A
            0xB7 => {
                ::cpu::alu::or(&mut self.registers.a, None, &mut self.registers.f);
                4
            }
            // CP A,B
            0xB8 => {
                ::cpu::alu::cp(
                    self.registers.a,
                    Some(self.registers.b),
                    &mut self.registers.f,
                );
                4
            }
            // CP A,C
            0xB9 => {
                ::cpu::alu::cp(
                    self.registers.a,
                    Some(self.registers.c),
                    &mut self.registers.f,
                );
                4
            }
            // CP A,D
            0xBA => {
                ::cpu::alu::cp(
                    self.registers.a,
                    Some(self.registers.d),
                    &mut self.registers.f,
                );
                4
            }
            // CP A,E
            0xBB => {
                ::cpu::alu::cp(
                    self.registers.a,
                    Some(self.registers.e),
                    &mut self.registers.f,
                );
                4
            }
            // CP A,H
            0xBC => {
                ::cpu::alu::cp(
                    self.registers.a,
                    Some(self.registers.h),
                    &mut self.registers.f,
                );
                4
            }
            // CP A,L
            0xBD => {
                ::cpu::alu::cp(
                    self.registers.a,
                    Some(self.registers.l),
                    &mut self.registers.f,
                );
                4
            }
            // CP A,[HL]
            0xBE => {
                let mem_val = self.mmu.fetch(copy_registers.hl());
                ::cpu::alu::cp(self.registers.a, Some(mem_val), &mut self.registers.f);
                8
            }
            // CP A,A
            0xBF => {
                ::cpu::alu::cp(self.registers.a, None, &mut self.registers.f);
                4
            }
            // ADD A,#
            0xC6 => {
                let byte = self.fetch_byte();
                ::cpu::alu::add(&mut self.registers.a, Some(byte), &mut self.registers.f);
                8
            }
            // TODO: ADC
            // SUB A,#
            0xD6 => {
                let byte = self.fetch_byte();
                ::cpu::alu::sub(&mut self.registers.a, Some(byte), &mut self.registers.f);
                8
            }
            // TODO: SBC
            // AND A,#
            0xE6 => {
                let byte = self.fetch_byte();
                ::cpu::alu::and(&mut self.registers.a, Some(byte), &mut self.registers.f);
                8
            }
            // XOR A,#
            0xEE => {
                let byte = self.fetch_byte();
                ::cpu::alu::xor(&mut self.registers.a, Some(byte), &mut self.registers.f);
                8
            }
            // OR A,#
            0xF6 => {
                let byte = self.fetch_byte();
                ::cpu::alu::or(&mut self.registers.a, Some(byte), &mut self.registers.f);
                8
            }
            // CP A,#
            0xFE => {
                let byte = self.fetch_byte();
                ::cpu::alu::cp(self.registers.a, Some(byte), &mut self.registers.f);
                8
            }
            other => panic!("Instruction {} not implemented!", other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_creates_properly() {
        let cpu = CPU::new().unwrap();
        assert!(!cpu.halted);
    }
}
