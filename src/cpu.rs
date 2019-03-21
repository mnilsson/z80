use crate::flags::Flag;
use crate::flags::Flag::*;

use crate::registers::*;

use crate::util::make_u16;

use crate::operations as ops;

use crate::operations::Ops;

use crate::bus::Bus;

use crate::times;

#[derive(Debug)]
pub struct ImmByte;
#[derive(Debug)]
pub struct ImmWord;

// use disassembler::Disassembler;
use crate::disassembler::traits::{IntoAddress, IntoArg16, IntoArg8, IntoCond};

pub enum Indirect {
    BC,
    DE,
    HL,
    ImmWord,
}

pub trait Source<T> {
    fn read(self, cpu: &mut Z80, bus: &mut impl Bus) -> T;
}
impl Read8 for u8 {
    fn read8(self, _: &mut Z80, _: &mut impl Bus) -> u8 {
        self
    }
}

impl Read16 for u16 {
    fn read16(self, _: &mut Z80, _: &mut impl Bus) -> u16 {
        self
    }
}

pub trait Read8: IntoArg8 {
    fn read8(self, cpu: &mut Z80, bus: &mut impl Bus) -> u8;
}

pub trait Write8: IntoArg8 {
    fn write8(self, cpu: &mut Z80, bus: &mut impl Bus, value: u8);
}

pub trait Read16: IntoArg16 + IntoAddress {
    fn read16(self, cpu: &mut Z80, bus: &mut impl Bus) -> u16;
}

pub trait Write16: IntoArg16 + IntoAddress {
    fn write16(self, cpu: &mut Z80, bus: &mut impl Bus, value: u16);
}

pub trait ReadCond: IntoCond {
    fn read_cond(self, cpu: &mut Z80) -> bool;
}

impl Source<bool> for bool {
    #[allow(unused_variables)]
    fn read(self, cpu: &mut Z80, bus: &mut impl Bus) -> bool {
        self
    }
}

pub trait Dest<T> {
    fn write(self, cpu: &mut Z80, bus: &mut impl Bus, val: T);
}

impl Source<bool> for Flag {
    fn read(self, cpu: &mut Z80, _: &mut impl Bus) -> bool {
        cpu.registers.get_flag(self)
    }
}

impl ReadCond for bool {
    fn read_cond(self, _cpu: &mut Z80) -> bool {
        self
    }
}

impl ReadCond for Flag {
    fn read_cond(self, cpu: &mut Z80) -> bool {
        cpu.registers.get_flag(self)
    }
}

impl Source<bool> for Not<Flag> {
    fn read(self, cpu: &mut Z80, _: &mut impl Bus) -> bool {
        let Not(flag) = self;
        !cpu.registers.get_flag(flag)
    }
}

impl ReadCond for Not<Flag> {
    fn read_cond(self, cpu: &mut Z80) -> bool {
        let Not(flag) = self;
        !cpu.registers.get_flag(flag)
    }
}

pub struct Not<T: Source<bool>>(pub T);
#[derive(Copy, Clone)]
pub struct Mem<T: Read16>(pub T);

#[derive(Copy, Clone)]
pub struct RelOffset<T: Read16>(pub T);

#[derive(Default)]
pub struct Z80 {
    pub registers: Registers,

    pub interrupt_mode: u8,
    iff1: u8,
    iff2: u8,
    ei_instr: bool,

    pub nmi: bool,

    pub debug: bool,

    pub sp: u16, // stack pointer
    pub pc: u16, // program counter
    halted: bool,

    pub t_cycles: u32,
    pub m_cycles: u32,
}

impl Z80 {
    pub fn new() -> Z80 {
        Z80 {
            registers: Registers::default(),

            interrupt_mode: 0,
            iff1: 0,
            iff2: 0,
            ei_instr: false,

            nmi: false,

            debug: true,
            sp: 0xdff0,
            pc: 0,

            halted: false,
            t_cycles: 0,
            m_cycles: 0,
        }
    }

    pub fn step(&mut self, bus: &mut impl Bus, int_flags: u8) -> u32 {
        self.handle_interrupt(bus, int_flags);
        self.execute_next_instruction(bus);

        0
    }

    pub fn nmi(&mut self, bus: &mut impl Bus) {
        self.nmi = true;
        self.iff2 = self.iff1;
        self.iff1 = 0;
        let pc = self.pc;
        self.push_word(bus, pc);
        self.pc = 0x0066;
        self.registers.r = (self.registers.r & 128) + 1;
    }

    pub fn reset_interrupt(&mut self, bus: &mut impl Bus) {
        if self.iff1 == 0 || self.ei_instr {
            return;
        }
        self.iff1 = 0;
        self.halted = false;
        let pc = self.pc;
        self.push_word(bus, pc);
        self.pc = 0x66;
    }

    pub fn handle_interrupt(&mut self, bus: &mut impl Bus, int_flags: u8) {
        if self.iff1 == 0 || self.ei_instr {
            return;
        }

        if int_flags != 0 && self.iff1 > 0 && self.interrupt_mode == 1 {
            self.halted = false;
            let pc = self.pc;
            self.push_word(bus, pc);
            if self.iff1 == 2 {
                let i = Reg8::I.read8(self, bus);
                let addr = (i as u16) << 8;
                let addr_lo = (i + 1) as u16;
                Reg16::PC.write16(self, bus, addr | addr_lo);
            } else {
                Reg16::PC.write16(self, bus, 0x38);
            }
            self.iff1 = 0;
            self.iff2 = 0;
        }
    }

    pub fn execute_next_instruction(&mut self, bus: &mut impl Bus) -> u8 {
        self.ei_instr = false;

        let instr = if !self.halted {
            self.read_instruction(bus)
        } else {
            bus.tick(1, times::OCF);
            0
        };

        ops::decode((self, bus), instr);
        instr
    }

    fn test_bit(&mut self, bit: u8, val: u8) {
        let res = val & (1 << bit);
        self.registers.set_flag(Zero, res == 0);
        self.registers.set_flag(Parity, res == 0);
        self.registers.set_flag(Sign, bit == 7 && res != 0);
        self.registers.set_flag(HalfCarry, true);
        self.registers.set_flag(Subtract, false);
        // self.registers.set_xy(res);
    }

    fn otir(&mut self, bus: &mut impl Bus) -> u8 {
        self.outi(bus);
        if Reg8::B.read8(self, bus) != 0 {
            self.pc -= 2;
            21
        } else {
            16
        }
    }

    fn outi(&mut self, bus: &mut impl Bus) {
        bus.tick(0, 1); // ocf 2 takes 5 tstates
        let hl = Reg16::HL.read16(self, bus);
        let io_val = bus.memory_read(hl as usize);
        bus.tick(1, times::MR);

        self.inc16(bus, Reg16::HL);
        let b = Reg8::B.read8(self, bus);
        Reg8::B.write8(self, bus, b.wrapping_sub(1));
        let bc = Reg16::BC.read16(self, bus);
        self.outp(bus, bc, io_val);

        let l = Reg8::L.read8(self, bus);

        self.registers.set_flag(Zero, b == 1);
        self.registers.set_flag(Subtract, true);
        let carry = l as u16 + b as u16 > 255;
        self.registers.set_flag(Carry, carry);
        self.registers.set_flag(HalfCarry, carry);
        // bus.tick(0, 1);
        // bus.tick(1, times::MR);
    }

    fn jp_cond<C: Source<bool>>(&mut self, bus: &mut impl Bus, cond: C) {
        let cond = cond.read(self, bus);
        let addr = self.read_u16(bus);
        if cond {
            self.pc = addr;
        }
    }

    fn call_cond<C: Source<bool>>(&mut self, bus: &mut impl Bus, cond: C) {
        bus.tick(0, 1);
        let cond = cond.read(self, bus);
        let addr = ImmWord.read16(self, bus);
        if cond {
            self.call(bus, addr);
        }
    }

    fn write_port<P: Read8, V: Read8>(&mut self, bus: &mut impl Bus, port: P, val: V) {
        let port = port.read8(self, bus);
        let val = val.read8(self, bus);
        // println!("out {:x},{:x}", port, val);
        bus.port_write(port, val);
        bus.tick(1, times::PW);
    }

    fn read_port<D: Write8, P: Read8>(&mut self, bus: &mut impl Bus, reg: D, port: P) {
        let port = port.read8(self, bus);
        let val = bus.port_read(port);
        bus.tick(1, times::PR);
        reg.write8(self, bus, val);
        self.registers.set_flag(Sign, val & 0x80 == 0x80);
        self.registers.set_flag(Zero, val == 0);
        self.registers.set_flag(HalfCarry, false);
        self.registers.set_flag(Parity, val.count_ones() % 2 == 0);
        self.registers.set_flag(Subtract, false);
        // println!("in {:x},{:x}", port, val);
    }

    pub fn read_instruction(&mut self, bus: &mut impl Bus) -> u8 {
        bus.tick(1, times::OCF);
        let val = bus.memory_read(self.pc as usize);
        self.pc += 1;
        val
    }

    pub fn read_u8(&mut self, bus: &mut impl Bus) -> u8 {
        bus.tick(1, times::OD);
        let val = bus.memory_read(self.pc as usize);
        self.pc += 1;
        val
    }

    pub fn read_u16(&mut self, bus: &mut impl Bus) -> u16 {
        let lo = bus.memory_read(self.pc as usize);
        self.pc += 1;
        bus.tick(1, times::ODL);
        let hi = bus.memory_read(self.pc as usize);
        self.pc += 1;
        bus.tick(1, times::ODH);
        make_u16(lo, hi)
    }

    pub fn read_address(&mut self, bus: &mut impl Bus) -> usize {
        self.read_u16(bus) as usize
    }

    fn rra(&mut self, bus: &mut impl Bus) {
        let val = Reg8::A.read8(self, bus);
        let carry = if self.registers.get_flag(Carry) { 1 } else { 0 };
        let res = (val >> 1 | carry << 7) & 0xff;
        self.registers.set_flag(Carry, val & 1 == 1);
        self.registers.set_flag(Subtract, false);
        self.registers.set_flag(HalfCarry, false);
        self.registers.set_xy(res);
        Reg8::A.write8(self, bus, res);
    }

    fn inc16<R: Write16 + Read16 + Copy>(&mut self, bus: &mut impl Bus, reg: R) {
        let v = reg.read16(self, bus);
        reg.write16(self, bus, v.wrapping_add(1));
    }

    pub fn dec16<R: Write16 + Read16 + Copy>(&mut self, bus: &mut impl Bus, reg: R) {
        let v = reg.read16(self, bus);
        reg.write16(self, bus, v.wrapping_sub(1));
    }

    fn call<A: Read16>(&mut self, bus: &mut impl Bus, addr: A) {
        let addr = addr.read16(self, bus);
        let pc = self.pc;
        self.push_word(bus, pc);
        self.pc = addr;
    }

    fn ldi(&mut self, bus: &mut impl Bus) {
        let de = Reg16::DE.read16(self, bus);
        let hl = Reg16::HL.read16(self, bus);
        let hl_val = bus.memory_read(hl as usize);
        bus.memory_write(de as usize, hl_val);
        self.inc16(bus, Reg16::DE);
        self.inc16(bus, Reg16::HL);
        self.dec16(bus, Reg16::BC);

        let n = (self.registers.a as u16 + hl_val as u16) as u8;

        self.registers.set_flag(Y, n & 0b10 != 0);
        self.registers.set_flag(X, n & 0b1000 != 0);

        self.registers.set_flag(HalfCarry, false);
        let bc = Reg16::BC.read16(self, bus);
        self.registers.set_flag(Parity, bc != 0);
        self.registers.set_flag(Subtract, false);
    }

    fn ldd(&mut self, bus: &mut impl Bus) {
        let de = Reg16::DE.read16(self, bus);
        let hl = Reg16::HL.read16(self, bus);
        let hl_val = bus.memory_read(hl as usize);
        bus.memory_write(de as usize, hl_val);
        self.dec16(bus, Reg16::DE);
        self.dec16(bus, Reg16::HL);
        self.dec16(bus, Reg16::BC);

        let n = (self.registers.a as u16 + hl_val as u16) as u8;
        self.registers.set_flag(Y, n & 0b10 != 0);
        self.registers.set_flag(X, n & 0b1000 != 0);

        self.registers.set_flag(HalfCarry, false);
        let bc = Reg16::BC.read16(self, bus);
        self.registers.set_flag(Parity, bc != 0);
        self.registers.set_flag(Subtract, false);
    }

    fn cpi(&mut self, bus: &mut impl Bus) {
        let a = Reg8::A.read8(self, bus);
        let hl = Reg16::HL.read16(self, bus);
        let hl_mem = bus.memory_read(hl as usize);
        let v = a.wrapping_sub(hl_mem);
        self.inc16(bus, Reg16::HL);
        self.dec16(bus, Reg16::BC);

        self.registers.set_flag(Sign, v & 0x80 == 0x80);
        self.registers.set_flag(Zero, v == 0);
        self.registers.set_flag(HalfCarry, (v & 0xf) > (a & 0xf));
        let bc = Reg16::BC.read16(self, bus);
        self.registers.set_flag(Parity, bc != 0);
        self.registers.set_flag(Subtract, true);

        let n = v - if self.registers.get_flag(HalfCarry) {
            1
        } else {
            0
        };
        self.registers.set_flag(Y, n & 0b10 != 0);
        self.registers.set_flag(X, n & 0b1000 != 0);
    }

    fn ldir(&mut self, bus: &mut impl Bus) {
        self.ldi(bus);
        self.registers.set_flag(Parity, false);
        if Reg16::BC.read16(self, bus) != 0 {
            self.pc -= 2;
        }
    }

    fn lddr(&mut self, bus: &mut impl Bus) {
        self.ldd(bus);
        self.registers.set_flag(Parity, false);
        if Reg16::BC.read16(self, bus) != 0 {
            self.pc -= 2;
        }
    }

    fn cpir(&mut self, bus: &mut impl Bus) {
        self.cpi(bus);
        if Reg16::BC.read16(self, bus) != 0 && !self.registers.get_flag(Zero) {
            self.pc -= 2;
        }
    }

    pub fn common_rot_flags(&mut self) {
        self.registers.set_flag(HalfCarry, false);
        self.registers.set_flag(Subtract, false);
    }

    fn outp(&mut self, bus: &mut impl Bus, port: u16, val: u8) {
        bus.port_write(port as u8, val);
    }

    fn push_word(&mut self, bus: &mut impl Bus, word: u16) {
        let lo = (word & 0xff) as u8;
        let hi = (word >> 8) as u8;

        self.sp -= 1;
        bus.memory_write(self.sp as usize, hi);
        bus.tick(1, times::SWH);

        self.sp -= 1;
        bus.memory_write(self.sp as usize, lo);
        bus.tick(1, times::SWL);
    }

    fn pop_byte(&mut self, bus: &mut impl Bus) -> u8 {
        let sp = self.sp;
        let val = bus.memory_read(sp as usize);
        self.sp = sp.wrapping_add(1);
        val
    }

    pub fn pop_word(&mut self, bus: &mut impl Bus) -> u16 {
        let lo = self.pop_byte(bus);
        bus.tick(1, times::SRL);

        let hi = self.pop_byte(bus);
        bus.tick(1, times::SRH);
        make_u16(lo, hi)
    }

    pub fn szp_flags(&mut self, val: u8) {
        let v = val & 0xff;
        self.registers.set_flag(Sign, val & 0x80 == 0x80);
        self.registers.set_flag(Zero, v == 0);
        self.registers.set_flag(Parity, v.count_ones() % 2 == 0);
    }

    pub fn interrupt(&mut self, bus: &mut impl Bus) {
        if self.iff1 != 0 {
            if self.halted {
                self.pc += 1;
                self.halted = false;
            }
            // cycles = 7
            let r = (Reg8::R.read8(self, bus) + 1) & 0x7f;
            Reg8::R.write8(self, bus, r);
            self.iff1 = 0;
            self.iff2 = 0;

            let pc = self.pc;
            self.push_word(bus, pc);

            match self.interrupt_mode {
                0 | 1 => {
                    self.pc = 0x0038;
                }
                2 => {
                    let i = Reg8::I.read8(self, bus);
                    let inttemp = ((i as u16) << 8) | 0xff;
                    self.pc = bus.memory_read_word(inttemp as usize);
                }
                _ => {
                    panic!("Unknown interrupt mode");
                }
            }
        }
    }
}

impl<'a, B: Bus> Ops for (&'a mut Z80, &'a mut B) {
    type R = ();

    fn and<R: Read8>(self, reg: R) {
        let (cpu, bus) = self;
        ops::and(cpu, bus, reg);
    }

    fn add8<D: Write8 + Read8 + Copy, S: Read8>(self, dest: D, source: S) {
        let (cpu, bus) = self;
        ops::add(cpu, bus, dest, source);
    }

    fn adc8<D: Write8 + Read8 + Copy, S: Read8>(self, dest: D, source: S) {
        let (cpu, bus) = self;
        ops::adc(cpu, bus, dest, source);
    }

    fn nop(self) {
        // nothing
    }
    fn ccf(self) {
        let (cpu, _) = self;
        let carry = cpu.registers.get_flag(Carry);
        cpu.registers.set_flag(HalfCarry, carry);
        cpu.registers.set_flag(Subtract, false);
        cpu.registers.set_flag(Carry, !carry);
    }

    fn cpl(self) {
        let (cpu, bus) = self;
        let a = Reg8::A.read8(cpu, bus);
        let res = a ^ 0xff;
        Reg8::A.write8(cpu, bus, res);
        cpu.registers.set_flag(HalfCarry, true);
        cpu.registers.set_flag(Subtract, true);
    }

    fn daa(self) {
        let (cpu, bus) = self;
        let mut a = Reg8::A.read8(cpu, bus) as i16;
        let n = cpu.registers.get_flag(Subtract);
        let c = cpu.registers.get_flag(Carry);
        let h = cpu.registers.get_flag(HalfCarry);

        if c || a & 0xff > 0x99 {
            if n {
                a -= 0x60
            } else {
                a += 0x60
            }
            cpu.registers.set_flag(Carry, true);
        }
        if h || a & 0xf > 0x9 {
            if n {
                a -= 0x6
            } else {
                a += 0x6
            }
        }

        let old_a = Reg8::A.read8(cpu, bus) as i16;
        cpu.registers
            .set_flag(HalfCarry, (old_a ^ a) & 0b1_0000 == 0b1_0000);
        cpu.szp_flags(a as u8);
        Reg8::A.write8(cpu, bus, a as u8);
    }

    fn dec8<R: Write8 + Read8 + Copy>(self, reg: R) {
        let (cpu, bus) = self;
        ops::dec_u8(cpu, bus, reg);
    }
    fn di(self) {
        let (cpu, _) = self;
        cpu.iff1 = 0;
        cpu.iff2 = 0;
        cpu.ei_instr = true;
    }

    fn ei(self) {
        let (cpu, _) = self;
        cpu.iff1 = 1;
        cpu.iff2 = 1;
        cpu.ei_instr = true;
    }

    fn exx(self) {
        let (cpu, bus) = self;
        use self::Reg16::*;

        let regs = [(BC, _BC), (DE, _DE), (HL, _HL)].iter();

        for &(source, dest) in regs {
            let val = source.read16(cpu, bus);
            let val2 = dest.read16(cpu, bus);
            dest.write16(cpu, bus, val);
            source.write16(cpu, bus, val2);
        }
    }

    fn ld8<D: Write8, S: Read8>(self, dest: D, source: S) {
        let (cpu, bus) = self;

        let val = source.read8(cpu, bus);

        dest.write8(cpu, bus, val);
    }

    fn ld8_int<D: Write8, S: Read8>(self, dest: D, source: S) {
        let (cpu, bus) = self;

        let val = source.read8(cpu, bus);

        dest.write8(cpu, bus, val);
        cpu.registers.set_flag(Parity, cpu.iff2 == 1);
    }

    fn ld8_address_dest<D: ReadAddress, S: Read8>(self, dest: D, source: S) {
        let (cpu, bus) = self;

        let addr = dest.read_address(cpu, bus);
        let val = source.read8(cpu, bus);
        Mem(addr).write8(cpu, bus, val);
    }

    fn ld8_address_source<D: Write8, S: ReadAddress>(self, dest: D, source: S) {
        let (cpu, bus) = self;

        let addr = source.read_address(cpu, bus);
        let val = Mem(addr).read8(cpu, bus);
        dest.write8(cpu, bus, val);
    }

    fn ld16<D: Write16, S: Read16>(self, dest: D, source: S) {
        let (cpu, bus) = self;
        let val = source.read16(cpu, bus);
        dest.write16(cpu, bus, val);
    }

    fn in8<D: Write8, S: Read8>(self, dest: D, source: S) {
        let (cpu, bus) = self;

        cpu.read_port(bus, dest, source)
    }

    fn in8_noflags<D: Write8, S: Read8>(self, _dest: D, _source: S) {
        let (cpu, bus) = self;

        let addr = cpu.read_u8(bus) as u16 | ((Reg8::A.read8(cpu, bus) as u16) << 8);
        let port_val = bus.port_read(addr as u8);
        Reg8::A.write8(cpu, bus, port_val);
    }

    fn inc8<R: Write8 + Read8 + Copy>(self, reg: R) {
        let (cpu, bus) = self;
        ops::inc_u8(cpu, bus, reg);
    }

    fn inc8_memory<R: ReadAddress>(self, reg: R) {
        let (cpu, bus) = self;
        let addr = reg.read_address(cpu, bus);
        ops::inc_u8(cpu, bus, Mem(addr));
    }

    fn dec8_memory<R: ReadAddress>(self, reg: R) {
        let (cpu, bus) = self;
        let addr = reg.read_address(cpu, bus);
        ops::dec_u8(cpu, bus, Mem(addr));
    }

    fn inc16<R: Write16 + Read16 + Copy>(self, reg: R) {
        let (cpu, bus) = self;

        let v = reg.read16(cpu, bus);
        reg.write16(cpu, bus, v.wrapping_add(1));
        bus.tick(0, 2);
    }

    fn dec16<R: Write16 + Read16 + Copy>(self, reg: R) {
        let (cpu, bus) = self;

        cpu.dec16(bus, reg);
        bus.tick(0, 2);
    }

    fn jp<A: Read16>(self, addr: A) {
        let (cpu, bus) = self;
        let addr = addr.read16(cpu, bus);

        cpu.pc = addr;
    }

    fn jp_cond<C: ReadCond, A: Read16>(self, condition: C, _: A) {
        let (cpu, bus) = self;
        let cond = condition.read_cond(cpu);
        cpu.jp_cond(bus, cond);
    }

    fn jr<C: ReadCond>(self, condition: C) {
        let (cpu, bus) = self;

        let cond = condition.read_cond(cpu);
        let temp = cpu.read_u8(bus) as i8 as i32;

        if cond {
            cpu.pc = (cpu.pc as i32 + temp) as u16;
            cpu.registers.xy_int = (cpu.pc >> 8) as u8;
            bus.tick(1, times::IO);
        }
    }

    fn djnz(self) {
        let (cpu, bus) = self;
        let b = Reg8::B.read8(cpu, bus);
        let b = b.wrapping_sub(1); //((B.read(self, bus) as i32 - 1) & 0xff) as u8;
        Reg8::B.write8(cpu, bus, b);

        if b != 0 {
            (cpu, bus).jr(true);
        } else {
            cpu.pc += 1; // @todo Not nice!!!
        }
    }

    fn ret_cond<C: ReadCond>(self, condition: C) {
        let (cpu, bus) = self;
        bus.tick(0, 1); // @todo for some reason  ocf is 5 don't know why
        if condition.read_cond(cpu) {
            cpu.pc = cpu.pop_word(bus);
        }
    }

    fn ret(self) {
        let (cpu, bus) = self;
        cpu.pc = cpu.pop_word(bus);
    }

    fn halt(self) {
        let (cpu, _) = self;
        cpu.halted = true;
    }

    fn or<R: Read8>(self, reg: R) {
        let (cpu, bus) = self;
        ops::or(cpu, bus, reg);
    }

    fn out8<D: Read8, S: Read8>(self, dest: D, source: S) {
        let (cpu, bus) = self;
        cpu.write_port(bus, dest, source)
    }

    fn out8_noflags<D: Read8, S: Read8>(self, _dest: D, _source: S) {
        let (cpu, bus) = self;
        let port = cpu.read_u8(bus);
        let a = Reg8::A.read8(cpu, bus);
        bus.port_write(port, a);
    }

    fn outi(self) {
        let (cpu, bus) = self;
        bus.tick(0, 1); // ocf #2 takes 5 tstates for outi

        let hl = Reg16::HL.read16(cpu, bus);
        let byte = bus.memory_read(hl as usize);
        bus.tick(1, times::MR);
        cpu.write_port(bus, Reg8::C, byte);

        ops::dec_u8(cpu, bus, Reg8::B);

        cpu.inc16(bus, Reg16::HL);

        let l = Reg8::L.read8(cpu, bus);
        let carry = l as u16 + byte as u16 > 255;
        cpu.registers.set_flag(Carry, carry);
        cpu.registers.set_flag(HalfCarry, carry);

        cpu.registers.set_flag(Subtract, byte & 0x80 == 0x80);
    }

    fn rla(self) {
        let (cpu, bus) = self;

        let val = Reg8::A.read8(cpu, bus);
        let carry = if cpu.registers.get_flag(Carry) { 1 } else { 0 };
        let res = val << 1 | carry;
        cpu.registers.set_flag(Carry, val >> 7 == 1);
        cpu.registers.set_flag(Subtract, false);
        cpu.registers.set_flag(HalfCarry, false);
        cpu.registers.set_xy(res);
        Reg8::A.write8(cpu, bus, res);
    }

    fn rlca(self) {
        let (cpu, bus) = self;
        let val = Reg8::A.read8(cpu, bus);
        let res = val << 1 | val >> 7;
        cpu.registers.set_flag(Carry, val >> 7 == 1);
        cpu.registers.set_flag(Subtract, false);
        cpu.registers.set_flag(HalfCarry, false);
        cpu.registers.set_xy(res);
        Reg8::A.write8(cpu, bus, res);
    }

    fn rra(self) {
        let (cpu, bus) = self;
        cpu.rra(bus);
    }

    fn rrca(self) {
        let (cpu, bus) = self;
        let val = Reg8::A.read8(cpu, bus);
        let res = (val >> 1 | val << 7) & 0xff;
        cpu.registers.set_flag(Carry, val & 0b1 == 1);
        cpu.registers.set_flag(Subtract, false);
        cpu.registers.set_flag(HalfCarry, false);
        cpu.registers.set_xy(res);
        Reg8::A.write8(cpu, bus, res);
    }

    fn neg(self) {
        let (cpu, bus) = self;

        let temp = Reg8::A.read8(cpu, bus);
        Reg8::A.write8(cpu, bus, 0);
        ops::sub(cpu, bus, temp);
        cpu.registers.set_flag(Parity, temp == 0x80);
        cpu.registers.set_flag(Carry, temp != 0);
    }

    fn retn(self) {
        let (cpu, bus) = self;
        let pc = cpu.pop_word(bus);
        cpu.pc = pc;
        cpu.iff1 = cpu.iff2;
        cpu.nmi = false;
    }

    fn sbc16<D: Write16 + Read16 + Copy, S: Read16>(self, dest: D, source: S) {
        let (cpu, bus) = self;

        let destval = dest.read16(cpu, bus) as u32;
        let val = source.read16(cpu, bus) as u32;
        let carry1 = if cpu.registers.get_flag(Carry) { 1 } else { 0 };
        let lo = ops::raw_sub(cpu, destval as u8, val as u8, carry1);
        let carry = if cpu.registers.get_flag(Carry) { 1 } else { 0 };
        let hi = ops::raw_sub(cpu, (destval >> 8) as u8, (val >> 8) as u8, carry);
        let res = make_u16(lo, hi);

        dest.write16(cpu, bus, res);
        cpu.registers.set_flag(Zero, res == 0);
    }

    fn scf(self) {
        let (cpu, _) = self;
        cpu.registers.set_flag(HalfCarry, false);
        cpu.registers.set_flag(Subtract, false);
        cpu.registers.set_flag(Carry, true);
    }

    fn xor<R: Read8>(self, reg: R) {
        let (cpu, bus) = self;
        ops::xor(cpu, bus, reg);
    }

    fn adc16<D: Write16 + Read16 + Copy, S: Read16>(self, dest: D, source: S) {
        let (cpu, bus) = self;

        let val = source.read16(cpu, bus) as u32;
        let destval = dest.read16(cpu, bus) as u32;
        let carry = if cpu.registers.get_flag(Carry) { 1 } else { 0 };
        let lo = ops::raw_addc(cpu, destval as u8, val as u8, carry);
        let carry = if cpu.registers.get_flag(Carry) { 1 } else { 0 };
        let hi = ops::raw_addc(cpu, (destval >> 8) as u8, (val >> 8) as u8, carry);
        let res = make_u16(lo, hi);
        Reg16::HL.write16(cpu, bus, res);
        cpu.registers.set_flag(Zero, res == 0);

        dest.write16(cpu, bus, res as u16);
    }

    fn cpi(self) {
        let (cpu, bus) = self;
        cpu.cpi(bus);
    }
    fn cpir(self) {
        let (cpu, bus) = self;
        cpu.cpir(bus);
    }
    fn cpd(self) {
        let (cpu, bus) = self;
        ops::cpd(cpu, bus);
    }
    fn cpdr(self) {
        let (cpu, bus) = self;

        ops::cpd(cpu, bus);
        if Reg16::BC.read16(cpu, bus) != 0 && !cpu.registers.get_flag(Zero) {
            cpu.pc -= 2;
        }
    }

    fn ldd(self) {
        let (cpu, bus) = self;
        cpu.ldd(bus);
    }
    fn lddr(self) {
        let (cpu, bus) = self;
        cpu.lddr(bus);
    }

    fn otir(self) {
        let (cpu, bus) = self;
        cpu.otir(bus);
    }
    fn ldir(self) {
        let (cpu, bus) = self;
        cpu.ldir(bus);
    }

    fn outd(self) {
        let (cpu, bus) = self;
        let hl = Reg16::HL.read16(cpu, bus);
        let byte = bus.memory_read(hl as usize);
        cpu.write_port(bus, Reg8::C, byte);

        ops::dec_u8(cpu, bus, Reg8::B);

        cpu.dec16(bus, Reg16::HL);

        let l = Reg8::L.read8(cpu, bus);
        let carry = l as u16 + byte as u16 > 255;
        cpu.registers.set_flag(Carry, carry);
        cpu.registers.set_flag(HalfCarry, carry);

        cpu.registers.set_flag(Subtract, byte & 0x80 == 0x80);
    }
    fn ini(self) {
        let (cpu, bus) = self;

        let bc = Reg16::BC.read16(cpu, bus);
        let hl = Reg16::HL.read16(cpu, bus);
        let ini = bus.port_read(bc as u8);
        bus.port_write(hl as u8, ini);
        let b = Reg8::B.read8(cpu, bus);
        ops::dec_u8(cpu, bus, Reg8::B);
        cpu.inc16(bus, Reg16::HL);
        //                cpu.set_hl(hl + 1);

        let ini2 = ini.wrapping_add(Reg8::C.read8(cpu, bus)).wrapping_add(1);
        cpu.registers.set_flag(Subtract, ini & 0x80 != 0);
        cpu.registers.set_flag(HalfCarry, ini2 < ini);
        cpu.registers.set_flag(Carry, ini2 < ini);
        cpu.registers.set_flag(Parity, (ini2 & 0x07) ^ b != 0);
    }

    fn im(self, im: u8) {
        let (cpu, _) = self;
        cpu.interrupt_mode = im;
    }

    fn rrd(self) {
        let (cpu, bus) = self;

        let addr = Reg16::HL.read16(cpu, bus) as usize;
        let v = bus.memory_read(addr);
        let a = Reg8::A.read8(cpu, bus);
        let ah = a & 0xf0;
        let al = a & 0x0f;
        let a = ah | (v & 0x0f);
        Reg8::A.write8(cpu, bus, a);
        bus.memory_write(addr, (v >> 4 | al << 4) & 0xff);
        // wz = addr + 1
        cpu.szp_flags(a);
        cpu.registers.set_xy(a);
        cpu.common_rot_flags();
    }

    fn rld(self) {
        let (cpu, bus) = self;
        // cpu.rld(bus);
        let addr = Reg16::HL.read16(cpu, bus) as usize;
        let v = bus.memory_read(addr);
        let a = Reg8::A.read8(cpu, bus);
        let ah = a & 0xf0;
        let al = a & 0x0f;

        let a = ah | (v >> 4 & 0x0f);
        Reg8::A.write8(cpu, bus, a);
        bus.memory_write(addr, (v << 4 | al) & 0xff);
        // wz = addr + 1
        cpu.szp_flags(a);
        cpu.registers.set_xy(a);
        cpu.common_rot_flags();
    }

    fn reti(self) {
        let (cpu, bus) = self;
        let pc = cpu.pop_word(bus);
        cpu.pc = pc;
        cpu.iff1 = 1;
        cpu.iff2 = 1;
    }

    fn ldi(self) {
        let (cpu, bus) = self;
        cpu.ldi(bus);
    }
    fn otdr(self) {
        panic!("otdr?")
    }

    fn inir(self) {
        panic!("inir?")
    }

    fn ind(self) {
        let (cpu, bus) = self;
        let c_val = Reg8::C.read8(cpu, bus);

        let port_val = bus.port_read(c_val);

        let hl_val = Reg16::HL.read16(cpu, bus);
        bus.memory_write(hl_val.into(), port_val);

        let b_val = Reg8::B.read8(cpu, bus);

        Reg8::B.write8(cpu, bus, b_val.wrapping_sub(1));
        Reg16::HL.write16(cpu, bus, hl_val.wrapping_sub(1));

        Flag::Subtract.write(&mut cpu.registers, true);
        Flag::Zero.write(&mut cpu.registers, b_val == 1);
    }

    fn indr(self) {
        panic!("indr?");
    }
    fn cb_op(self) {
        let (cpu, bus) = self;
        let op = cpu.read_instruction(bus);
        ops::decode_cb((cpu, bus), op);
    }

    fn dd_op(self) {
        let (cpu, bus) = self;
        let op = cpu.read_instruction(bus);

        ops::decode_dd((cpu, bus), op);
    }

    fn ed_op(self) {
        let (cpu, bus) = self;
        let op = cpu.read_instruction(bus);

        ops::decode_ed((cpu, bus), op);
    }

    fn fd_op(self) {
        let (cpu, bus) = self;
        let op = cpu.read_instruction(bus);
        ops::decode_fd((cpu, bus), op);
    }

    fn dd_fd_cb_op(self, ireg: Reg16) {
        let (cpu, bus) = self;
        let address = RelOffset(ireg).read16(cpu, bus);
        let op = cpu.read_instruction(bus);
        // let op = bus.memory_read(cpu.pc as usize);
        // cpu.pc += 1;
        // bus.tick(0, times::OCF);

        ops::decode_dd_fd_cb((cpu, bus), address, op)
    }

    fn ex<D: Write16 + Read16 + Copy, S: Write16 + Read16 + Copy>(self, dest: D, source: S) {
        let (cpu, bus) = self;
        let val = source.read16(cpu, bus);
        let val2 = dest.read16(cpu, bus);
        dest.write16(cpu, bus, val);
        source.write16(cpu, bus, val2);
    }

    fn cp<S: Read8>(self, source: S) {
        let (cpu, bus) = self;
        ops::cp(cpu, bus, source);
    }

    fn bit<S: Read8>(self, bit: u8, source: S) {
        let (cpu, bus) = self;
        let val = source.read8(cpu, bus);
        cpu.test_bit(bit, val);
    }

    fn set<S: Read8 + Write8 + Copy>(self, bit: u8, source: S) {
        let (cpu, bus) = self;
        let val: u8 = source.read8(cpu, bus);
        source.write8(cpu, bus, val | (1 << bit));
    }

    fn res<S: Read8 + Write8 + Copy>(self, bit: u8, source: S) {
        let (cpu, bus) = self;
        let val: u8 = source.read8(cpu, bus);
        source.write8(cpu, bus, val & !(1 << bit));
    }

    fn srl<S: Read8 + Write8 + Copy>(self, source: S) {
        let (cpu, bus) = self;
        ops::srl(cpu, bus, source);
    }

    fn sll<S: Read8 + Write8 + Copy>(self, source: S) {
        let (cpu, bus) = self;
        ops::sll(cpu, bus, source);
    }

    fn sra<S: Read8 + Write8 + Copy>(self, source: S) {
        let (cpu, bus) = self;
        ops::sra(cpu, bus, source);
    }

    fn rl<S: Read8 + Write8 + Copy>(self, source: S) {
        let (cpu, bus) = self;
        ops::rl(cpu, bus, source);
    }
    fn rlc<S: Read8 + Write8 + Copy>(self, source: S) {
        let (cpu, bus) = self;
        ops::rlc(cpu, bus, source);
    }

    fn rr<S: Read8 + Write8 + Copy>(self, source: S) {
        let (cpu, bus) = self;
        ops::rr(cpu, bus, source);
    }

    fn sla<S: Read8 + Write8 + Copy>(self, source: S) {
        let (cpu, bus) = self;
        ops::sla(cpu, bus, source);
    }

    fn rrc<S: Read8 + Write8 + Copy>(self, source: S) {
        let (cpu, bus) = self;
        ops::rrc(cpu, bus, source);
    }

    fn sub8<S: Read8>(self, source: S) {
        let (cpu, bus) = self;
        ops::sub(cpu, bus, source);
    }

    fn sbc8<S: Read8>(self, source: S) {
        let (cpu, bus) = self;
        ops::sbc(cpu, bus, source);
    }

    fn call<A: Read16>(self, _addr: A) {
        let (cpu, bus) = self;
        cpu.call_cond(bus, true);
    }

    fn call_cond<C: ReadCond, A: Read16>(self, condition: C, _addr: A) {
        let (cpu, bus) = self;
        let cond = condition.read_cond(cpu);
        cpu.call_cond(bus, cond);
    }

    fn rst(self, byte: u8) {
        let (cpu, bus) = self;
        let pc = cpu.pc;
        cpu.push_word(bus, pc);
        cpu.pc = byte as u16;
    }

    fn pop<T: Write16>(self, target: T) {
        let (cpu, bus) = self;

        let val = cpu.pop_word(bus);
        target.write16(cpu, bus, val);
    }

    fn push<S: Read16>(self, source: S) {
        let (cpu, bus) = self;
        let val = source.read16(cpu, bus);
        cpu.push_word(bus, val);
    }

    fn add16<D: Write16 + Read16 + Copy, S: Read16>(self, dest: D, source: S) {
        let (cpu, bus) = self;

        let val = source.read16(cpu, bus) as u32;
        let destval = dest.read16(cpu, bus) as u32;

        let res = val + destval;

        cpu.registers
            .set_flag(HalfCarry, ((destval ^ res ^ val) >> 8) & (1 << 4) != 0);
        cpu.registers.set_flag(Subtract, false);
        cpu.registers.set_flag(Carry, res >> 16 == 1);
        cpu.registers.set_xy((res >> 8) as u8);

        dest.write16(cpu, bus, res as u16);

        cpu.registers.xy_int = cpu.registers.h;
    }
}
