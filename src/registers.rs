use cpu::{Z80, ImmByte, ImmWord, RelOffset, Read8, Read16, Write8, Write16};
use cpu::Mem;
use flags::Flag;
use bus::Bus;
use util::make_u16;
use times;

use std::fmt;

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AF: 0x{:02X}{:02X}, BC: 0x{:02X}{:02X} DE: 0x{:02X}{:02X} HL: 0x{:02X}{:02X} Flags: {:08b}", self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l, self.f)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,

    R,
    I,

    IXH,
    IXL,
    IYH,
    IYL,

    _A,
    _B,
    _C,
    _D,
    _E,
    _F,
    _H,
    _L,
}
impl fmt::Display for Reg8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Reg8::*;
        match *self {
            A => write!(f, "a"),
            B => write!(f, "b"),
            C => write!(f, "c"),
            D => write!(f, "d"),
            E => write!(f, "e"),
            F => write!(f, "f"),
            H => write!(f, "h"),
            L => write!(f, "l"),

            R => write!(f, "r"),
            I => write!(f, "i"),

            IXH => write!(f, "ixh"),
            IXL => write!(f, "ixl"),
            IYH => write!(f, "iyh"),
            IYL => write!(f, "iyl"),

            _A => write!(f, "'a"),
            _B => write!(f, "'b"),
            _C => write!(f, "'c"),
            _D => write!(f, "'d"),
            _E => write!(f, "'e"),
            _F => write!(f, "'f"),
            _H => write!(f, "'h"),
            _L => write!(f, "'l"),
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,

    SP,
    PC,
    IX,
    IY,

    _AF,
    _BC,
    _DE,
    _HL,
}

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub r: u8,
    pub i: u8,


    pub ix: u16,
    pub iy: u16,

    _a: u8,
    _b: u8,
    _c: u8,
    _d: u8,
    _e: u8,
    _f: u8,
    _h: u8,
    _l: u8,

    pub xy_int: u8,
}

impl Default for Registers {
    fn default() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            r: 0x8c,
            i: 0,

            ix: 0,
            iy: 0,

            _a: 0,
            _b: 0,
            _c: 0,
            _d: 0,
            _e: 0,
            _f: 0,
            _h: 0,
            _l: 0,

            xy_int: 0,

        }
    }

}

impl Registers {
    pub fn set_flag(&mut self, flag: Flag, val: bool) {
        flag.write(self, val)
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        flag.read(self)
    }

    pub fn set_xy(&mut self, val: u8) {
        Flag::Y.write(self, val & 0b0010_0000 != 0);
        Flag::X.write(self, val & 0b0000_1000 != 0);
    }


    fn alt_swap8(&mut self, register: Reg8) {
        match register {
            Reg8::B => { let r = self.b; self.b = self._b; self._b = r },
            Reg8::C => { let r = self.c; self.c = self._c; self._c = r },
            Reg8::D => { let r = self.d; self.d = self._d; self._d = r },
            Reg8::E => { let r = self.e; self.e = self._e; self._e = r },
            Reg8::F => { let r = self.f; self.f = self._f; self._f = r },
            Reg8::H => { let r = self.h; self.h = self._h; self._h = r },
            Reg8::L => { let r = self.l; self.l = self._l; self._l = r },
            Reg8::A => { let r = self.a; self.a = self._a; self._a = r },
            _ => panic!("No no alt register for {:?}", register)
        };
    }

    pub fn alt_swap(&mut self, register: Reg16) {
        match register {
            Reg16::AF => {self.alt_swap8(Reg8::A); self.alt_swap8(Reg8::F);},
            Reg16::BC => {self.alt_swap8(Reg8::B); self.alt_swap8(Reg8::C);},
            Reg16::DE => {self.alt_swap8(Reg8::D); self.alt_swap8(Reg8::E);},
            Reg16::HL => {self.alt_swap8(Reg8::H); self.alt_swap8(Reg8::L);},
            _ => panic!("No no alt register for {:?}", register)
        }
    }

}



impl Read8 for Reg8 {
    fn read8<B: Bus>(self, cpu: &mut Z80, _: &mut B) -> u8 {
        use self::Reg8::*;

        match self {
            A => cpu.registers.a,
            B => cpu.registers.b,
            C => cpu.registers.c,
            D => cpu.registers.d,
            E => cpu.registers.e,
            F => cpu.registers.f,
            H => cpu.registers.h,
            L => cpu.registers.l,
            R => cpu.registers.r,
            I => cpu.registers.i,
            IXH => (cpu.registers.ix >> 8) as u8,
            IXL => cpu.registers.ix as u8,
            IYH => (cpu.registers.iy >> 8) as u8,
            IYL => cpu.registers.iy as u8,

            _A => cpu.registers._a,
            _B => cpu.registers._b,
            _C => cpu.registers._c,
            _D => cpu.registers._d,
            _E => cpu.registers._e,
            _F => cpu.registers._f,
            _H => cpu.registers._h,
            _L => cpu.registers._l,
        }
    }
}



impl Write8 for Reg8 {
    fn write8<B: Bus>(self, cpu: &mut Z80, _: &mut B, val: u8) {
        use self::Reg8::*;
        match self {
            A => cpu.registers.a = val,
            B => cpu.registers.b = val,
            C => cpu.registers.c = val,
            D => cpu.registers.d = val,
            E => cpu.registers.e = val,
            F => cpu.registers.f = val,
            H => cpu.registers.h = val,
            L => cpu.registers.l = val,
            R => cpu.registers.r = val,
            I => cpu.registers.i = val,

            IXH => cpu.registers.ix = make_u16(cpu.registers.ix as u8, val),
            IXL => cpu.registers.ix = make_u16(val, (cpu.registers.ix >> 8) as u8),
            IYH => cpu.registers.iy = make_u16(cpu.registers.iy as u8, val),
            IYL => cpu.registers.iy = make_u16(val, (cpu.registers.iy >> 8) as u8),

            _A => cpu.registers._a = val,
            _B => cpu.registers._b = val,
            _C => cpu.registers._c = val,
            _D => cpu.registers._d = val,
            _E => cpu.registers._e = val,
            _F => cpu.registers._f = val,
            _H => cpu.registers._h = val,
            _L => cpu.registers._l = val,
        }
    }
}



impl Read16 for Reg16 {
    fn read16<B: Bus>(self, cpu: &mut Z80, bus: &mut B) -> u16 {
        use self::Reg8::*;
        use self::Reg16::*;
        match self {
            AF => make_u16(F.read8(cpu, bus), A.read8(cpu, bus)),
            BC => make_u16(C.read8(cpu, bus), B.read8(cpu, bus)),
            DE => make_u16(E.read8(cpu, bus), D.read8(cpu, bus)),
            HL => make_u16(L.read8(cpu, bus), H.read8(cpu, bus)),
            SP => cpu.sp,
            PC => cpu.pc,
            IX => cpu.registers.ix,
            IY => cpu.registers.iy,

            _AF => make_u16(_F.read8(cpu, bus), _A.read8(cpu, bus)),
            _BC => make_u16(_C.read8(cpu, bus), _B.read8(cpu, bus)),
            _DE => make_u16(_E.read8(cpu, bus), _D.read8(cpu, bus)),
            _HL => make_u16(_L.read8(cpu, bus), _H.read8(cpu, bus)),
        }
    }
}



impl Write16 for Reg16 {
    fn write16<B: Bus>(self, cpu: &mut Z80, bus: &mut B, val: u16) {
        use self::Reg16::*;
        match self {
            AF => { Reg8::A.write8(cpu, bus, (val >> 8) as u8); Reg8::F.write8(cpu, bus, val as u8);}
            BC => { Reg8::B.write8(cpu, bus, (val >> 8) as u8); Reg8::C.write8(cpu, bus, val as u8);}
            DE => { Reg8::D.write8(cpu, bus, (val >> 8) as u8); Reg8::E.write8(cpu, bus, val as u8);}
            HL => { Reg8::H.write8(cpu, bus, (val >> 8) as u8); Reg8::L.write8(cpu, bus, val as u8);}
            SP => cpu.sp = val,
            PC => cpu.pc = val,
            IX => cpu.registers.ix = val,
            IY => cpu.registers.iy = val,
            _AF => { cpu.registers._a = (val >> 8) as u8; cpu.registers._f = val as u8; }
            _BC => { cpu.registers._b = (val >> 8) as u8; cpu.registers._c = val as u8; }
            _DE => { cpu.registers._d = (val >> 8) as u8; cpu.registers._e = val as u8; }
            _HL => { cpu.registers._h = (val >> 8) as u8; cpu.registers._l = val as u8; }
        }
    }
}


impl Read8 for ImmByte {
    fn read8<B: Bus>(self, cpu: &mut Z80, bus: &mut B) -> u8 {
        cpu.read_u8(bus)
    }
}


impl Read16 for ImmWord {
    fn read16<B: Bus>(self, cpu: &mut Z80, bus: &mut B) -> u16 {
        // cpu.t_cycles += 3;
        // cpu.m_cycles += 1;
        // cpu.t_cycles += 3;
        // cpu.m_cycles += 1;
        cpu.read_u16(bus)
    }
}

impl Write8 for Mem<ImmWord> {
    fn write8<B: Bus>(self, cpu: &mut Z80, bus: &mut B, val: u8) {
        let Mem(imm) = self;
        // cpu.t_cycles += 3;
        // cpu.m_cycles += 1;
        let addr = imm.read16(cpu, bus);
        bus.memory_write(addr as usize, val);
        bus.tick(1, times::MW);
    }
}

impl Write16 for Mem<ImmWord> {
    fn write16<B: Bus>(self, cpu: &mut Z80, bus: &mut B, val: u16) {
        let Mem(imm) = self;
        // cpu.t_cycles += 3;
        // cpu.m_cycles += 1;
        // cpu.t_cycles += 3;
        // cpu.m_cycles += 1;
        let addr = imm.read16(cpu, bus);
        let lo = val as u8;
        let hi = (val >> 8) as u8;
        bus.memory_write(addr as usize, lo);
        bus.tick(1, times::MWL);
        bus.memory_write(addr as usize + 1, hi);
        bus.tick(1, times::MWH);
    }
}

impl Write16 for Mem<Reg16> {
    fn write16<B: Bus>(self, cpu: &mut Z80, bus: &mut B, val: u16) {
        let Mem(imm) = self;
        
        let addr = imm.read16(cpu, bus);
        let lo = val as u8;
        let hi = (val >> 8) as u8;
        bus.memory_write(addr as usize, lo);
        bus.tick(1, times::MWL);
        bus.memory_write(addr as usize + 1, hi);
        bus.tick(1, times::MWH);
    }
}

impl Write8 for Mem<u16> {
    fn write8<B: Bus>(self, cpu: &mut Z80, bus: &mut B, val: u8) {
        let Mem(imm) = self;
        let addr = imm.read16(cpu, bus);
        bus.memory_write(addr as usize, val);
        bus.tick(1, times::MW);
        
    }
}

impl Read16 for Mem<ImmWord> {
    fn read16<B: Bus>(self, cpu: &mut Z80, bus: &mut B) -> u16 {
        let Mem(imm) = self;
        let addr = imm.read16(cpu, bus);
        let lo = bus.memory_read(addr as usize);
        bus.tick(1, times::MRL);
        let hi = bus.memory_read(addr as usize + 1);
        bus.tick(1, times::MRH);
        make_u16(lo, hi)
    }
}

impl Read16 for Mem<Reg16>{
    fn read16<B: Bus>(self, cpu: &mut Z80, bus: &mut B) -> u16 {
        let Mem(reg) = self;
        let addr = reg.read16(cpu, bus);
        let lo = bus.memory_read(addr as usize);
        bus.tick(1, times::MRL);
        let hi = bus.memory_read(addr as usize + 1);
        bus.tick(1, times::MRH);
        make_u16(lo, hi)
    }
}


impl Write8 for Mem<Reg16> {
    fn write8<B: Bus>(self, cpu: &mut Z80, bus: &mut B, val: u8) {
        let Mem(imm) = self;
        // cpu.t_cycles += 3;
        // cpu.m_cycles += 1;
        let addr = imm.read16(cpu, bus);
        bus.memory_write(addr as usize, val);
        bus.tick(1, times::MW);
    }
}

impl Read8 for Mem<Reg16> {
    fn read8<B: Bus>(self, cpu: &mut Z80, bus: &mut B) -> u8 {
        let Mem(reg) = self;
        let addr = reg.read16(cpu, bus);
        bus.tick(1, times::MR);
        bus.memory_read(addr as usize)
    }
}

//
impl Read16 for RelOffset<u16> {
    fn read16<B: Bus>(self, cpu: &mut Z80, bus: &mut B) -> u16 {
        let RelOffset(reg) = self;
        let offset = cpu.read_u8(bus) as i8 as i32;
        let val = reg.read16(cpu, bus) as i32;
        bus.tick(1, times::IO);
        (val + offset) as u16
    }
}

impl Read16 for RelOffset<Reg16> {
    fn read16<B: Bus>(self, cpu: &mut Z80, bus: &mut B) -> u16 {
        let RelOffset(reg) = self;
        let offset = cpu.read_u8(bus) as i8 as i32;
        let val = reg.read16(cpu, bus) as i32;
        bus.tick(1, times::IO);
        (val + offset) as u16
    }
}

impl Write8 for Mem<RelOffset<Reg16>> {
    fn write8<B: Bus>(self, cpu: &mut Z80, bus: &mut B, val: u8) {
        let Mem(imm) = self;
        let addr = imm.read16(cpu, bus);
        bus.tick(1, times::MW);
        bus.memory_write(addr as usize, val)
    }
}

pub trait ReadAddress {
    fn read_address<B: Bus>(self, cpu: &mut Z80, bus: &mut B) -> u16;
}

impl ReadAddress for Mem<RelOffset<Reg16>> {
    fn read_address<B: Bus>(self, cpu: &mut Z80, bus: &mut B) -> u16 {
        let Mem(addr) = self;
        addr.read16(cpu, bus)
    }
}

//
impl Read8 for Mem<RelOffset<Reg16>> {
    fn read8<B: Bus>(self, cpu: &mut Z80, bus: &mut B) -> u8 {
        let Mem(reg) = self;
        let addr = reg.read16(cpu, bus);
        bus.tick(1, times::MR);
        bus.memory_read(addr as usize)
    }
}

impl Read8 for Mem<RelOffset<u16>> {
    fn read8<B: Bus>(self, cpu: &mut Z80, bus: &mut B) -> u8 {
        let Mem(reg) = self;
        let addr = reg.read16(cpu, bus);
        bus.tick(1, times::MR);
        bus.memory_read(addr as usize)
    }
}

impl Read8 for Mem<ImmWord> {
    fn read8<B: Bus>(self, cpu: &mut Z80, bus: &mut B) -> u8 {

        let Mem(val) = self;
        let addr = val.read16(cpu, bus);
        bus.tick(1, times::MR);
        bus.memory_read(addr as usize)
    }
}

impl Read8 for Mem<u16> {
    fn read8<B: Bus>(self, cpu: &mut Z80, bus: &mut B) -> u8 {
        let Mem(val) = self;
        let addr = val.read16(cpu, bus);
        bus.tick(1, times::MR);
        bus.memory_read(addr as usize)
    }
}
