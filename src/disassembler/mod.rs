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

use cpu::{Write8, Write16, Read8, Read16, ReadCond, ImmByte};
use operations::Ops;
use operations::{decode_cb, decode_dd, decode_ed, decode_fd, decode_dd_fd_cb};
use registers::Reg16;
use registers::ReadAddress;
use self::instruction::Instruction;
use self::traits::IntoArg8;

#[allow(unused)]
impl<'a> Ops for &'a Disassembler {
    type R = Instruction;
    fn and<R: Read8>(self, reg: R) -> Self::R {
        Instruction::AND(reg.into_arg8(self))
    }
    fn add8<D: Write8 + Read8 + Copy, S: Read8>(self, dest: D, source: S) -> Self::R {
        Instruction::ADD8(dest.into_arg8(self), source.into_arg8(self))
    }
    fn adc8<D: Write8 + Read8 + Copy, S: Read8>(self, dest: D, source: S) -> Self::R {
        Instruction::ADC8(dest.into_arg8(self), source.into_arg8(self))
    }
    fn add16<D: Write16 + Read16 + Copy, S: Read16>(self, dest: D, source: S) -> Self::R{
        Instruction::ADD16(dest.into_arg16(self), source.into_arg16(self))
    }

    fn call<A: Read16>(self, addr: A) -> Self::R {
        Instruction::CALL(addr.into_address(self))
    }
    
    fn call_cond<C: ReadCond, A: Read16>(self, condition: C, addr: A) -> Self::R {
        Instruction::CALL_COND(condition.into_cond(self), addr.into_address(self))
    }

    fn ccf(self) -> Self::R { Instruction::CCF }
    fn cp<S: Read8>(self, source: S) -> Self::R {
        Instruction::CP(source.into_arg8(self))
    }
    fn cpl(self) -> Self::R { Instruction::CPL }
    fn daa(self) -> Self::R { Instruction::DAA }
    fn dec8<R: Write8 + Read8 + Copy>(self, reg: R) -> Self::R { Instruction::DEC8(reg.into_arg8(self)) }

    fn dec16<R: Write16 + Read16 + Copy>(self, reg: R) -> Self::R {
        Instruction::DEC16(reg.into_arg16(self))
    }
    
    fn di(self) -> Self::R { Instruction::DI }
    fn ei(self) -> Self::R { Instruction::EI }
    fn ex<D: Write16 + Read16 + Copy, S: Write16 + Read16 + Copy>(
        self,
        dest: D,
        source: S,
    ) -> Self::R {
        Instruction::EX(dest.into_arg16(self), source.into_arg16(self))
    }
    fn exx(self) -> Self::R { Instruction::EXX }
    fn halt(self) -> Self::R { Instruction::HALT }

    fn in8<D: Write8, S: Read8>(self, dest: D, source: S) -> Self::R {
        Instruction::IN(dest.into_arg8(self), source.into_arg8(self))
    }
    fn in8_noflags<D: Write8, S: Read8>(self, dest: D, source: S) -> Self::R { self.in8(dest, source) }

    fn inc8<R: Write8 + Read8 + Copy>(self, reg: R) -> Self::R { Instruction::INC8(reg.into_arg8(self))}
    fn inc8_memory<R: ReadAddress>(self, reg: R) -> Self::R {

//        Instruction::INC8_MEMORY(reg.read_address(self).into_address(self))
        Instruction::NOP
    }
    fn dec8_memory<R: ReadAddress>(self, reg: R) -> Self::R{         Instruction::NOP     }
    fn inc16<R: Write16 + Read16 + Copy>(self, reg: R) -> Self::R{
        Instruction::INC16(reg.into_arg16(self))
    }

    fn jp<A: Read16>(self, addr: A) -> Self::R {
        Instruction::JP(addr.into_address(self))
    }

    fn jp_cond<C: ReadCond, A: Read16>(self, condition: C, addr: A) -> Self::R{
        Instruction::JP_COND(condition.into_cond(self), addr.into_address(self))
    }

    fn jr<C: ReadCond>(self, condition: C) -> Self::R{
        Instruction::JR_COND(condition.into_cond(self), self.next_byte())
    }
    fn djnz(self) -> Self::R{ Instruction::DJNZ(ImmByte.into_arg8(self)) }
    fn ret(self) -> Self::R {
        Instruction::RET    
    }
    fn ret_cond<C: ReadCond>(self, condition: C) -> Self::R { Instruction::RET_COND(condition.into_cond(self)) }
    fn ld8<D: Write8, S: Read8>(self, dest: D, source: S) -> Self::R{
        Instruction::LD8(dest.into_arg8(self), source.into_arg8(self))
    }
    fn ld8_int<D: Write8, S: Read8>(self, dest: D, source: S) -> Self::R{
        Instruction::LD8(dest.into_arg8(self), source.into_arg8(self))
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
        Instruction::LD16(dest.into_arg16(self), source.into_arg16(self))
    }

    fn nop(self) -> Self::R { Instruction::NOP }

    fn out8<D: Read8, S: Read8>(self, dest: D, source: S) -> Self::R {
        Instruction::OUT(dest.into_arg8(self), source.into_arg8(self))
    }

    fn out8_noflags<D: Read8, S: Read8>(self, dest: D, source: S) -> Self::R {
        Instruction::OUT(dest.into_arg8(self), source.into_arg8(self))
    }

    fn or<R: Read8>(self, reg: R) -> Self::R {
        Instruction::OR(reg.into_arg8(self))
    }

    fn rla(self) -> Self::R { Instruction::RLA }
    fn rlca(self) -> Self::R { Instruction::RLCA }
    fn rra(self) -> Self::R { Instruction::RRA }
    fn rrca(self) -> Self::R { Instruction::RRCA }
    fn scf(self) -> Self::R{ Instruction::SCF }

    fn xor<R: Read8>(self, reg: R) -> Self::R { Instruction::XOR(reg.into_arg8(self)) }

    fn sub8<S: Read8>(self, source: S) -> Self::R { Instruction::SUB8(source.into_arg8(self)) }
    fn sbc8<S: Read8>(self, source: S) -> Self::R { Instruction::SBC8(source.into_arg8(self)) }

    fn sbc16<D: Write16 + Read16 + Copy, S: Read16>(
        self,
        dest: D,
        source: S,
    ) -> Self::R { Instruction::SBC16(dest.into_arg16(self), source.into_arg16(self)) }
    fn adc16<D: Write16 + Read16 + Copy, S: Read16>(
        self,
        dest: D,
        source: S,
    ) -> Self::R { Instruction::ADC16(dest.into_arg16(self), source.into_arg16(self)) }
    fn neg(self) -> Self::R { Instruction::NEG }
    fn retn(self) -> Self::R { Instruction::RETN }
    fn cpd(self) -> Self::R { Instruction::CPD }
    fn cpi(self) -> Self::R { Instruction::CPI }
    fn ind(self) -> Self::R { Instruction::IND }
    fn outd(self) -> Self::R { Instruction::OUTD }
    fn ldir(self) -> Self::R {
        Instruction::LDIR
    }
    fn cpir(self) -> Self::R { Instruction::CPIR }
    fn inir(self) -> Self::R { Instruction::INIR }
    fn lddr(self) -> Self::R { Instruction::LDDR }
    fn cpdr(self) -> Self::R { Instruction::CPDR }
    fn indr(self) -> Self::R { Instruction::INDR }
    fn otdr(self) -> Self::R { Instruction::OTDR }
    fn outi(self) -> Self::R { Instruction::OUTI }
    fn otir(self) -> Self::R { Instruction::OTIR }
    fn ldd(self) -> Self::R { Instruction::LDD }
    fn ini(self) -> Self::R { Instruction::INI }
    fn im(self, im: u8) -> Self::R { Instruction::IM }
    fn rrd(self) -> Self::R { Instruction::RRD }
    fn rld(self) -> Self::R { Instruction::RLD }
    fn reti(self) -> Self::R { Instruction::RETI }
    fn ldi(self) -> Self::R { Instruction::LDI }

    fn pop<T: Write16>(self, target: T) -> Self::R{
        Instruction::POP(target.into_arg16(self)) 
    }
    fn push<S: Read16>(self, source: S) -> Self::R {
        Instruction::PUSH(source.into_arg16(self))
    }
    

    fn rst(self, byte: u8) -> Self::R { Instruction::RST(byte) }

    fn srl<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R {
        Instruction::SRL(source.into_arg8(self))
    }
    fn sll<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R {
        Instruction::SLL(source.into_arg8(self))
    }
    fn sra<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R {
        Instruction::SRA(source.into_arg8(self))
    }
    fn sla<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R {
        Instruction::SLA(source.into_arg8(self))
    }
    fn rlc<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R  {
        Instruction::RLC(source.into_arg8(self))
    }
    fn rrc<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R {
        Instruction::RRC(source.into_arg8(self))
    }
    fn bit<S: Read8>(self, bit: u8, source: S) -> Self::R {
        Instruction::BIT(bit, source.into_arg8(self))
    }
    fn rl<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R {
        Instruction::RL(source.into_arg8(self))
    }
    fn res<S: Read8 + Write8 + Copy>(self, bit: u8, source: S) -> Self::R {
        Instruction::RES(bit, source.into_arg8(self))
    }
    fn rr<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R {
        Instruction::RR(source.into_arg8(self))
    }
    fn set<S: Read8 + Write8 + Copy>(self, bit: u8, source: S) -> Self::R {
        Instruction::SET(bit, source.into_arg8(self))
    }

    fn cb_op(self) -> Self::R { decode_cb(self, self.next_byte())}
    
    fn dd_op(self) -> Self::R{     decode_dd(self, self.next_byte()) }
    fn ed_op(self) -> Self::R{         decode_ed(self, self.next_byte())  }
    fn fd_op(self) -> Self::R{         decode_fd(self, self.next_byte()) }
    fn dd_fd_cb_op(self, _: Reg16) -> Self::R {

        let _ = self.next_byte();
        let op = self.next_byte();

        decode_dd_fd_cb(self, 0, op)
    }
}
