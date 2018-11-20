use crate::cpu::{Z80, Read8, Write8};
use crate::flags::Flag::*;
use crate::bus::Bus;


pub fn rlc<R: Read8 + Write8 + Copy, B: Bus>(z80: &mut Z80, bus: &mut B, reg: R) {
    let val = reg.read8(z80, bus);

    let res = val.rotate_left(1);


    z80.registers.set_flag(Carry, val & 0x80 == 0x80);

    z80.common_rot_flags();
    z80.szp_flags(res);
    z80.registers.set_xy(res);
    reg.write8(z80, bus,res);
}

pub fn rl<R: Read8 + Write8 + Copy, B: Bus>(z80: &mut Z80, bus: &mut B, reg: R) {
    let val = reg.read8(z80, bus);
    let mut res = val << 1;
    if z80.registers.get_flag(Carry) {
        res |= 1;
    }

    z80.registers.set_flag(Carry, val & 0x80 == 0x80);

    z80.szp_flags( res);
    z80.common_rot_flags();
    z80.registers.set_xy(res);

    reg.write8(z80, bus,res);
}


pub fn rr<R: Write8 + Read8 + Copy, B: Bus>(z80: &mut Z80, bus: &mut B, r: R) {
    let c = if z80.registers.get_flag(Carry) { 1 } else { 0 };

    let val = r.read8(z80, bus);

    let co = val & 0x01;

    let res = (val >> 1) | (c << 7);
    z80.registers.set_flag(HalfCarry, false);
    z80.registers.set_flag(Subtract,false);
    z80.registers.set_flag(Carry,co == 0x1);
    z80.szp_flags( res);
    z80.registers.set_xy(res);
    r.write8(z80, bus,res);
}

pub fn rrc<R: Read8 + Write8 + Copy, B: Bus>(z80: &mut Z80, bus: &mut B, reg: R) {
    let val = reg.read8(z80, bus);
    let res = val.rotate_right(1);

    z80.registers.set_flag(Carry, val & 0x1 == 1);

    z80.common_rot_flags();
    z80.szp_flags( res);
    z80.registers.set_xy(res);
    reg.write8(z80, bus,res);
}

pub fn sla<R: Read8 + Write8 + Copy, B: Bus>(z80: &mut Z80, bus: &mut B, reg: R) {
    let val = reg.read8(z80, bus);
    let r = val << 1;

    z80.registers.set_flag(HalfCarry, false);
    z80.registers.set_flag(Subtract, false);
    z80.registers.set_flag(Carry, val & 0x80 == 0x80);
    z80.szp_flags(r);
    z80.registers.set_xy(r);
    reg.write8(z80, bus, r);
}

pub fn sra<R: Read8 + Write8 + Copy, B: Bus>(z80: &mut Z80, bus: &mut B, reg: R) {
    let val = reg.read8(z80, bus);
    let r = ((val as i8) >> 1) as u8 ;//| (val & 0x80);

    //        let r = if z80.flags.c { r | 0x80 } else { r };


    z80.registers.set_flag(HalfCarry,false);
    z80.registers.set_flag(Subtract, false);
    z80.registers.set_flag(Carry, val & 0x1 == 1);
    z80.szp_flags(r);
    z80.registers.set_xy(r);
    reg.write8(z80, bus,r);
}

pub fn srl<R: Read8 + Write8 + Copy, B: Bus>(z80: &mut Z80, bus: &mut B, reg: R) {
    let val = reg.read8(z80, bus);
    let r = val >> 1;

    z80.registers.set_flag(HalfCarry,false);
    z80.registers.set_flag(Subtract, false);
    z80.registers.set_flag(Carry,val & 0x01 != 0);
    z80.szp_flags( r);
    z80.registers.set_xy(r);
    reg.write8(z80, bus,r);
}

pub fn sll<R: Read8 + Write8 + Copy, B: Bus>(z80: &mut Z80, bus: &mut B, reg: R) {
    let val = reg.read8(z80, bus);
    let r = (val << 1) | 1;

    z80.registers.set_flag(HalfCarry, false);
    z80.registers.set_flag(Subtract, false);
    z80.registers.set_flag(Carry, val & 0x80 != 0);
    z80.szp_flags( r);
    z80.registers.set_xy(r);
    reg.write8(z80, bus,r);
}