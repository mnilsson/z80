use cpu::{Z80, Read8, Write8};

use flags::Flag::*;
use bus::Bus;
use registers::*;




pub fn or<S: Read8, B: Bus>(z80: &mut Z80, bus: &mut B, reg: S) {
    let val = reg.read8(z80, bus);
    let res = Reg8::A.read8(z80, bus) | val;
    z80.registers.f = 0;
    common_logic_flags(z80, res);

    z80.registers.set_flag(HalfCarry, false);
    z80.registers.set_flag(Subtract, false);
    z80.registers.set_flag(Carry, false);
    Reg8::A.write8(z80, bus, res);
}

pub fn and<S: Read8, B: Bus>(z80: &mut Z80, bus: &mut B, reg: S) {
    let val = reg.read8(z80, bus);
    let res = Reg8::A.read8(z80, bus) & val;
    z80.registers.f = 0;
    common_logic_flags(z80, res);
    z80.registers.set_flag(HalfCarry, true);
    z80.registers.set_flag(Subtract, false);
    z80.registers.set_flag(Carry, false);
    z80.registers.set_xy(res as u8);
    Reg8::A.write8(z80, bus, res);
}

pub fn xor<S: Read8, B: Bus>(z80: &mut Z80, bus: &mut B, reg: S) {
    let val = reg.read8(z80, bus);
    let res = Reg8::A.read8(z80, bus) ^ val;
    z80.registers.f = 0;
    common_logic_flags(z80, res);
    z80.registers.set_flag(HalfCarry, false);
    z80.registers.set_flag(Subtract, false);
    z80.registers.set_flag(Carry, false);
    Reg8::A.write8(z80, bus, res);
}

pub fn cp<S: Read8, B: Bus>(z80: &mut Z80, bus: &mut B, source: S) {
    let val = source.read8(z80, bus) as u16;
    let a = Reg8::A.read8(z80, bus) as u16;
    let res = (a as i32 - val as i32) as u16;

    z80.registers.set_flag(Zero, res as u8 == 0);
    z80.registers.set_flag(Subtract, true);
    z80.registers.set_flag(Carry, res >> 8 != 0);
    z80.registers.set_flag(HalfCarry, (a ^ val ^ res) & (1 << 4) != 0);
    z80.registers.set_flag(Sign, res & 0x80 == 0x80);
    z80.registers.set_flag(Parity, (((a ^ val) & (res ^ a)) >> 5) & (1<<2) != 0);
    z80.registers.set_xy(val as u8);
}

fn common_logic_flags(z80: &mut Z80, val: u8) {

    z80.registers.set_flag(Parity, val.count_ones() & 1 == 0);
    z80.registers.set_flag(Zero, val == 0);
    z80.registers.set_flag(Sign, val & 0x80 == 0x80);
    z80.registers.set_xy(val);
}

