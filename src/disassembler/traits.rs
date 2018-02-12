use disassembler::instruction::{Address, Data8, Data16, Cond, Arg8, Arg16};
use disassembler::Disassembler;

use registers::{Reg8, Reg16};
use cpu::{Not, ImmByte, ImmWord, Mem, RelOffset};
use flags::Flag;

pub trait IntoArg8 {
    fn into_arg8(self, disassembler: &Disassembler) -> Arg8;
}

pub trait IntoAddress {
    fn into_address(self, disassembler: &Disassembler) -> Address;
}

pub trait IntoArg16 {
    fn into_arg16(self, disassembler: &Disassembler) -> Arg16;
}

pub trait IntoCond {
    fn into_cond(self, disassembler: &Disassembler) -> Cond;
}

impl IntoArg8 for Reg8 {
    fn into_arg8(self, _disassembler: & Disassembler) -> Arg8 {
        Arg8::Register(self)
    }
}

impl IntoArg8 for ImmByte {
     fn into_arg8(self, disassembler: & Disassembler) -> Arg8 {
        Arg8::Immediate(Data8(disassembler.next_byte()))
    }
}

impl IntoArg8 for Mem<Reg16> {
    fn into_arg8(self, disassembler: &Disassembler) -> Arg8 {
        let Mem(reg) = self;
        Arg8::Memory(reg.into_address(disassembler))
    }
}



impl IntoArg8 for Mem<u16> {
    fn into_arg8(self, _disassembler: &Disassembler) -> Arg8 {
        let Mem(addr) = self;
        Arg8::Memory(Address::Direct(Data16(addr)))
    }
}

impl IntoArg8 for Mem<ImmWord> {
    fn into_arg8(self, disassembler: &Disassembler) -> Arg8 {
        let addr = disassembler.next_word();
        Arg8::Memory(Address::Direct(Data16(addr)))
    }
}

impl IntoArg8 for Mem<RelOffset<Reg16>> {
    fn into_arg8(self, _disassembler: &Disassembler) -> Arg8 {
        let address = Address::BC; // @todo not correct
        Arg8::Memory(address)
    }
}

impl IntoArg8 for Mem<RelOffset<u16>> {
    fn into_arg8(self, disassembler: &Disassembler) -> Arg8 {
        let Mem(RelOffset(addr)) = self;
        let offset = disassembler.next_byte();
        let address = Address::RelOffset(Data16(addr), Data8(offset)); // @todo not correct
        Arg8::Memory(address)
    }
}


impl IntoAddress for Reg16 {
    fn into_address(self, _disassembler: &Disassembler) -> Address {
        match self {
            Reg16::BC => Address::BC,
            Reg16::DE => Address::DE,
            Reg16::HL => Address::HL,
            _ => panic!("invalid address register: {:?}", self)
        }
    }
}

impl IntoAddress for u16 {
    fn into_address(self, _disassembler: &Disassembler) -> Address {
        Address::Direct(Data16(self))
    }
}

impl IntoAddress for ImmWord {
    fn into_address(self, disassembler: &Disassembler) -> Address {
        Address::Direct(Data16(disassembler.next_word()))
        
    }
}


impl IntoAddress for Mem<ImmWord> {
    fn into_address(self, _disassembler: &Disassembler) -> Address {
        panic!("should not be called, i think")
        // Address::Direct(Data16(disassembler.next_word()))
    }
}

impl IntoAddress for Mem<Reg16> {
    fn into_address(self, _disassembler: &Disassembler) -> Address {
        panic!("should not be called, i think")
        // let Mem(reg) = self;
        // reg.into_address(disassembler)
    }
}

impl IntoAddress for Mem<RelOffset<Reg16>> {
    fn into_address(self, _disassembler: &Disassembler) -> Address {
        panic!("should not be called, i think")
        // let Mem(RelOffset(reg)) = self;
        // let offset = disassembler.next_byte();
        // let address = Address::RelOffset(Data16(addr), Data8(offset)); // @todo not correct
        // Arg8::Memory(address)
    }
}

impl IntoAddress for RelOffset<Reg16> {
    fn into_address(self, disassembler: &Disassembler) -> Address {
        let RelOffset(_reg) = self;
        let offset = disassembler.next_byte();
        Address::RelOffset(Data16(1), Data8(offset)) // @todo not correct, should show reg
    }
}



impl IntoAddress for RelOffset<u16> {
    fn into_address(self, disassembler: &Disassembler) -> Address {
        let RelOffset(addr) = self;
        let offset = disassembler.next_byte();
        Address::RelOffset(Data16(addr), Data8(offset)) // @todo not correct, should show reg
    }
}


impl IntoArg16 for u16 {
    fn into_arg16(self, _disassembler: &Disassembler) -> Arg16 {
        // should probably not be called
        Arg16::Immediate(Data16(self))
    }
}


impl IntoArg16 for Reg16 {
    fn into_arg16(self, _disassembler: &Disassembler) -> Arg16 {
        Arg16::Register(self)
    }
}

impl IntoArg16 for ImmWord {
    fn into_arg16(self, disassembler: &Disassembler) -> Arg16 {
        Arg16::Immediate(Data16(disassembler.next_word()))
    }
}

impl IntoArg16 for Mem<ImmWord> {
    fn into_arg16(self, disassembler: &Disassembler) -> Arg16 {
        Arg16::Memory(Address::Direct(Data16(disassembler.next_word())))
    }
}

impl IntoArg16 for Mem<Reg16> {
    fn into_arg16(self, disassembler: &Disassembler) -> Arg16 {
        let Mem(reg) = self;
        Arg16::Memory(reg.into_address(disassembler))
    }
}

impl IntoArg16 for RelOffset<Reg16> {
    fn into_arg16(self, _disassembler: &Disassembler) -> Arg16 {
        panic!("nope")
    }
}

impl IntoArg16 for RelOffset<u16> {
    fn into_arg16(self, _disassembler: &Disassembler) -> Arg16 {
        panic!("nope")
    }
}


impl IntoArg8 for u8 {
    fn into_arg8(self, _disassembler: &Disassembler) -> Arg8 {
        Arg8::Immediate(Data8(self))
    }
}

impl IntoCond for bool {
    fn into_cond(self, _disassembler: &Disassembler) -> Cond {

        match self {
            true => Cond::True,
            false => Cond::False,
        }
    }
}

impl IntoCond for Flag {
    fn into_cond(self, _disassembler: &Disassembler) -> Cond {
        match self {
            Flag::Carry => Cond::Carry,
            Flag::Zero => Cond::Zero,
            Flag::Subtract => Cond::Negative,
            _ => unreachable!("invalid cond"),
        }
    }
}

impl IntoCond for Not<Flag> {
    fn into_cond(self, _disassembler: &Disassembler) -> Cond {
        let Not(flag) = self; 
        match flag {
            Flag::Carry => Cond::NotCarry,
            Flag::Zero => Cond::NotZero,
            Flag::Subtract => Cond::Positive,
            _ => unreachable!("invalid cond"),
        }
    }
}
 