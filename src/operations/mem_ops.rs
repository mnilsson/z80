use crate::cpu::{Z80, Read8, Read16};

use crate::flags::Flag::*;
use crate::bus::Bus;
use crate::registers::*;




pub fn cpd(cpu: &mut Z80, bus: &mut impl Bus) {
    let a = Reg8::A.read8(cpu, bus);
    let hl = Reg16::HL.read16(cpu, bus);
    let hl_mem = bus.memory_read(hl as usize);
    let v = a.wrapping_sub(hl_mem);
    cpu.dec16(bus, Reg16::HL);
    cpu.dec16(bus, Reg16::BC);

    cpu.registers.set_flag(Sign, v & 0x80 == 0x80);
    cpu.registers.set_flag(Zero, v == 0);
    cpu.registers.set_flag(HalfCarry, (v & 0xf) > (a & 0xf));
    let bc = Reg16::BC.read16(cpu, bus);
    cpu.registers.set_flag(Parity, bc != 0);
    cpu.registers.set_flag(Subtract, true);

    let n = v - if cpu.registers.get_flag(HalfCarry) { 1 } else { 0 };
    cpu.registers.set_flag(Y, n & 0b10 != 0);
    cpu.registers.set_flag(X, n & 0b1000 != 0);
}