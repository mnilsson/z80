//
pub mod instruction;
pub mod traits;

use bus::Bus;
pub struct Disassembler {
    pub bus: Box<Bus>,
    pub pc: u16
}

impl Disassembler {
    pub fn next_byte(&self) -> u8 {
        self.bus.memory_read((self.pc + 1) as usize)
    }

    pub fn next_word(&self) -> u16 {
        let lo = self.bus.memory_read((self.pc + 1) as usize);
        let hi = self.bus.memory_read((self.pc + 2) as usize);
        lo as u16 | ((hi as u16) << 8)
    }
}

use cpu::{Write8, Write16, Read8, Read16, Source, ReadCond};
use operations::Ops;
use registers::Reg16;
use registers::ReadAddress;
use self::instruction::Instruction;

#[allow(unused)]
impl<'a> Ops for &'a Disassembler {
    type R = Instruction;
    fn and<R: Read8>(self, reg: R) -> Self::R {
        Instruction::AND(reg.into_arg8(self))
    }
    fn add8<D: Write8 + Read8 + Copy, S: Read8>(self, dest: D, source: S) -> Self::R {
        Instruction::NOP
    }
    fn adc8<D: Write8 + Read8 + Copy, S: Read8>(self, dest: D, source: S) -> Self::R{         Instruction::NOP     }
    fn add16<D: Write16 + Read16 + Copy, S: Read16>(self, dest: D, source: S) -> Self::R{
        Instruction::ADD16(dest.into_arg16(self), source.into_arg16(self))
    }

    fn call<A: Read16>(self, addr: A) -> Self::R {
        Instruction::CALL(addr.into_address(self))
    }
    
    fn call_cond<C: ReadCond, A: Read16>(self, condition: C, addr: A) -> Self::R {
        Instruction::CALL_COND(condition.into_cond(self), addr.into_address(self))
    }

    fn ccf(self) -> Self::R{         Instruction::NOP     }
    fn cp<S: Read8>(self, source: S) -> Self::R {
        Instruction::CP(source.into_arg8(self))
    }
    fn cpl(self) -> Self::R{         Instruction::NOP     }
    fn daa(self) -> Self::R{         Instruction::NOP     }
    fn dec8<R: Write8 + Read8 + Copy>(self, reg: R) -> Self::R{         Instruction::NOP     }

    fn dec16<R: Write16 + Read16 + Copy>(self, reg: R) -> Self::R {
        Instruction::DEC16(reg.into_arg16(self))
    }
    
    fn di(self) -> Self::R{         Instruction::NOP     }
    fn ei(self) -> Self::R{         Instruction::NOP     }
    fn ex<D: Write16 + Read16 + Copy, S: Write16 + Read16 + Copy>(
        self,
        dest: D,
        source: S,
    ) -> Self::R {
        Instruction::EX(dest.into_arg16(self), source.into_arg16(self))
    }
    fn exx(self) -> Self::R{         Instruction::NOP     }
    fn halt(self) -> Self::R{         Instruction::NOP     }

    fn in8<D: Write8, S: Read8>(self, dest: D, source: S) -> Self::R{         Instruction::NOP     }

    fn inc8<R: Write8 + Read8 + Copy>(self, reg: R) -> Self::R{         Instruction::NOP     }
    fn inc8_memory<R: ReadAddress>(self, reg: R) -> Self::R{         Instruction::NOP     }
    fn dec8_memory<R: ReadAddress>(self, reg: R) -> Self::R{         Instruction::NOP     }
    fn inc16<R: Write16 + Read16 + Copy>(self, reg: R) -> Self::R{
        Instruction::INC16(reg.into_arg16(self))
    }

    fn jp<A: Read16>(self, addr: A) -> Self::R {
        Instruction::JP(addr.into_address(self))
    }

    fn jp_cond<C: ReadCond, A: Read16>(self, condition: C, addr: A) -> Self::R{
        Instruction::JP_COND(condition.into_cond(self), addr.into_address(self))
        // Instruction::NOP
    }

    fn jr<C: Source<bool>>(self, condition: C) -> Self::R{
                 Instruction::NOP     
        // Instruction::JR()
    }
    fn djnz(self) -> Self::R{         Instruction::NOP     }
    fn ret(self) -> Self::R {
        Instruction::RET    
    }
    fn ret_cond<C: Source<bool>>(self, condition: C) -> Self::R{         Instruction::NOP     }
    fn ld8<D: Write8, S: Read8>(self, dest: D, source: S) -> Self::R{
        Instruction::LD8(dest.into_arg8(&self), source.into_arg8(&self))     
    }
    fn ld8_address_dest<D: ReadAddress, S: Read8>(self, dest: D, source: S) -> Self::R{
        // Instruction::LD8(dest.into_arg8(&self), source.into_arg8(&self))     
        Instruction::NOP
    }

    fn ld8_address_source<D: Write8, S: ReadAddress>(self, dest: D, source: S) -> Self::R{
        // Instruction::LD8(dest.into_arg8(&self), source.into_arg8(&self))     
        Instruction::NOP
    }
    fn ld16<D: Write16, S: Read16>(self, dest: D, source: S) -> Self::R{
        Instruction::LD16(dest.into_arg16(&self), source.into_arg16(&self))
    }

    fn nop(self) -> Self::R{         Instruction::NOP     }

    fn out8<D: Read8, S: Read8>(self, dest: D, source: S) -> Self::R {
        Instruction::OUT(dest.into_arg8(self), source.into_arg8(self))
        // Instruction::NOP
    }

    fn or<R: Read8>(self, reg: R) -> Self::R {
        Instruction::OR(reg.into_arg8(self))
    }

    fn rla(self) -> Self::R{         Instruction::NOP     }
    fn rlca(self) -> Self::R { Instruction::RLCA }
    fn rra(self) -> Self::R{         Instruction::NOP     }
    fn rrca(self) -> Self::R{         Instruction::NOP     }
    fn scf(self) -> Self::R{         Instruction::NOP     }

    fn xor<R: Read8>(self, reg: R) -> Self::R{         Instruction::NOP     }

    fn sub8<S: Read8>(self, source: S) -> Self::R{         Instruction::NOP     }
    fn sbc8<S: Read8>(self, source: S) -> Self::R{         Instruction::NOP     }

    fn sbc16<D: Write16 + Read16 + Copy, S: Read16>(
        self,
        dest: D,
        source: S,
    ) -> Self::R{         Instruction::NOP     }
    fn adc16<D: Write16 + Read16 + Copy, S: Read16>(
        self,
        dest: D,
        source: S,
    ) -> Self::R{         Instruction::NOP     }
    fn neg(self) -> Self::R{         Instruction::NOP     }
    fn retn(self) -> Self::R{         Instruction::NOP     }
    fn cpd(self) -> Self::R{         Instruction::NOP     }
    fn cpi(self) -> Self::R{         Instruction::NOP     }
    fn ind(self) -> Self::R{         Instruction::NOP     }
    fn outd(self) -> Self::R{         Instruction::NOP     }
    fn ldir(self) -> Self::R {
        Instruction::LDIR
    }
    fn cpir(self) -> Self::R{         Instruction::NOP     }
    fn inir(self) -> Self::R{         Instruction::NOP     }
    fn lddr(self) -> Self::R{         Instruction::NOP     }
    fn cpdr(self) -> Self::R{         Instruction::NOP     }
    fn indr(self) -> Self::R{         Instruction::NOP     }
    fn otdr(self) -> Self::R{         Instruction::NOP     }
    fn outi(self) -> Self::R{         Instruction::NOP     }
    fn otir(self) -> Self::R{         Instruction::NOP     }
    fn ldd(self) -> Self::R{         Instruction::NOP     }
    fn ini(self) -> Self::R{         Instruction::NOP     }
    fn im(self, im: u8) -> Self::R{         Instruction::NOP     }
    fn rrd(self) -> Self::R{         Instruction::NOP     }
    fn rld(self) -> Self::R{         Instruction::NOP     }
    fn reti(self) -> Self::R{         Instruction::NOP     }
    fn ldi(self) -> Self::R{         Instruction::NOP     }

    fn pop<T: Write16>(self, target: T) -> Self::R{
        Instruction::POP(target.into_arg16(self)) 
    }
    fn push<S: Read16>(self, source: S) -> Self::R {
        Instruction::PUSH(source.into_arg16(self))
    }
    

    fn rst(self, byte: u8) -> Self::R { Instruction::RST(byte) }

    fn srl<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R{         Instruction::NOP     }
    fn sll<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R{         Instruction::NOP     }
    fn sra<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R{         Instruction::NOP     }
    fn sla<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R{         Instruction::NOP     }
    fn rlc<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R{         Instruction::NOP     }
    fn rrc<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R{         Instruction::NOP     }
    fn bit<S: Read8>(self, bit: u8, source: S) -> Self::R{         Instruction::NOP     }
    fn rl<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R{         Instruction::NOP     }
    fn res<S: Read8 + Write8 + Copy>(self, bit: u8, source: S) -> Self::R{         Instruction::NOP     }
    fn rr<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R{         Instruction::NOP     }
    fn set<S: Read8 + Write8 + Copy>(self, bit: u8, source: S) -> Self::R{         Instruction::NOP     }

    fn cb_op(self) -> Self::R{         Instruction::NOP     }
    
    fn dd_op(self) -> Self::R{         Instruction::NOP     }
    fn ed_op(self) -> Self::R{         Instruction::NOP     }
    fn fd_op(self) -> Self::R{         Instruction::NOP     }
    fn dd_fd_cb_op(self, ireg: Reg16) -> Self::R{         Instruction::NOP     }
}
