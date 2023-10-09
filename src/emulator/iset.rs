use std::process;

use crate::IO;
use super::arch::{flag::FlagType, opcodes::Opcode, state::State};
use super::utils::*;


impl State {
    pub fn generate_interrupt(&mut self, n: u16) {
        let (pchi, pclo) = split_bytes(self.pc );
        self.mem[self.sp as usize - 1] = pchi;
        self.mem[self.sp as usize - 2] = pclo;
        self.sp -= 2;
        self.enable = 0;

        // RST n
        self.pc = 0x08*n;
    }

    pub fn run_op(&mut self, io: &mut IO) {
        let _pc = self.pc as usize;
        let opcode = self.mem[_pc];
    
        match Opcode::convert(opcode) {
            Opcode::NOP => (),
            Opcode::LXIB => {
                self.b = self.mem[_pc + 2];
                self.c = self.mem[_pc + 1];
                self.pc += 2;
            }
            Opcode::STAXB => {
                let bc = join_bytes(self.b, self.c);
                self.mem[bc as usize] = self.a;
            }
            Opcode::INXB => (self.b, self.c) = split_bytes(join_bytes(self.b, self.c) + 1),
            Opcode::INRB => {
                check_flag_ac(self.b, self.b + 1, self);
                self.b += 1;
                check_flag_z(self.b, self);
                check_flag_s(self.b, self);
                check_flag_p(self.b, self);
            }
            Opcode::DCRB => {
                check_flag_ac(self.b, self.b - 1, self);
                self.b -= 1;
                check_flag_z(self.b, self);
                check_flag_s(self.b, self);
                check_flag_p(self.b, self);
            }
            Opcode::MVIB => {
                self.b = self.mem[_pc + 1];
                self.pc += 1;
            }
            Opcode::RLC => {
                check_flag_cy8((self.a as u16) << 1, self);
                self.a = self.a.rotate_left(1);
            }
            Opcode::DADB => {
                let hl = join_bytes(self.h, self.l);
                let bc = join_bytes(self.b, self.c);
                check_flag_cy16(hl as u32 + bc as u32, self);
                (self.h, self.l) = split_bytes(hl + bc);
            }
            Opcode::LDAXB => {
                let bc = join_bytes(self.b, self.c);
                self.a = self.mem[bc as usize];
            }
            Opcode::DCXB => (self.b, self.c) = split_bytes(join_bytes(self.b, self.c) - 1),
            Opcode::INRC => {
                check_flag_ac(self.c, self.c + 1, self);
                self.c += 1;
                check_flag_z(self.c, self);
                check_flag_s(self.c, self);
                check_flag_p(self.c, self);
            }
            Opcode::DCRC => {
                check_flag_ac(self.c, self.c - 1, self);
                self.c -= 1;
                check_flag_z(self.c, self);
                check_flag_s(self.c, self);
                check_flag_p(self.c, self);
            }
            Opcode::MVIC => {
                self.c = self.mem[_pc + 1];
                self.pc += 1;
            }
            Opcode::RRC => {
                check_flag_cy8((self.a as u16).rotate_right(1), self);
                self.a = self.a.rotate_right(1);
            }
            Opcode::LXID => {
                self.d = self.mem[_pc + 2];
                self.e = self.mem[_pc + 1];
                self.pc += 2;
            }
            Opcode::STAXD => {
                let de = join_bytes(self.d, self.e);
                self.mem[de as usize] = self.a;
            }
            Opcode::INXD => (self.d, self.e) = split_bytes(join_bytes(self.d, self.e) + 1),
            Opcode::INRD => {
                check_flag_ac(self.d, self.d + 1, self);
                self.d += 1;
                check_flag_z(self.d, self);
                check_flag_s(self.d, self);
                check_flag_p(self.d, self);
            }
            Opcode::DCRD => {
                check_flag_ac(self.d, self.d - 1, self);
                self.d -= 1;
                check_flag_z(self.d, self);
                check_flag_s(self.d, self);
                check_flag_p(self.d, self);
            }
            Opcode::MVID => {
                self.d = self.mem[_pc + 1];
                self.pc += 1;
            }
            Opcode::RAL => {
                let prev = self.flags.get(FlagType::CY);
                check_flag_cy8((self.a as u16) << 1, self);
                self.a = self.a << 1;
                self.a |= prev;
            }
            Opcode::DADD => {
                let hl = join_bytes(self.h, self.l);
                let de = join_bytes(self.d, self.e);
                check_flag_cy16(hl as u32 + de as u32, self);
                (self.h, self.l) = split_bytes(hl + de);
            }
            Opcode::LDAXD => {
                let de = join_bytes(self.d, self.e);
                self.a = self.mem[de as usize];
            }
            Opcode::DCXD => (self.d, self.e) = split_bytes(join_bytes(self.d, self.e) - 1),
            Opcode::INRE => {
                check_flag_ac(self.e, self.e + 1, self);
                self.e += 1;
                check_flag_z(self.e, self);
                check_flag_s(self.e, self);
                check_flag_p(self.e, self);
            }
            Opcode::DCRE => {
                check_flag_ac(self.e, self.e - 1, self);
                self.e -= 1;
                check_flag_z(self.e, self);
                check_flag_s(self.e, self);
                check_flag_p(self.e, self);
            }
            Opcode::MVIE => {
                self.e = self.mem[_pc + 1];
                self.pc += 1;
            }
            Opcode::RAR => {
                check_flag_cy8((self.a as u16).rotate_right(1), self);
                self.a = self.a >> 1;
                self.a |= (self.a << 1) & 0x80;
            }
            Opcode::LXIH => {
                self.h = self.mem[_pc + 2];
                self.l = self.mem[_pc + 1];
                self.pc += 2;
            }
            Opcode::SHLD => {
                let adr = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]);
                self.mem[adr as usize] = self.l;
                self.mem[(adr + 1) as usize] = self.h;
                self.pc += 2;
            }
            Opcode::INXH => (self.h, self.l) = split_bytes(join_bytes(self.h, self.l) + 1),
            Opcode::INRH => {
                check_flag_ac(self.h, self.h + 1, self);
                self.h += 1;
                check_flag_z(self.h, self);
                check_flag_s(self.h, self);
                check_flag_p(self.h, self);
            }
            Opcode::DCRH => {
                check_flag_ac(self.h, self.h - 1, self);
                self.h -= 1;
                check_flag_z(self.h, self);
                check_flag_s(self.h, self);
                check_flag_p(self.h, self);
            }
            Opcode::MVIH => {
                self.h = self.mem[_pc + 1];
                self.pc += 1;
            }
            Opcode::DAA => {
                if (self.a & 0xf) > 9 {
                    self.a += 0x06;
                }
            }
            Opcode::DADH => {
                let hl = join_bytes(self.h, self.l);
                check_flag_cy16((hl as u32) * 2, self);
                (self.h, self.l) = split_bytes(2 * hl);
            }
            Opcode::LHLD => {
                let adr = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]);
                self.l = self.mem[adr as usize];
                self.h = self.mem[(adr + 1) as usize];
                self.pc += 2;
            }
            Opcode::DCXH => (self.h, self.l) = split_bytes(join_bytes(self.h, self.l) - 1),
            Opcode::INRL => {
                check_flag_ac(self.l, self.l + 1, self);
                self.l += 1;
                check_flag_z(self.l, self);
                check_flag_s(self.l, self);
                check_flag_p(self.l, self);
            }
            Opcode::DCRL => {
                check_flag_ac(self.l, self.l - 1, self);
                self.l -= 1;
                check_flag_z(self.l, self);
                check_flag_s(self.l, self);
                check_flag_p(self.l, self);
            }
            Opcode::MVIL => {
                self.l = self.mem[_pc + 1];
                self.pc += 1;
            }
            Opcode::CMA => {
                self.a = !self.a;
            }
            Opcode::LXISP => {
                self.sp = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]);
                self.pc += 2;
            }
            Opcode::STA => {
                let adr = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]);
                self.mem[adr as usize] = self.a;
                self.pc += 2;
            }
            Opcode::INXSP => self.sp += 1,
            Opcode::INRM => {
                let adr = join_bytes(self.h, self.l) as usize;
                check_flag_ac(self.mem[adr], self.mem[adr] + 1, self);
                self.mem[adr] += 1;
                check_flag_z(self.mem[adr], self);
                check_flag_s(self.mem[adr], self);
                check_flag_p(self.mem[adr], self);
            }
            Opcode::DCRM => {
                let adr = join_bytes(self.h, self.l) as usize;
                check_flag_ac(self.mem[adr], self.mem[adr] - 1, self);
                self.mem[adr] -= 1;
                check_flag_z(self.mem[adr], self);
                check_flag_s(self.mem[adr], self);
                check_flag_p(self.mem[adr], self);
            }
            Opcode::MVIM => {
                let adr = join_bytes(self.h, self.l) as usize;
                self.mem[adr] = self.mem[_pc + 1];
                self.pc += 1;
            }
            Opcode::STC => self.flags.set(FlagType::CY),
            Opcode::DADSP => {
                let hl = join_bytes(self.h, self.l);
                check_flag_cy16((hl + self.sp) as u32, self);
                (self.h, self.l) = split_bytes(hl + self.sp);
            }
            Opcode::LDA => {
                let adr = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]);
                self.a = self.mem[adr as usize];
                self.pc += 2;
            }
            Opcode::DCXSP => self.sp -= 1,
            Opcode::INRA => {
                check_flag_ac(self.a, self.a + 1, self);
                self.a += 1;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::DCRA => {
                check_flag_ac(self.a, self.a - 1, self);
                self.a -= 1;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::MVIA => {
                self.a = self.mem[_pc + 1];
                self.pc += 1;
            }
            Opcode::CMC => match self.flags.get(FlagType::CY) == 0 {
                true => self.flags.set(FlagType::CY),
                false => self.flags.unset(FlagType::CY),
            },
            Opcode::MOVBB => self.b = self.b,
            Opcode::MOVBC => self.b = self.c,
            Opcode::MOVBD => self.b = self.d,
            Opcode::MOVBE => self.b = self.e,
            Opcode::MOVBH => self.b = self.h,
            Opcode::MOVBL => self.b = self.l,
            Opcode::MOVBM => self.b = self.mem[join_bytes(self.h, self.l) as usize],
            Opcode::MOVBA => self.b = self.a,
    
            Opcode::MOVCB => self.c = self.b,
            Opcode::MOVCC => self.c = self.c,
            Opcode::MOVCD => self.c = self.d,
            Opcode::MOVCE => self.c = self.e,
            Opcode::MOVCH => self.c = self.h,
            Opcode::MOVCL => self.c = self.l,
            Opcode::MOVCM => self.c = self.mem[join_bytes(self.h, self.l) as usize],
            Opcode::MOVCA => self.c = self.a,
    
            Opcode::MOVDB => self.d = self.b,
            Opcode::MOVDC => self.d = self.c,
            Opcode::MOVDD => self.d = self.d,
            Opcode::MOVDE => self.d = self.e,
            Opcode::MOVDH => self.d = self.h,
            Opcode::MOVDL => self.d = self.l,
            Opcode::MOVDM => self.d = self.mem[join_bytes(self.h, self.l) as usize],
            Opcode::MOVDA => self.d = self.a,
    
            Opcode::MOVEC => self.e = self.c,
            Opcode::MOVEB => self.e = self.b,
            Opcode::MOVED => self.e = self.d,
            Opcode::MOVEE => self.e = self.e,
            Opcode::MOVEH => self.e = self.h,
            Opcode::MOVEL => self.e = self.l,
            Opcode::MOVEM => self.e = self.mem[join_bytes(self.h, self.l) as usize],
            Opcode::MOVEA => self.e = self.a,
    
            Opcode::MOVHB => self.h = self.b,
            Opcode::MOVHC => self.h = self.c,
            Opcode::MOVHD => self.h = self.d,
            Opcode::MOVHE => self.h = self.e,
            Opcode::MOVHH => self.h = self.h,
            Opcode::MOVHL => self.h = self.l,
            Opcode::MOVHM => self.h = self.mem[join_bytes(self.h, self.l) as usize],
            Opcode::MOVHA => self.h = self.a,
    
            Opcode::MOVLB => self.l = self.b,
            Opcode::MOVLC => self.l = self.c,
            Opcode::MOVLD => self.l = self.d,
            Opcode::MOVLE => self.l = self.e,
            Opcode::MOVLH => self.l = self.h,
            Opcode::MOVLL => self.l = self.l,
            Opcode::MOVLM => self.l = self.mem[join_bytes(self.h, self.l) as usize],
            Opcode::MOVLA => self.l = self.a,
    
            Opcode::MOVMB => self.mem[join_bytes(self.h, self.l) as usize] = self.b,
            Opcode::MOVMC => self.mem[join_bytes(self.h, self.l) as usize] = self.c,
            Opcode::MOVMD => self.mem[join_bytes(self.h, self.l) as usize] = self.d,
            Opcode::MOVME => self.mem[join_bytes(self.h, self.l) as usize] = self.e,
            Opcode::MOVMH => self.mem[join_bytes(self.h, self.l) as usize] = self.h,
            Opcode::MOVML => self.mem[join_bytes(self.h, self.l) as usize] = self.l,
            Opcode::HLT => {
                println!("Halted");
                process::exit(-2)
            }
            Opcode::MOVMA => self.mem[join_bytes(self.h, self.l) as usize] = self.a,
    
            Opcode::MOVAB => self.a = self.b,
            Opcode::MOVAC => self.a = self.c,
            Opcode::MOVAD => self.a = self.d,
            Opcode::MOVAE => self.a = self.e,
            Opcode::MOVAH => self.a = self.h,
            Opcode::MOVAL => self.a = self.l,
            Opcode::MOVAM => self.a = self.mem[join_bytes(self.h, self.l) as usize],
            Opcode::MOVAA => self.a = self.a,
    
            Opcode::ADDB => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) + (self.b as u16), self);
                self.a += self.b;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ADDC => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) + (self.c as u16), self);
                self.a += self.c;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ADDD => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) + (self.d as u16), self);
                self.a += self.d;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ADDE => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) + (self.e as u16), self);
                self.a += self.e;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ADDH => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) + (self.h as u16), self);
                self.a += self.h;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ADDL => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) + (self.l as u16), self);
                self.a += self.l;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ADDM => {
                let adr = join_bytes(self.h, self.l) as usize;
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) + (self.mem[adr] as u16), self);
                self.a += self.mem[adr];
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ADDA => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) + (self.a as u16), self);
                self.a += self.a;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
    
            Opcode::ADCB => {
                // check_flag_ac(reg, self)
                check_flag_cy8(
                    (self.a as u16) + (self.b as u16) + (self.flags.get(FlagType::CY) as u16),
                    self,
                );
                self.a += self.b + self.flags.get(FlagType::CY);
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ADCC => {
                // check_flag_ac(reg, self)
                check_flag_cy8(
                    (self.a as u16) + (self.c as u16) + (self.flags.get(FlagType::CY) as u16),
                    self,
                );
                self.a += self.c + self.flags.get(FlagType::CY);
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ADCD => {
                // check_flag_ac(reg, self)
                check_flag_cy8(
                    (self.a as u16) + (self.d as u16) + (self.flags.get(FlagType::CY) as u16),
                    self,
                );
                self.a += self.d + self.flags.get(FlagType::CY);
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ADCE => {
                // check_flag_ac(reg, self)
                check_flag_cy8(
                    (self.a as u16) + (self.e as u16) + (self.flags.get(FlagType::CY) as u16),
                    self,
                );
                self.a += self.e + self.flags.get(FlagType::CY);
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ADCH => {
                // check_flag_ac(reg, self)
                check_flag_cy8(
                    (self.a as u16) + (self.h as u16) + (self.flags.get(FlagType::CY) as u16),
                    self,
                );
                self.a += self.h + self.flags.get(FlagType::CY);
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ADCL => {
                // check_flag_ac(reg, self)
                check_flag_cy8(
                    (self.a as u16) + (self.l as u16) + (self.flags.get(FlagType::CY) as u16),
                    self,
                );
                self.a += self.l + self.flags.get(FlagType::CY);
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ADCM => {
                let adr = join_bytes(self.h, self.l) as usize;
                // check_flag_ac(reg, self)
                check_flag_cy8(
                    (self.a as u16) + (self.mem[adr] as u16) + (self.flags.get(FlagType::CY) as u16),
                    self,
                );
                self.a += self.mem[adr] + self.flags.get(FlagType::CY);
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ADCA => {
                // check_flag_ac(reg, self)
                check_flag_cy8(
                    (self.a as u16) + (self.a as u16) + (self.flags.get(FlagType::CY) as u16),
                    self,
                );
                self.a += self.a + self.flags.get(FlagType::CY);
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
    
            Opcode::SUBB => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) - (self.b as u16), self);
                self.a -= self.b;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::SUBC => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) - (self.c as u16), self);
                self.a -= self.c;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::SUBD => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) - (self.d as u16), self);
                self.a -= self.d;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::SUBE => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) - (self.e as u16), self);
                self.a -= self.e;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::SUBH => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) - (self.h as u16), self);
                self.a -= self.h;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::SUBL => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) - (self.l as u16), self);
                self.a -= self.l;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::SUBM => {
                let adr = join_bytes(self.h, self.l) as usize;
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) - (self.mem[adr] as u16), self);
                self.a -= self.mem[adr];
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::SUBA => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) - (self.a as u16), self);
                self.a -= self.a;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
    
            Opcode::SBBB => {
                // check_flag_ac(reg, self)
                check_flag_cy8(
                    (self.a as u16) - (self.b as u16) - (self.flags.get(FlagType::CY) as u16),
                    self,
                );
                self.a -= self.b + self.flags.get(FlagType::CY);
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::SBBC => {
                // check_flag_ac(reg, self)
                check_flag_cy8(
                    (self.a as u16) - (self.c as u16) - (self.flags.get(FlagType::CY) as u16),
                    self,
                );
                self.a -= self.c + self.flags.get(FlagType::CY);
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::SBBD => {
                // check_flag_ac(reg, self)
                check_flag_cy8(
                    (self.a as u16) - (self.d as u16) - (self.flags.get(FlagType::CY) as u16),
                    self,
                );
                self.a -= self.d + self.flags.get(FlagType::CY);
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::SBBE => {
                // check_flag_ac(reg, self)
                check_flag_cy8(
                    (self.a as u16) - (self.e as u16) - (self.flags.get(FlagType::CY) as u16),
                    self,
                );
                self.a -= self.e + self.flags.get(FlagType::CY);
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::SBBH => {
                // check_flag_ac(reg, self)
                check_flag_cy8(
                    (self.a as u16) - (self.h as u16) - (self.flags.get(FlagType::CY) as u16),
                    self,
                );
                self.a -= self.h + self.flags.get(FlagType::CY);
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::SBBL => {
                // check_flag_ac(reg, self)
                check_flag_cy8(
                    (self.a as u16) - (self.l as u16) - (self.flags.get(FlagType::CY) as u16),
                    self,
                );
                self.a -= self.l + self.flags.get(FlagType::CY);
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::SBBM => {
                let adr = join_bytes(self.h, self.l) as usize;
                // check_flag_ac(reg, self)
                check_flag_cy8(
                    (self.a as u16) - (self.mem[adr] as u16) - (self.flags.get(FlagType::CY) as u16),
                    self,
                );
                self.a -= self.mem[adr] + self.flags.get(FlagType::CY);
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::SBBA => {
                // check_flag_ac(reg, self)
                check_flag_cy8(
                    (self.a as u16) - (self.a as u16) - (self.flags.get(FlagType::CY) as u16),
                    self,
                );
                self.a -= self.a + self.flags.get(FlagType::CY);
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
    
            Opcode::ANAB => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) & (self.b as u16), self);
                self.a &= self.b;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ANAC => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) & (self.c as u16), self);
                self.a &= self.c;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ANAD => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) & (self.d as u16), self);
                self.a &= self.d;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ANAE => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) & (self.e as u16), self);
                self.a &= self.e;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ANAH => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) & (self.h as u16), self);
                self.a &= self.h;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ANAL => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) & (self.l as u16), self);
                self.a &= self.l;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ANAM => {
                let adr = join_bytes(self.h, self.l) as usize;
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) & (self.mem[adr] as u16), self);
                self.a &= self.mem[adr];
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ANAA => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) & (self.a as u16), self);
                self.a &= self.a;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
    
            Opcode::XRAB => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) ^ (self.b as u16), self);
                self.a ^= self.b;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::XRAC => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) ^ (self.c as u16), self);
                self.a ^= self.c;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::XRAD => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) ^ (self.d as u16), self);
                self.a ^= self.d;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::XRAE => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) ^ (self.e as u16), self);
                self.a ^= self.e;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::XRAH => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) ^ (self.h as u16), self);
                self.a ^= self.h;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::XRAL => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) ^ (self.l as u16), self);
                self.a ^= self.l;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::XRAM => {
                let adr = join_bytes(self.h, self.l) as usize;
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) ^ (self.mem[adr] as u16), self);
                self.a ^= self.mem[adr];
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::XRAA => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) ^ (self.a as u16), self);
                self.a ^= self.a;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
    
            Opcode::ORAB => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) | (self.b as u16), self);
                self.a |= self.b;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ORAC => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) | (self.c as u16), self);
                self.a |= self.c;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ORAD => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) | (self.d as u16), self);
                self.a |= self.d;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ORAE => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) | (self.e as u16), self);
                self.a |= self.e;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ORAH => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) | (self.h as u16), self);
                self.a |= self.h;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ORAL => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) | (self.l as u16), self);
                self.a |= self.l;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ORAM => {
                let adr = join_bytes(self.h, self.l) as usize;
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) | (self.mem[adr] as u16), self);
                self.a |= self.mem[adr];
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
            Opcode::ORAA => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) | (self.a as u16), self);
                self.a |= self.a;
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
            }
    
            Opcode::CMPB => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) - (self.b as u16), self);
                check_flag_z(self.a - self.b, self);
                check_flag_s(self.a - self.b, self);
                check_flag_p(self.a - self.b, self);
            }
            Opcode::CMPC => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) - (self.c as u16), self);
                check_flag_z(self.a - self.c, self);
                check_flag_s(self.a - self.c, self);
                check_flag_p(self.a - self.c, self);
            }
            Opcode::CMPD => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) - (self.d as u16), self);
                check_flag_z(self.a - self.d, self);
                check_flag_s(self.a - self.d, self);
                check_flag_p(self.a - self.d, self);
            }
            Opcode::CMPE => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) - (self.e as u16), self);
                check_flag_z(self.a - self.e, self);
                check_flag_s(self.a - self.e, self);
                check_flag_p(self.a - self.e, self);
            }
            Opcode::CMPH => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) - (self.h as u16), self);
                check_flag_z(self.a - self.h, self);
                check_flag_s(self.a - self.h, self);
                check_flag_p(self.a - self.h, self);
            }
            Opcode::CMPL => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) - (self.l as u16), self);
                check_flag_z(self.a - self.l, self);
                check_flag_s(self.a - self.l, self);
                check_flag_p(self.a - self.l, self);
            }
            Opcode::CMPM => {
                let adr = join_bytes(self.h, self.l) as usize;
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) - (self.mem[adr] as u16), self);
                check_flag_z(self.a - self.mem[adr], self);
                check_flag_s(self.a - self.mem[adr], self);
                check_flag_p(self.a - self.mem[adr], self);
            }
            Opcode::CMPA => {
                // check_flag_ac(reg, self)
                check_flag_cy8((self.a as u16) - (self.a as u16), self);
                check_flag_z(self.a - self.a, self);
                check_flag_s(self.a - self.a, self);
                check_flag_p(self.a - self.a, self);
            }
    
            Opcode::RNZ => {
                if self.flags.get(FlagType::Z) != 0b1 {
                    self.pc = join_bytes(
                        self.mem[self.sp as usize + 1],
                        self.mem[self.sp as usize],
                    ) - 1;
                    self.sp += 2;
                }
            }
            Opcode::POPB => {
                self.c = self.mem[self.sp as usize];
                self.b = self.mem[self.sp as usize + 1];
                self.sp += 2;
            }
            Opcode::JNZ => {
                if self.flags.get(FlagType::Z) != 0b1 {
                    self.pc = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]) - 1;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::JMP => {
                self.pc = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]) - 1;
            }
            Opcode::CNZ => {
                if self.flags.get(FlagType::Z) != 0b1 {
                    let (pchi, pclo) = split_bytes(self.pc + 3);
                    self.mem[self.sp as usize - 1] = pchi;
                    self.mem[self.sp as usize - 2] = pclo;
                    self.sp -= 2;
                    self.pc = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]) - 1;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::PUSHB => {
                self.mem[self.sp as usize - 1] = self.b;
                self.mem[self.sp as usize - 2] = self.c;
                self.sp -= 2;
            }
            Opcode::ADI => {
                check_flag_cy8((self.a as u16) + (self.mem[_pc + 1] as u16), self);
                self.a += self.mem[_pc + 1];
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
                self.pc += 1;
            }
            Opcode::RST0 => {
                let (pchi, pclo) = split_bytes(self.pc + 1);
                self.mem[self.sp as usize - 1] = pchi;
                self.mem[self.sp as usize - 2] = pclo;
                self.sp -= 2;
                self.pc = 0xff; // 0x00 - 1 due to end pc increment
            }
    
            Opcode::RZ => {
                if self.flags.get(FlagType::Z) == 0b1 {
                    self.pc = join_bytes(
                        self.mem[self.sp as usize + 1],
                        self.mem[self.sp as usize],
                    ) - 1;
                    self.sp += 2;
                }
            }
            Opcode::RET => {
                self.pc = join_bytes(
                    self.mem[self.sp as usize + 1],
                    self.mem[self.sp as usize],
                ) - 1;
                self.sp += 2;
            }
            Opcode::JZ => {
                if self.flags.get(FlagType::Z) == 0b1 {
                    self.pc = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]) - 1;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::CZ => {
                if self.flags.get(FlagType::Z) == 0b1 {
                    let (pchi, pclo) = split_bytes(self.pc + 3);
                    self.mem[self.sp as usize - 1] = pchi;
                    self.mem[self.sp as usize - 2] = pclo;
                    self.sp -= 2;
                    self.pc = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]) - 1;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::CALL => {
                let (pchi, pclo) = split_bytes(self.pc + 3);
                self.mem[self.sp as usize - 1] = pchi;
                self.mem[self.sp as usize - 2] = pclo;
                self.sp -= 2;
                self.pc = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]) - 1;
            }
            Opcode::ACI => {
                check_flag_cy8(
                    (self.a as u16)
                        + (self.mem[_pc + 1] as u16)
                        + (self.flags.get(FlagType::CY) as u16),
                    self,
                );
                self.a += self.mem[_pc + 1] + self.flags.get(FlagType::CY);
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
                self.pc += 1;
            }
            Opcode::RST1 => {
                let (pchi, pclo) = split_bytes(self.pc + 1);
                self.mem[self.sp as usize - 1] = pchi;
                self.mem[self.sp as usize - 2] = pclo;
                self.sp -= 2;
                self.pc = 0x08 - 1;
            }
    
            Opcode::RNC => {
                if self.flags.get(FlagType::CY) != 0b1 {
                    self.pc = join_bytes(
                        self.mem[self.sp as usize + 1],
                        self.mem[self.sp as usize],
                    ) - 1;
                    self.sp += 2;
                }
            }
            Opcode::POPD => {
                self.e = self.mem[self.sp as usize];
                self.d = self.mem[self.sp as usize + 1];
                self.sp += 2;
            }
            Opcode::JNC => {
                if self.flags.get(FlagType::CY) != 0b1 {
                    self.pc = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]) - 1;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::OUT => {
                let port = self.mem[_pc + 1];
                io.machine_out(port, self.a); 
                self.pc += 1;
            },
            Opcode::CNC => {
                if self.flags.get(FlagType::CY) != 0b1 {
                    let (pchi, pclo) = split_bytes(self.pc + 3);
                    self.mem[self.sp as usize - 1] = pchi;
                    self.mem[self.sp as usize - 2] = pclo;
                    self.sp -= 2;
                    self.pc = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]) - 1;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::PUSHD => {
                self.mem[self.sp as usize - 1] = self.d;
                self.mem[self.sp as usize - 2] = self.e;
                self.sp -= 2;
            }
            Opcode::SUI => {
                check_flag_cy8((self.a as u16) - (self.mem[_pc + 1] as u16), self);
                self.a -= self.mem[_pc + 1];
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
                self.pc += 1;
            }
            Opcode::RST2 => {
                let (pchi, pclo) = split_bytes(self.pc + 1);
                self.mem[self.sp as usize - 1] = pchi;
                self.mem[self.sp as usize - 2] = pclo;
                self.sp -= 2;
                self.pc = 0x10 - 1;
            }
    
            Opcode::RC => {
                if self.flags.get(FlagType::CY) == 0b1 {
                    self.pc = join_bytes(
                        self.mem[self.sp as usize + 1],
                        self.mem[self.sp as usize],
                    ) - 1;
                    self.sp += 2;
                }
            }
            Opcode::JC => {
                if self.flags.get(FlagType::CY) == 0b1 {
                    self.pc = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]) - 1;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::IN => {
                let port = self.mem[_pc + 1];
                self.a = io.machine_in(port);
                self.pc += 1;
            }
            Opcode::CC => {
                if self.flags.get(FlagType::CY) == 0b1 {
                    let (pchi, pclo) = split_bytes(self.pc + 3);
                    self.mem[self.sp as usize - 1] = pchi;
                    self.mem[self.sp as usize - 2] = pclo;
                    self.sp -= 2;
                    self.pc = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]) - 1;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::SBI => {
                check_flag_cy8(
                    (self.a as u16)
                        - (self.mem[_pc + 1] as u16)
                        - (self.flags.get(FlagType::CY) as u16),
                    self,
                );
                self.a -= self.mem[_pc + 1] + self.flags.get(FlagType::CY);
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
                self.pc += 1;
            }
            Opcode::RST3 => {
                let (pchi, pclo) = split_bytes(self.pc + 1);
                self.mem[self.sp as usize - 1] = pchi;
                self.mem[self.sp as usize - 2] = pclo;
                self.sp -= 2;
                self.pc = 0x18 - 1;
            }
    
            Opcode::RPO => {
                if self.flags.get(FlagType::P) != 0b1 {
                    self.pc = join_bytes(
                        self.mem[self.sp as usize + 1],
                        self.mem[self.sp as usize],
                    ) - 1;
                    self.sp += 2;
                }
            }
            Opcode::POPH => {
                self.l = self.mem[self.sp as usize];
                self.h = self.mem[self.sp as usize + 1];
                self.sp += 2;
            }
            Opcode::JPO => {
                if self.flags.get(FlagType::P) != 0b1 {
                    self.pc = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]) - 1;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::XTHL => {
                let tempsp = self.mem[self.sp as usize];
                let tempsp1 = self.mem[self.sp as usize + 1];
                self.mem[self.sp as usize] = self.l;
                self.mem[self.sp as usize + 1] = self.h;
                self.l = tempsp;
                self.h = tempsp1;
            }
            Opcode::CPO => {
                if self.flags.get(FlagType::P) != 0b1 {
                    let (pchi, pclo) = split_bytes(self.pc + 3);
                    self.mem[self.sp as usize - 1] = pchi;
                    self.mem[self.sp as usize - 2] = pclo;
                    self.sp -= 2;
                    self.pc = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]) - 1;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::PUSHH => {
                self.mem[self.sp as usize - 1] = self.h;
                self.mem[self.sp as usize - 2] = self.l;
                self.sp -= 2;
            }
            Opcode::ANI => {
                check_flag_cy8((self.a as u16) & (self.mem[_pc + 1] as u16), self);
                self.a &= self.mem[_pc + 1];
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
                self.pc += 1;
            }
            Opcode::RST4 => {
                let (pchi, pclo) = split_bytes(self.pc + 1);
                self.mem[self.sp as usize - 1] = pchi;
                self.mem[self.sp as usize - 2] = pclo;
                self.sp -= 2;
                self.pc = 0x20 - 1;
            }
    
            Opcode::RPE => {
                if self.flags.get(FlagType::P) == 0b1 {
                    self.pc = join_bytes(
                        self.mem[self.sp as usize + 1],
                        self.mem[self.sp as usize],
                    ) - 1;
                    self.sp += 2;
                }
            }
            Opcode::PCHL => {
                self.pc = join_bytes(self.h, self.l) - 1;
            }
            Opcode::JPE => {
                if self.flags.get(FlagType::P) == 0b1 {
                    self.pc = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]) - 1;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::XCHG => {
                let temph = self.h;
                let templ = self.l;
                self.h = self.d;
                self.l = self.e;
                self.d = temph;
                self.e = templ;
            }
            Opcode::CPE => {
                if self.flags.get(FlagType::P) == 0b1 {
                    let (pchi, pclo) = split_bytes(self.pc + 3);
                    self.mem[self.sp as usize - 1] = pchi;
                    self.mem[self.sp as usize - 2] = pclo;
                    self.sp -= 2;
                    self.pc = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]) - 1;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::XRI => {
                check_flag_cy8((self.a as u16) ^ (self.mem[_pc + 1] as u16), self);
                self.a ^= self.mem[_pc + 1];
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
                self.pc += 1;
            }
            Opcode::RST5 => {
                let (pchi, pclo) = split_bytes(self.pc + 1);
                self.mem[self.sp as usize - 1] = pchi;
                self.mem[self.sp as usize - 2] = pclo;
                self.sp -= 2;
                self.pc = 0x28 - 1;
            }
    
            Opcode::RP => {
                if self.flags.get(FlagType::S) != 0b1 {
                    self.pc = join_bytes(
                        self.mem[self.sp as usize + 1],
                        self.mem[self.sp as usize],
                    ) - 1;
                    self.sp += 2;
                }
            }
            Opcode::POPPSW => {
                self.flags.reg = self.mem[self.sp as usize];
                self.a = self.mem[self.sp as usize + 1];
                self.sp += 2;
            }
            Opcode::JP => {
                if self.flags.get(FlagType::S) != 0b1 {
                    self.pc = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]) - 1;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::DI => {
                self.enable = 0;
            }
            Opcode::CP => {
                if self.flags.get(FlagType::S) != 0b1 {
                    self.pc = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]) - 1;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::PUSHPSW => {
                self.mem[self.sp as usize - 2] = self.flags.reg;
                self.mem[self.sp as usize - 1] = self.a;
                self.sp -= 2;
            }
            Opcode::ORI => {
                check_flag_cy8((self.a as u16) | (self.mem[_pc + 1] as u16), self);
                self.a |= self.mem[_pc + 1];
                check_flag_z(self.a, self);
                check_flag_s(self.a, self);
                check_flag_p(self.a, self);
                self.pc += 1;
            }
            Opcode::RST6 => {
                let (pchi, pclo) = split_bytes(self.pc + 1);
                self.mem[self.sp as usize - 1] = pchi;
                self.mem[self.sp as usize - 2] = pclo;
                self.sp -= 2;
                self.pc = 0x30 - 1;
            }
    
            Opcode::RM => {
                if self.flags.get(FlagType::S) == 0b1 {
                    self.pc = join_bytes(
                        self.mem[self.sp as usize + 1],
                        self.mem[self.sp as usize],
                    ) - 1;
                    self.sp += 2;
                }
            }
            Opcode::SPHL => {
                self.sp = join_bytes(self.h, self.l);
            }
            Opcode::JM => {
                if self.flags.get(FlagType::S) == 0b1 {
                    self.pc = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]) - 1;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::EI => {
                self.enable = 1;
            }
            Opcode::CM => {
                if self.flags.get(FlagType::S) == 0b1 {
                    let (pchi, pclo) = split_bytes(self.pc + 3);
                    self.mem[self.sp as usize - 1] = pchi;
                    self.mem[self.sp as usize - 2] = pclo;
                    self.sp -= 2;
                    self.pc = join_bytes(self.mem[_pc + 2], self.mem[_pc + 1]) - 1;
                } else {
                    self.pc += 2;
                }
            }
            Opcode::CPI => {
                check_flag_cy8((self.a as u16) - (self.mem[_pc + 1] as u16), self);
                check_flag_z(self.a - self.mem[_pc + 1], self);
                check_flag_s(self.a - self.mem[_pc + 1], self);
                check_flag_p(self.a - self.mem[_pc + 1], self);
                self.pc += 1;
            }
            Opcode::RST7 => {
                let (pchi, pclo) = split_bytes(self.pc + 1);
                self.mem[self.sp as usize - 1] = pchi;
                self.mem[self.sp as usize - 2] = pclo;
                self.sp -= 2;
                self.pc = 0x38 - 1;
            }
    
            // Not implemented Instructions
            Opcode::NIMP(x) => {
                println!("Instruction {:#04x} not implemented", x);
                process::exit(-1);
            }
        };
    
        self.pc += 1;
    }
}
