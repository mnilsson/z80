use crate::cpu::{Z80, Read8, Write8};

use crate::flags::Flag::*;
use crate::bus::Bus;
use crate::registers::{Reg8};

pub fn inc_u8<R: Read8 + Write8 + Copy, B: Bus>(z80: &mut Z80, bus: &mut B, reg: R) {
    let val = reg.read8(z80, bus);
    let res = val.wrapping_add(1);

    z80.registers.set_flag(Sign, res & 0x80 == 0x80);
    z80.registers.set_flag(Zero, res == 0);
    z80.registers.set_flag(HalfCarry, (res & 0x0f) == 0x0);
    z80.registers.set_flag(Parity, (val & 0x80 == 0) && (res & 0x80 == 0x80));
    z80.registers.set_flag(Subtract, false);

    z80.registers.set_xy(res);
    reg.write8(z80, bus, res);
}

pub fn dec_u8<R: Write8 + Read8 + Copy, B: Bus>(z80: &mut Z80, bus: &mut B, reg: R) {
    let val = reg.read8(z80, bus);
    let res = val.wrapping_sub(1);

    z80.registers.set_flag(Sign, res & 0x80 == 0x80);
    z80.registers.set_flag(Zero, res == 0);
    z80.registers.set_flag(HalfCarry, res & 0x0f == 0x0f);
    z80.registers.set_flag(Parity, res == 0x7f);
    z80.registers.set_flag(Parity, (val & 0x80 == 0x80) && (res & 0x80 == 0));
    z80.registers.set_flag(Subtract, true);

    z80.registers.set_xy(res);

    reg.write8(z80, bus, res);
}

pub fn add<D: Write8 + Read8 + Copy, S: Read8, B: Bus>(z80: &mut Z80, bus: &mut B, dest: D, source: S) {
    let val = source.read8(z80, bus);
    let destval = dest.read8(z80, bus);
    let res = raw_add(z80, destval, val);

    dest.write8(z80, bus, res as u8);
}

pub fn raw_add(z80: &mut Z80, dest: u8, source:u8) -> u8 {
    let res = dest as u16 + source as u16;

    flags_add(z80, dest, source, res);
    res as u8
}

pub fn raw_addc(z80: &mut Z80, dest: u8, val: u8, carry: u8) -> u8 {
    let res = dest as u16 + val as u16 + carry as u16;


    flags_add(z80, dest, val, res as u16);
    res as u8
}


pub fn adc<D: Write8 + Read8 + Copy, S: Read8, B: Bus>(z80: &mut Z80, bus: &mut B, dest: D, source: S) {
    let val = source.read8(z80, bus);
    let destval = dest.read8(z80, bus);

    let carry = if z80.registers.get_flag(Carry) {1} else {0};
    let res = raw_addc(z80, destval, val, carry);


    dest.write8(z80, bus, res as u8);
}

pub fn sub<S: Read8, B: Bus>(z80: &mut Z80, bus: &mut B, source: S) {
    let a = Reg8::A.read8(z80, bus);
    let val = source.read8(z80, bus);
    let res = raw_sub(z80, a, val, 0);
    Reg8::A.write8(z80, bus, res as u8);
}

pub fn raw_sub(z80: &mut Z80, dest: u8, val: u8, carry: u8) -> u8 {
    
    let res = (dest as i32 - val as i32 - carry as i32) as u16;

// println!("{:X} = {:X}-{:X}-{}", res as u8, dest, val, carry);
    flags_sub(z80, dest as u16, val as u16, res);
    res as u8
}

pub fn sbc<S: Read8, B: Bus>(z80: &mut Z80, bus: &mut B, source: S) {
    let a = Reg8::A.read8(z80, bus);
    let val = source.read8(z80, bus);
    let carry = if z80.registers.get_flag(Carry) {1} else {0};
    let res = raw_sub(z80, a, val, carry);
    Reg8::A.write8(z80, bus, res as u8);
}

pub fn flags_add(z80: &mut Z80, acc: u8, add: u8, res: u16) {
    z80.registers.set_flag(Sign, res & 0x80 == 0x80);
    z80.registers.set_flag(Zero, res & 0xff == 0);
    z80.registers.set_flag(HalfCarry, (acc ^ add ^ res as u8) & 0b1_0000 != 0); //(res & 0xf) < (acc & 0xf));
    z80.registers.set_flag(Subtract, false);
    z80.registers.set_flag(Parity, !(acc^add) & (acc ^ res as u8) & 0x80 != 0); //(((acc ^ add ^ 0x80) & (add ^ res)) >> 5) & 0b0000_0100 != 0);
    z80.registers.set_flag(Carry, res & 0b1_0000_0000 != 0);

    z80.registers.set_xy(res as u8);
}

fn flags_sub(z80: &mut Z80, acc: u16, sub: u16, res: u16) {
    z80.registers.set_flag(Sign, res & 0x80 == 0x80);
    z80.registers.set_flag(Zero, res & 0xff == 0);
    z80.registers.set_flag(HalfCarry, (acc ^ sub ^ res) & 0x10 != 0);
    z80.registers.set_flag(Subtract, true);
    z80.registers.set_flag(Parity, ((acc ^ sub) & (acc ^ res)) & 0x80 != 0);
    z80.registers.set_flag(Carry, res & 0b1_0000_0000 != 0);

    z80.registers.set_xy(res as u8);
}