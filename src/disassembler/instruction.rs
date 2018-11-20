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

    True,
    False,
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
            _ => write!(f, ""),
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

use crate::registers::{Reg8, Reg16};

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
    ADC8(Arg8, Arg8),
    ADD16(Arg16, Arg16),
    ADC16(Arg16, Arg16),
    AND(Arg8),
    BIT(u8, Arg8),
    CCF,
    CP(Arg8),
    CPD,
    CPL,
    CPDR,
    CPI,
    CPIR,
    CALL(Address),
    CALL_COND(Cond, Address),
    DAA,
    DEC8(Arg8),
    DEC16(Arg16),
    DI,
    DJNZ(Arg8),
    EI,
    EX(Arg16, Arg16),
    EXX,
    HALT,
    IM,
    IN(Arg8, Arg8),
    INC8(Arg8),
    INC8_MEMORY(Address),
    INC16(Arg16),
    IND,
    INDR,
    INI,
    INIR,
    JP(Address),
    JP_COND(Cond, Address),
    JR(Address),
    JR_COND(Cond, u8),
    LD8(Arg8, Arg8),
    LD16(Arg16, Arg16),
    LDD,
    LDDR,
    LDI,
    LDIR,
    NEG,
    NOP,
    OR(Arg8),
    OTDR,
    OTIR,
    OUT(Arg8, Arg8),
    OUTD,
    OUTI,
    POP(Arg16),
    PUSH(Arg16),
    RES(u8, Arg8),
    RET,
    RETI,
    RETN,
    RET_COND(Cond),
    RL(Arg8),
    RLA,
    RLC(Arg8),
    RLCA,
    RLD,
    RR(Arg8),
    RRA,
    RRC(Arg8),
    RRCA,
    RRD,
    RST(u8),
    SBC8(Arg8),
    SBC16(Arg16, Arg16),
    SCF,
    SET(u8, Arg8),
    SRA(Arg8),
    SLA(Arg8),
    SLL(Arg8),
    SRL(Arg8),
    SUB8(Arg8),
    XOR(Arg8),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::ADD8(ref d, ref s) => write!(f, "add {},{}", d, s),
            Instruction::ADC8(ref d, ref s) => write!(f, "adc {},{}", d, s),
            Instruction::ADD16(ref d, ref s) => write!(f, "add {},{}", d, s),
            Instruction::ADC16(ref d, ref s) => write!(f, "adc {},{}", d, s),
            Instruction::AND(ref val) => write!(f, "and {}", val),
            Instruction::BIT(ref b, ref r) => write!(f, "bit {},{}", b, r),
            Instruction::CP(ref val) => write!(f, "cp {}", val),
            Instruction::CALL(ref addr) => write!(f, "call {}", addr),
            Instruction::CALL_COND(ref cond, ref addr) => write!(f, "call {},{}", cond, addr),
            
            Instruction::DEC8(ref reg) => write!(f, "dec {}", reg),
            Instruction::DEC16(ref reg) => write!(f, "dec {}", reg),

            Instruction::DJNZ(ref val) => write!(f, "djnz {}", val),
            Instruction::EX(ref a, ref b) => write!(f, "ex {},{}", a, b),
            Instruction::IN(ref d, ref s) => write!(f, "in {},{}", d, s),
            Instruction::INC8(ref reg) => write!(f, "inc {}", reg),
            Instruction::INC8_MEMORY(ref addr) => write!(f, "inc {}", addr),
            Instruction::INC16(ref reg) => write!(f, "inc {}", reg),
            Instruction::JP(ref addr) => write!(f, "jp {}", addr),
            Instruction::JP_COND(ref cond, ref addr) => write!(f, "jp {},{}", cond, addr),
            Instruction::JR(ref addr) => write!(f, "jr {}", addr),
            Instruction::JR_COND(ref cond, addr) => write!(f, "jr {},{}", cond, addr as i8),
            Instruction::LD8(ref d, ref s) => write!(f, "ld {},{}", d, s),
            Instruction::LD16(ref d, ref s) => write!(f, "ld {},{}", d, s),
            Instruction::OR(ref val) => write!(f, "or {}", val),
            Instruction::OUT(ref port, ref val) => write!(f, "out ({}),{}", port, val),
            Instruction::POP(ref val) => write!(f, "pop {}", val),
            Instruction::PUSH(ref val) => write!(f, "push {}", val),
            Instruction::RES(ref b, ref r) => write!(f, "res {},{}", b, r),
            Instruction::RET => write!(f, "ret"),
            Instruction::RET_COND(ref cond) => write!(f, "ret {}", cond),
            Instruction::RL(ref reg) => write!(f, "rl {}", reg),
            Instruction::RLC(ref reg) => write!(f, "rlc {}", reg),
            Instruction::RR(ref reg) => write!(f, "rr {}", reg),
            Instruction::RRC(ref reg) => write!(f, "rrc {}", reg),
            Instruction::RST(ref byte) => write!(f, "rst {:02x}", byte),
            Instruction::SBC8(ref s) => write!(f, "sbc a,{}", s),
            Instruction::SBC16(ref d, ref s) => write!(f, "sbc {},{}", d, s),
            Instruction::SET(ref b, ref r) => write!(f, "set {},{}", b, r),
            Instruction::SLA(ref reg) => write!(f, "sla {}", reg),
            Instruction::SLL(ref reg) => write!(f, "sll {}", reg),
            Instruction::SRA(ref reg) => write!(f, "sra {}", reg),
            Instruction::SRL(ref reg) => write!(f, "srl {}", reg),
            Instruction::SUB8(ref reg) => write!(f, "sub {}", reg),
            Instruction::XOR(ref reg) => write!(f, "xor {}", reg),
            _ => write!(f, "{}", format!("{:?}", *self).to_lowercase()),
        }
    }
}


#[cfg(test)]
mod test {
    use crate::disassembler::instruction::Instruction;
    #[test]
    fn test_output_correct() {
        let out = format!("{}", Instruction::RRD);
        assert_eq!("rrd", out);
    }
}