use std::fmt;








#[derive(Debug)]
pub struct Data8(pub u8);

impl fmt::Display for Data8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${:02x}", self.0)
    }
}

#[derive(Debug)]
pub struct Data16(pub u16);
impl fmt::Display for Data16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "${:04x}", self.0)
    }
}

#[derive(Debug)]
pub enum Address {
    Direct(Data16),
    BC,
    DE,
    HL,
    ZeroPage(Data8),
    RelOffset(Data16, Data8),
}



impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Address::*;
        match *self {
            Direct(ref addr) => write!(f, "{}", addr),
            ZeroPage(ref addr) => write!(f, "{}", addr),
            _ => write!(f, "{:?}", *self),
        }
    }
}

#[derive(Debug)]
pub enum Cond {
    Zero,
    NotZero,
    Carry,
    NotCarry,
    Positive,
    Negative,
}

impl fmt::Display for Cond {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            
            &Cond::Zero => write!(f, "z"),
            &Cond::NotZero => write!(f, "nz"),
            &Cond::Carry => write!(f, "c"),
            &Cond::NotCarry => write!(f, "nc"),
            &Cond::Positive => write!(f, "p"),
            &Cond::Negative => write!(f, "m"),
        }
    }
}

#[derive(Debug)]
pub enum Arg8 {
    Register(Reg8),
    Immediate(Data8),
    Memory(Address),
}
impl fmt::Display for Arg8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Arg8::*;
        match *self {
            Register(reg) => write!(f, "{}", reg),
            Immediate(ref imm) => write!(f, "{}", imm), 
            Memory(ref addr) => write!(f, "({})", addr), 
        }
    }
}

#[derive(Debug)]
pub enum Arg16 {
    Register(Reg16),
    Immediate(Data16),
    Memory(Address),
}
impl fmt::Display for Arg16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Arg16::*;
        match *self {
            Register(reg) => write!(f, "{:?}", reg),
            Immediate(ref imm) => write!(f, "{}", imm), 
            Memory(ref addr) => write!(f, "({})", addr), 
        }
    }
}

use registers::{Reg8, Reg16};

pub enum Prefix {
    CB,
    DD,
    ED,
    FD,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    ADD8(Arg8, Arg8),
    ADD16(Arg16, Arg16),
    AND(Arg8),
    CP(Arg8),
    CALL(Address),
    CALL_COND(Cond, Address),
    DEC16(Arg16),
    DI,
    DJNZ,
    EX(Arg16, Arg16),
    HALT,
    IN,
    INC8,
    INC16(Arg16),
    JP(Address),
    JP_COND(Cond, Address),
    JR(Address),
    LD8(Arg8, Arg8),
    LD16(Arg16, Arg16),
    LDIR,
    NOP,
    OR(Arg8),
    OUT(Arg8, Arg8),
    POP(Arg16),
    PUSH(Arg16),
    RET,
    RLA,
    RLCA,
    RRA,
    RRCA,
    RST(u8),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::ADD8(ref d, ref s) => write!(f, "add {},{}", d, s),
            Instruction::ADD16(ref d, ref s) => write!(f, "add {},{}", d, s),
            Instruction::AND(ref val) => write!(f, "and {}", val),
            Instruction::CP(ref val) => write!(f, "cp {}", val),
            Instruction::CALL(ref addr) => write!(f, "call {}", addr),
            Instruction::CALL_COND(ref cond, ref addr) => write!(f, "call {},{}", cond, addr),
            
            Instruction::DEC16(ref reg) => write!(f, "dec {}", reg),
            Instruction::EX(ref a, ref b) => write!(f, "ex {},{}", a, b),
            Instruction::INC16(ref reg) => write!(f, "inc {}", reg),
            Instruction::JP(ref addr) => write!(f, "jp {}", addr),
            Instruction::JP_COND(ref cond, ref addr) => write!(f, "jp {},{}", cond, addr),
            Instruction::JR(ref addr) => write!(f, "jr {}", addr),
            Instruction::LD8(ref d, ref s) => write!(f, "ld {},{}", d, s),
            Instruction::LD16(ref d, ref s) => write!(f, "ld {},{}", d, s),
            Instruction::LDIR => write!(f, "ldir"),
            Instruction::NOP => write!(f, "                         nop"),
            Instruction::OR(ref val) => write!(f, "or {}", val),
            Instruction::OUT(ref port, ref val) => write!(f, "out ({}),{}", port, val),
            Instruction::POP(ref val) => write!(f, "pop {}", val),
            Instruction::PUSH(ref val) => write!(f, "push {}", val),
            Instruction::RET => write!(f, "ret"),
            Instruction::RST(byte) => write!(f, "rst {:02x}", byte),
            Instruction::RLCA => write!(f, "rlca"),
            _ => write!(f, ""),
        }
    }
}
