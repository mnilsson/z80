mod eight_bit_arithmetic;
mod logic;
mod rot_shf;
mod mem_ops;

pub use self::eight_bit_arithmetic::*;
pub use self::logic::*;
pub use self::rot_shf::*;
pub use self::mem_ops::*;

use crate::flags::Flag::*;
use crate::cpu::{Not, Read8, Read16, Write8, Write16, ReadCond};


use crate::registers::Reg8::*;
use crate::registers::Reg16::*;
use crate::registers::Reg16;
use crate::registers::ReadAddress;
use crate::cpu::{ImmByte, ImmWord, Mem, RelOffset};


pub trait Ops {
    type R;
    fn and<R: Read8>(self, reg: R) -> Self::R;
    fn add8<D: Write8 + Read8 + Copy, S: Read8>(self, dest: D, source: S) -> Self::R;
    fn adc8<D: Write8 + Read8 + Copy, S: Read8>(self, dest: D, source: S) -> Self::R;
    fn add16<D: Write16 + Read16 + Copy, S: Read16>(self, dest: D, source: S) -> Self::R;

    fn call<A: Read16>(self, addr: A) -> Self::R; 
    fn call_cond<C: ReadCond, A: Read16>(self, condition: C, addr: A) -> Self::R; 
    fn ccf(self) -> Self::R;
    fn cp<S: Read8>(self, source: S) -> Self::R;
    fn cpl(self) -> Self::R;
    fn daa(self) -> Self::R;
    fn dec8<R: Write8 + Read8 + Copy>(self, reg: R) -> Self::R;
    fn dec8_memory<R: ReadAddress>(self, reg: R) -> Self::R;
    fn dec16<R: Write16 + Read16 + Copy>(self, reg: R) -> Self::R;
    fn di(self) -> Self::R;
    fn ei(self) -> Self::R;
    fn ex<D: Write16 + Read16 + Copy, S: Write16 + Read16 + Copy>(
        self,
        dest: D,
        source: S,
    ) -> Self::R;
    fn exx(self) -> Self::R;
    fn halt(self) -> Self::R;

    fn in8<D: Write8, S: Read8>(self, dest: D, source: S) -> Self::R;
    fn in8_noflags<D: Write8, S: Read8>(self, dest: D, source: S) -> Self::R;

    fn inc8<R: Write8 + Read8 + Copy>(self, reg: R) -> Self::R;
    fn inc8_memory<R: ReadAddress>(self, reg: R) -> Self::R;
    
    fn inc16<R: Write16 + Read16 + Copy>(self, reg: R) -> Self::R;

    fn jp<A: Read16>(self, addr: A) -> Self::R;

    fn jp_cond<C: ReadCond, A: Read16>(self, condition: C, addr: A) -> Self::R;

    fn jr<C: ReadCond>(self, condition: C) -> Self::R;
    fn djnz(self) -> Self::R;
    fn ret(self) -> Self::R;
    fn ret_cond<C: ReadCond>(self, condition: C) -> Self::R;
    fn ld8<D: Write8, S: Read8>(self, dest: D, source: S) -> Self::R;
    fn ld8_int<D: Write8, S: Read8>(self, dest: D, source: S) -> Self::R;
    fn ld8_address_dest<D: ReadAddress, S: Read8>(self, dest: D, source: S) -> Self::R;
    fn ld8_address_source<D: Write8, S: ReadAddress>(self, dest: D, source: S) -> Self::R;
    fn ld16<D: Write16, S: Read16>(self, dest: D, source: S) -> Self::R;

    fn nop(self) -> Self::R;
    fn out8<D: Read8, S: Read8>(self, dest: D, source: S) -> Self::R;
    fn out8_noflags<D: Read8, S: Read8>(self, dest: D, source: S) -> Self::R;

    fn or<R: Read8>(self, reg: R) -> Self::R;

    fn rla(self) -> Self::R;
    fn rlca(self) -> Self::R;
    fn rra(self) -> Self::R;
    fn rrca(self) -> Self::R;
    fn scf(self) -> Self::R;

    fn xor<R: Read8>(self, reg: R) -> Self::R;

    fn sub8<S: Read8>(self, source: S) -> Self::R;
    fn sbc8<S: Read8>(self, source: S) -> Self::R;

    fn sbc16<D: Write16 + Read16 + Copy, S: Read16>(
        self,
        dest: D,
        source: S,
    ) -> Self::R;
    fn adc16<D: Write16 + Read16 + Copy, S: Read16>(
        self,
        dest: D,
        source: S,
    ) -> Self::R;
    fn neg(self) -> Self::R;
    fn retn(self) -> Self::R;
    fn cpd(self) -> Self::R;
    fn cpi(self) -> Self::R;
    fn ind(self) -> Self::R;
    fn outd(self) -> Self::R;
    fn ldir(self) -> Self::R;
    fn cpir(self) -> Self::R;
    fn inir(self) -> Self::R;
    fn lddr(self) -> Self::R;
    fn cpdr(self) -> Self::R;
    fn indr(self) -> Self::R;
    fn otdr(self) -> Self::R;
    fn outi(self) -> Self::R;
    fn otir(self) -> Self::R;
    fn ldd(self) -> Self::R;
    fn ini(self) -> Self::R;
    fn im(self, im: u8) -> Self::R;
    fn rrd(self) -> Self::R;
    fn rld(self) -> Self::R;
    fn reti(self) -> Self::R;
    fn ldi(self) -> Self::R;

    fn pop<T: Write16>(self, target: T) -> Self::R;
    fn push<S: Read16>(self, source: S) -> Self::R;
    

    fn rst(self, byte: u8) -> Self::R;

    fn srl<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R;
    fn sll<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R;
    fn sra<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R;
    fn sla<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R;
    fn rlc<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R;
    fn rrc<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R;
    fn bit<S: Read8>(self, bit: u8, source: S) -> Self::R;
    fn rl<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R;
    fn res<S: Read8 + Write8 + Copy>(self, bit: u8, source: S) -> Self::R;
    fn rr<S: Read8 + Write8 + Copy>(self, source: S) -> Self::R;
    fn set<S: Read8 + Write8 + Copy>(self, bit: u8, source: S) -> Self::R;

    fn cb_op(self) -> Self::R;
    
    fn dd_op(self) -> Self::R;
    fn ed_op(self) -> Self::R;
    fn fd_op(self) -> Self::R;
    fn dd_fd_cb_op(self, ireg: Reg16) -> Self::R;
}


pub fn decode<O: Ops>(ops: O, op: u8) -> O::R {
    
    match op {
        0x00 => ops.nop(),
        0x01 => ops.ld16(BC, ImmWord),
        0x02 => ops.ld8(Mem(BC), A),
        0x03 => ops.inc16(BC),
        0x04 => ops.inc8(B),
        0x05 => ops.dec8(B),
        0x06 => ops.ld8(B, ImmByte),
        0x07 => ops.rlca(),
        0x08 => ops.ex(AF, _AF),
        0x09 => ops.add16(HL, BC),
        0x0a => ops.ld8(A, Mem(BC)),
        0x0b => ops.dec16(BC),
        0x0c => ops.inc8(C),
        0x0d => ops.dec8(C),
        0x0e => ops.ld8(C, ImmByte),
        0x0f => ops.rrca(),

        0x10 => ops.djnz(),
        0x11 => ops.ld16(DE, ImmWord),
        0x12 => ops.ld8(Mem(DE), A),
        0x13 => ops.inc16(DE),
        0x14 => ops.inc8(D),
        0x15 => ops.dec8(D),
        0x16 => ops.ld8(D, ImmByte),
        0x17 => ops.rla(),
        0x18 => ops.jr(true),
        0x19 => ops.add16(HL, DE),
        0x1a => ops.ld8(A, Mem(DE)),
        0x1b => ops.dec16(DE),
        0x1c => ops.inc8(E),
        0x1d => ops.dec8(E),
        0x1e => ops.ld8(E, ImmByte),
        0x1f => ops.rra(),

        0x20 => ops.jr(Not(Zero)),
        0x21 => ops.ld16(HL, ImmWord),
        0x22 => ops.ld16(Mem(ImmWord), HL),
        0x23 => ops.inc16(HL),
        0x24 => ops.inc8(H),
        0x25 => ops.dec8(H),
        0x26 => ops.ld8(H, ImmByte),
        0x27 => ops.daa(),
        0x28 => ops.jr(Zero),
        0x29 => ops.add16(HL, HL),
        0x2a => ops.ld16(HL, Mem(ImmWord)),
        0x2b => ops.dec16(HL),
        0x2c => ops.inc8(L),
        0x2d => ops.dec8(L),
        0x2e => ops.ld8(L, ImmByte),
        0x2f => ops.cpl(),

        0x30 => ops.jr(Not(Carry)),
        0x31 => ops.ld16(SP, ImmWord),
        0x32 => ops.ld8(Mem(ImmWord), A),
        0x33 => ops.inc16(SP),
        0x34 => ops.inc8(Mem(HL)),
        0x35 => ops.dec8(Mem(HL)),
        0x36 => ops.ld8(Mem(HL), ImmByte),
        0x37 => ops.scf(),
        0x38 => ops.jr(Carry),
        0x39 => ops.add16(HL, SP),
        0x3a => ops.ld8(A, Mem(ImmWord)),
        0x3b => ops.dec16(SP),
        0x3c => ops.inc8(A),
        0x3d => ops.dec8(A),
        0x3e => ops.ld8(A, ImmByte),
        0x3f => ops.ccf(),

        0x40 => ops.ld8(B, B),
        0x41 => ops.ld8(B, C),
        //0x41
        0x42 => ops.ld8(B, D),
        0x43 => ops.ld8(B, E),
        //0x43
        0x44 => ops.ld8(B, H),
        0x45 => ops.ld8(B, L),
        //0x45
        0x46 => ops.ld8(B, Mem(HL)),
        0x47 => ops.ld8(B, A),
        //0x47
        0x48 => ops.ld8(C, B),
        0x49 => ops.ld8(C, C),
        //0x49
        0x4a => ops.ld8(C, D),
        0x4b => ops.ld8(C, E),

        0x4c => ops.ld8(C, H),
        0x4d => ops.ld8(C, L),
        //0x4d
        0x4e => ops.ld8(C, Mem(HL)),
        0x4f => ops.ld8(C, A),
        //0x4f
        0x50 => ops.ld8(D, B),
        0x51 => ops.ld8(D, C),
        0x52 => ops.ld8(D, D),
        0x53 => ops.ld8(D, E),
        0x54 => ops.ld8(D, H),
        0x55 => ops.ld8(D, L),
        0x56 => ops.ld8(D, Mem(HL)),
        0x57 => ops.ld8(D, A),

        0x58 => ops.ld8(E, B),
        0x59 => ops.ld8(E, C),
        0x5a => ops.ld8(E, D),
        0x5b => ops.ld8(E, E),

        0x5c => ops.ld8(E, H),
        0x5d => ops.ld8(E, L),
        0x5e => ops.ld8(E, Mem(HL)),
        0x5f => ops.ld8(E, A),

        0x60 => ops.ld8(H, B),
        0x61 => ops.ld8(H, C),
        0x62 => ops.ld8(H, D),
        0x63 => ops.ld8(H, E),
        0x64 => ops.ld8(H, H),
        0x65 => ops.ld8(H, L),
        0x66 => ops.ld8(H, Mem(HL)),
        0x67 => ops.ld8(H, A),

        0x68 => ops.ld8(L, B),
        0x69 => ops.ld8(L, C),
        0x6a => ops.ld8(L, D),
        0x6b => ops.ld8(L, E),
        0x6c => ops.ld8(L, H),
        0x6d => ops.ld8(L, L),
        0x6e => ops.ld8(L, Mem(HL)),
        0x6f => ops.ld8(L, A),

        0x70 => ops.ld8(Mem(HL), B),
        0x71 => ops.ld8(Mem(HL), C),
        0x72 => ops.ld8(Mem(HL), D),
        0x73 => ops.ld8(Mem(HL), E),
        0x74 => ops.ld8(Mem(HL), H),
        0x75 => ops.ld8(Mem(HL), L),
        0x76 => ops.halt(),
        0x77 => ops.ld8(Mem(HL), A),

        0x78 => ops.ld8(A, B),
        0x79 => ops.ld8(A, C),
        0x7a => ops.ld8(A, D),
        0x7b => ops.ld8(A, E),
        0x7c => ops.ld8(A, H),
        0x7d => ops.ld8(A, L),
        0x7e => ops.ld8(A, Mem(HL)),
        0x7f => ops.ld8(A, A),

        0x80 => ops.add8(A, B),
        0x81 => ops.add8(A, C),
        0x82 => ops.add8(A, D),
        0x83 => ops.add8(A, E),
        0x84 => ops.add8(A, H),
        0x85 => ops.add8(A, L),
        0x86 => ops.add8(A, Mem(HL)),
        0x87 => ops.add8(A, A),

        0x88 => ops.adc8(A, B),
        0x89 => ops.adc8(A, C),
        0x8a => ops.adc8(A, D),
        0x8b => ops.adc8(A, E),
        0x8c => ops.adc8(A, H),
        0x8d => ops.adc8(A, L),
        0x8e => ops.adc8(A, Mem(HL)),
        0x8f => ops.adc8(A, A),

        0x90 => ops.sub8(B),
        0x91 => ops.sub8(C),
        0x92 => ops.sub8(D),
        0x93 => ops.sub8(E),
        0x94 => ops.sub8(H),
        0x95 => ops.sub8(L),
        0x96 => ops.sub8(Mem(HL)),
        0x97 => ops.sub8(A),

        0x98 => ops.sbc8(B),
        0x99 => ops.sbc8(C),
        0x9a => ops.sbc8(D),
        0x9b => ops.sbc8(E),
        0x9c => ops.sbc8(H),
        0x9d => ops.sbc8(L),
        0x9e => ops.sbc8(Mem(HL)),
        0x9f => ops.sbc8(A),

        0xa0 => ops.and(B),
        0xa1 => ops.and(C),
        0xa2 => ops.and(D),
        0xa3 => ops.and(E),
        0xa4 => ops.and(H),
        0xa5 => ops.and(L),
        0xa6 => ops.and(Mem(HL)),
        0xa7 => ops.and(A),
        0xa8 => ops.xor(B),
        0xa9 => ops.xor(C),
        0xaa => ops.xor(D),
        0xab => ops.xor(E),
        0xac => ops.xor(H),
        0xad => ops.xor(L),
        0xae => ops.xor(Mem(HL)),
        0xaf => ops.xor(A),

        0xb0 => ops.or(B),
        0xb1 => ops.or(C),
        0xb2 => ops.or(D),
        0xb3 => ops.or(E),
        0xb4 => ops.or(H),
        0xb5 => ops.or(L),
        0xb6 => ops.or(Mem(HL)),
        0xb7 => ops.or(A),
        0xb8 => ops.cp(B),
        0xb9 => ops.cp(C),
        0xba => ops.cp(D),
        0xbb => ops.cp(E),
        0xbc => ops.cp(H),
        0xbd => ops.cp(L),
        0xbe => ops.cp(Mem(HL)),
        0xbf => ops.cp(A),

        0xc0 => ops.ret_cond(Not(Zero)),
        0xc1 => ops.pop(BC),
        0xc2 => ops.jp_cond(Not(Zero), ImmWord),
        0xc3 => ops.jp(ImmWord),
        0xc4 => ops.call_cond(Not(Zero), ImmWord),
        0xc5 => ops.push(BC),
        0xc6 => ops.add8(A, ImmByte),
        0xc7 => ops.rst(0x00),
        0xc8 => ops.ret_cond(Zero),
        0xc9 => ops.ret(),
        0xca => ops.jp_cond(Zero, ImmWord),
        0xcb => ops.cb_op(),
        0xcc => ops.call_cond(Zero, ImmWord),
        0xcd => ops.call(ImmWord),
        0xce => ops.adc8(A, ImmByte),
        0xcf => ops.rst(0x08),

        0xd0 => ops.ret_cond(Not(Carry)),
        0xd1 => ops.pop(DE), 
        0xd2 => ops.jp_cond(Not(Carry), ImmWord),
        0xd3 => ops.out8_noflags(ImmByte, A),
        0xd4 => ops.call_cond(Not(Carry), ImmWord),
        0xd5 => ops.push(DE),
        0xd6 => ops.sub8(ImmByte),
        0xd7 => ops.rst(0x10),
        0xd8 => ops.ret_cond(Carry),
        0xd9 => ops.exx(),
        0xda => ops.jp_cond(Carry, ImmWord),
        0xdb => ops.in8_noflags(A, ImmByte),
        0xdc => ops.call_cond(Carry, ImmWord),
        0xdd => ops.dd_op(),
        0xde => ops.sbc8(ImmByte),
        0xdf => ops.rst(0x18),

        0xe0 => ops.ret_cond(Not(Parity)),
        0xe1 => ops.pop(HL),
        0xe2 => ops.jp_cond(Not(Parity), ImmWord),
        0xe3 => ops.ex(Mem(SP), HL),
        0xe4 => ops.call_cond(Not(Parity), ImmWord),
        0xe5 => ops.push(HL),
        0xe6 => ops.and(ImmByte),
        0xe7 => ops.rst(0x20),
        0xe8 => ops.ret_cond(Parity),
        0xe9 => ops.jp(HL),
        0xea => ops.jp_cond(Parity, ImmWord),
        0xeb => ops.ex(DE, HL),
        0xec => ops.call_cond(Parity, ImmWord),
        0xed => ops.ed_op(),
        0xee => ops.xor(ImmByte),
        0xef => ops.rst(0x28),

        0xf0 => ops.ret_cond(Not(Sign)),
        0xf1 => ops.pop(AF),
        0xf2 => ops.jp_cond(Not(Sign), ImmWord),
        0xf3 => ops.di(),
        0xf4 => ops.call_cond(Not(Sign), ImmWord),
        0xf5 => ops.push(AF),
        0xf6 => ops.or(ImmByte),
        0xf7 => ops.rst(0x30),
        0xf8 => ops.ret_cond(Sign),
        0xf9 => ops.ld16(SP, HL),
        0xfa => ops.jp_cond(Sign, ImmWord),
        0xfb => ops.ei(),
        0xfc => ops.call_cond(Sign, ImmWord),
        0xfd => ops.fd_op(),
        0xfe => ops.cp(ImmByte),
        0xff => ops.rst(0x38),
        _ => unreachable!("Unreachable {:02X}", op),
    }
}


pub fn decode_dd<O: Ops>(ops: O, op: u8) -> O::R {

    decode_fd_dd(ops, IX, op)
}

pub fn decode_fd<O: Ops>(ops: O, op: u8) -> O::R {

    decode_fd_dd(ops, IY, op)
}

fn decode_fd_dd<O: Ops>(ops: O, ireg: Reg16, op: u8) -> O::R {
    
    let (iregh, iregl) = match ireg {
        IY => (IYH, IYL),
        IX => (IXH, IXL),
        _ => unreachable!("Only IY and IX is valid")
    };
    match op {
        0x09 => ops.add16(ireg, BC),
        0x19 => ops.add16(ireg, DE),
        
        0x21 => ops.ld16(ireg, ImmWord),
        0x22 => ops.ld16(Mem(ImmWord), ireg),
        0x23 => ops.inc16(ireg),
        0x24 => ops.inc8(iregh), // Undocumented
        0x25 => ops.dec8(iregh), // Undocumented
        0x26 => ops.ld8(iregh, ImmByte), // Undocumented

        0x29 => ops.add16(ireg, ireg),
        0x2a => ops.ld16(ireg, Mem(ImmWord)),
        0x2b => ops.dec16(ireg),
        0x2c => ops.inc8(iregl),
        0x2d => ops.dec8(iregl),
        0x2e => ops.ld8(iregl, ImmByte),

        0x34 => ops.inc8_memory(Mem(RelOffset(ireg))),
        0x35 => ops.dec8_memory(Mem(RelOffset(ireg))),
        0x36 => ops.ld8_address_dest(Mem(RelOffset(ireg)), ImmByte),

        0x39 => ops.add16(ireg, SP),

        0x44 => ops.ld8(B, iregh),
        0x45 => ops.ld8(B, iregl),
        0x46 => ops.ld8_address_source(B, Mem(RelOffset(ireg))),

        0x4c => ops.ld8(C, iregh),
        0x4d => ops.ld8(C, iregl),
        0x4e => ops.ld8_address_source(C, Mem(RelOffset(ireg))),

        0x54 => ops.ld8(D, iregh), 
        0x55 => ops.ld8(D, iregl),
        0x56 => ops.ld8_address_source(D, Mem(RelOffset(ireg))),

        0x5c => ops.ld8(E, iregh),
        0x5d => ops.ld8(E, iregl),
        0x5e => ops.ld8_address_source(E, Mem(RelOffset(ireg))),

        0x60 => ops.ld8(iregh, B),
        0x61 => ops.ld8(iregh, C),
        0x62 => ops.ld8(iregh, D),
        0x63 => ops.ld8(iregh, E),
        0x64 => ops.ld8(iregh, iregh),
        0x65 => ops.ld8(iregh, iregl),
        0x66 => ops.ld8_address_source(H, Mem(RelOffset(ireg))),
        0x67 => ops.ld8(iregh, A),
        0x68 => ops.ld8(iregl, B),
        0x69 => ops.ld8(iregl, C),
        0x6a => ops.ld8(iregl, D),
        0x6b => ops.ld8(iregl, E),
        0x6c => ops.ld8(iregl, iregh),
        0x6d => ops.ld8(iregl, iregl),
        0x6e => ops.ld8_address_source(L, Mem(RelOffset(ireg))),
        0x6f => ops.ld8(iregl, A),
        

        

        0x70 => ops.ld8_address_dest(Mem(RelOffset(ireg)), B),
        0x71 => ops.ld8_address_dest(Mem(RelOffset(ireg)), C),
        0x72 => ops.ld8_address_dest(Mem(RelOffset(ireg)), D),
        0x73 => ops.ld8_address_dest(Mem(RelOffset(ireg)), E),
        0x74 => ops.ld8_address_dest(Mem(RelOffset(ireg)), H),
        0x75 => ops.ld8_address_dest(Mem(RelOffset(ireg)), L),
        
        0x77 => ops.ld8_address_dest(Mem(RelOffset(ireg)), A),

        0x7c => ops.ld8(A, iregh),
        0x7d => ops.ld8(A, iregl),
        0x7e => ops.ld8_address_source(A, Mem(RelOffset(ireg))),

        0x84 => ops.add8(A, iregh),
        0x85 => ops.add8(A, iregl),
        0x86 => ops.add8(A, Mem(RelOffset(ireg))),

        0x8c => ops.adc8(A, iregh),
        0x8d => ops.adc8(A, iregl),
        0x8e => ops.adc8(A, Mem(RelOffset(ireg))),

        0x94 => ops.sub8(iregh),
        0x95 => ops.sub8(iregl),
        0x96 => ops.sub8(Mem(RelOffset(ireg))),

        0x9c => ops.sbc8(iregh),
        0x9d => ops.sbc8(iregl),
        0x9e => ops.sbc8(Mem(RelOffset(ireg))),

        0xa4 => ops.and(iregh),
        0xa5 => ops.and(iregl),
        0xa6 => ops.and(Mem(RelOffset(ireg))),

        0xac => ops.xor(iregh),
        0xad => ops.xor(iregl),
        0xae => ops.xor(Mem(RelOffset(ireg))),

        0xb4 => ops.or(iregh),
        0xb5 => ops.or(iregl),
        0xb6 => ops.or(Mem(RelOffset(ireg))),

        0xbc => ops.cp(iregh),
        0xbd => ops.cp(iregl),
        0xbe => ops.cp(Mem(RelOffset(ireg))),

        0xcb => ops.dd_fd_cb_op(ireg),

        0xe1 => ops.pop(ireg),
        0xe3 => ops.ex(Mem(SP), ireg),
        0xe5 => ops.push(ireg),
        0xe9 => ops.jp(ireg),
        0xf9 => ops.ld16(SP, ireg),
        
        _ => decode(ops, op), //unreachable!("nope {:X}", op)
    }
}

pub fn decode_dd_fd_cb<O: Ops>(ops: O, address: u16, op: u8) -> O::R {
    match op & 0b1100_0111 {
        0x06 => {
            match op {
                0x06 => ops.rlc(Mem(address)),
                0x0e => ops.rrc(Mem(address)),
                0x16 => ops.rl(Mem(address)),
                0x1e => ops.rr(Mem(address)),
                0x26 => ops.sla(Mem(address)),
                0x2e => ops.sra(Mem(address)),
                0x36 => ops.sll(Mem(address)),
                0x3e => ops.srl(Mem(address)),
                _ => unreachable!()
            }
        },

        0x46 => ops.bit((op & 0b0011_1000) >> 3, Mem(address)),
                        
        0x86 => ops.res((op & 0b0011_1000) >> 3, Mem(address)),
        0xc6 => ops.set((op & 0b0011_1000) >> 3, Mem(address)),
        _ => ops.nop(), //unreachable!()
    }
}

pub fn decode_ed<O: Ops>(ops: O, op: u8) -> O::R {
    if op != 0x4d && op != 0xb0 {
        // println!(" {:02x}", op);
    }
    match op {
        0x40 => ops.in8(B, C),
        0x41 => ops.out8(C, B),
        0x42 => ops.sbc16(HL, BC),
        0x43 => ops.ld16(Mem(ImmWord), BC),
        0x44 => ops.neg(),
        0x45 => ops.retn(),
        0x46 => ops.im(0),
        0x47 => ops.ld8(I, A),
        0x48 => ops.in8(C, C),
        0x49 => ops.out8(C, C),
        0x4a => ops.adc16(HL, BC),
        0x4b => ops.ld16(BC, Mem(ImmWord)),
        0x4c => ops.neg(), // @todo undoc
        0x4d => ops.reti(),
        // 0x4e
        0x4f => ops.ld8(R, A),

        0x50 => ops.in8(D, C),
        0x51 => ops.out8(C, D),
        0x52 => ops.sbc16(HL, DE),
        0x53 => ops.ld16(Mem(ImmWord), DE),
        0x54 => ops.neg(), // @todo undoc
        0x55 => ops.retn(),
        0x56 => ops.im(1),
        0x57 => ops.ld8_int(A, I),
        0x58 => ops.in8(E, C),
        0x59 => ops.out8(C, E),
        0x5a => ops.adc16(HL, DE),
        0x5b => ops.ld16(DE, Mem(ImmWord)),
        0x5c => ops.neg(), // @todo undoc
        0x5d => ops.retn(),
        0x5e => ops.im(2),
        0x5f => ops.ld8_int(A, R),

        0x60 => ops.in8(H, C),
        0x61 => ops.out8(C, H),
        0x62 => ops.sbc16(HL, HL),
        0x63 => ops.ld16(Mem(ImmWord), HL), // @todo undoc
        0x64 => ops.neg(),                // @todo undoc
        0x65 => ops.retn(),
        0x66 => ops.im(0),
        0x67 => ops.rrd(),
        0x68 => ops.in8(L, C),
        0x69 => ops.out8(C, L),
        0x6a => ops.adc16(HL, HL),
        0x6b => ops.ld16(HL, Mem(ImmWord)), // @todo undoc
        0x6c => ops.neg(),                // @todo undoc
        0x6d => ops.retn(),
        //        0x6e ,
        0x6f => ops.rld(),

        //        0x70
        //        0x71
        0x71 => ops.out8(C, 0),
        0x72 => ops.sbc16(HL, SP),
        0x73 => ops.ld16(Mem(ImmWord), SP),
        //        0x74
        0x75 => ops.retn(),
        0x76 => ops.im(1),

        0x78 => ops.in8(A, C),
        0x79 => ops.out8(C, A),
        0x7a => ops.adc16(HL, SP),
        0x7b => ops.ld16(SP, Mem(ImmWord)),
        0x7c => ops.neg(), //@todo undoc
        0x7d => ops.retn(),
        0x7e => ops.im(2),

        0xa0 => ops.ldi(),
        0xa1 => ops.cpi(),
        0xa2 => ops.ini(),
        0xa3 => ops.outi(),

        0xa8 => ops.ldd(),
        0xa9 => ops.cpd(),
        0xaa => ops.ind(),
        0xab => ops.outd(),

        0xb0 => ops.ldir(),
        0xb1 => ops.cpir(),
        0xb2 => ops.inir(),
        0xb3 => ops.otir(),

        0xb8 => ops.lddr(),
        0xb9 => ops.cpdr(),
        0xba => ops.indr(),
        0xbb => ops.otdr(),
        _ => unreachable!("no! {:02x}", op),
    }
}

pub fn decode_cb<O: Ops>(ops: O, op: u8) -> O::R {
    let reg = match op & 0b111 {
        0 => Some(B),
        1 => Some(C),
        2 => Some(D),
        3 => Some(E),
        4 => Some(H),
        5 => Some(L),
        7 => Some(A),
        _ => None,
    };

    match op >> 3 {
        0b000 => match reg { Some(r) => ops.rlc(r), _ => ops.rlc(Mem(HL))},
        0b001 => match reg { Some(r) => ops.rrc(r), _ => ops.rrc(Mem(HL))},
        0b010 => match reg { Some(r) => ops.rl(r), _ => ops.rl(Mem(HL))},
        0b011 => match reg { Some(r) => ops.rr(r), _ => ops.rr(Mem(HL))},
        0b100 => match reg { Some(r) => ops.sla(r), _ =>  ops.sla(Mem(HL))},
        0b101 => match reg { Some(r) => ops.sra(r), _ => ops.sra(Mem(HL))},
        0b110 => match reg { Some(r) => ops.sll(r), _ => ops.sll(Mem(HL))},
        0b111 => match reg { Some(r) => ops.srl(r), _ => ops.srl(Mem(HL))},

        0b1000...0b1111 => {
            let bit = (op >> 3) & 0b111;
              match reg { Some(r) => ops.bit(bit,r), _ => ops.bit(bit,Mem(HL))}
        }

        // reset bit
        0b1_0000...0b1_0111 => {
            let bit = (op >> 3) & 0b111;
            match reg { Some(r) => ops.res(bit,r), _ => ops.res(bit,Mem(HL))}
        }

        // set bit
        0b1_1000...0b1_1111 => {
            let bit = (op >> 3) & 0b111;
            match reg {
                Some(r) => ops.set(bit, r),
                _ => ops.set(bit, Mem(HL))
            }
        }
        _ => {
            unreachable!("CB{:X}", op);
        }
    }
}
