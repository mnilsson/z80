use registers::Registers;
pub enum Flag {
    Sign,
    Zero,
    HalfCarry,
    Parity,
    Overflow,
    Subtract,
    Carry,
    X,
    Y,
}

impl Flag {
    pub fn read(self, registers: &Registers) -> bool{
        match self {
            Flag::Sign => registers.f & 0b1000_0000 != 0,
            Flag::Zero => registers.f & 0b0100_0000 != 0,
            Flag::Y => registers.f & 0b0010_0000 != 0,
            Flag::HalfCarry => registers.f & 0b0001_0000 != 0,
            Flag::X => registers.f & 0b0000_1000 != 0,
            Flag::Parity | Flag::Overflow => registers.f & 0b0000_0100 != 0,
            Flag::Subtract => registers.f & 0b0000_0010 != 0,
            Flag::Carry => registers.f & 0b0000_0001 != 0,
        }
    }

    pub fn write(self, registers: &mut Registers, val: bool) {
        let flags = registers.f;
        match self {
            Flag::Sign => registers.f = if val { set_bit(flags, 7) } else { reset_bit(flags, 7)},
            Flag::Zero => registers.f = if val { set_bit(flags, 6) } else { reset_bit(flags, 6)},
            Flag::Y => registers.f = if val { set_bit(flags, 5) } else { reset_bit(flags, 5)},
            Flag::HalfCarry => registers.f = if val { set_bit(flags, 4) } else { reset_bit(flags, 4)},
            Flag::X => registers.f = if val { set_bit(flags, 3) } else { reset_bit(flags, 3)},
            Flag::Parity | Flag::Overflow => registers.f = if val { set_bit(flags, 2) } else { reset_bit(flags, 2)},
            Flag::Subtract => registers.f = if val { set_bit(flags, 1) } else { reset_bit(flags, 1)},
            Flag::Carry => registers.f = if val { set_bit(flags, 0) } else { reset_bit(flags, 0)},
        }
    }
}

fn set_bit(val: u8, bit: u8) -> u8 {
    val | (1 << bit)
}

fn reset_bit(val: u8, bit: u8) -> u8 {
    val & !(1 << bit)
}

pub trait WriteFlag {
    fn write_flag(self, flag: Flag, val: bool);
}

pub trait ReadFlag {
    fn read_flag(self, flag: Flag) -> bool;
}


#[cfg(test)]
mod tests {
//    use super::*;
//    use interconnect::Interconnect;
//    use rom::Rom;
//    use z80::registers::Reg8::*;
//
//    fn reset_cpu() -> Z80 {
//        let interconnect = Interconnect::new(Rom::new(vec![0; (1 << 16)]));
//        let cpu = Z80::new(interconnect);
//        cpu
//    }

    #[test]
    fn test_set_bit() {
        let res = super::set_bit(0, 0); assert_eq!(0b0000_0001, res);
        let res = super::set_bit(0, 1); assert_eq!(0b0000_0010, res);
        let res = super::set_bit(0, 2); assert_eq!(0b0000_0100, res);
        let res = super::set_bit(0, 3); assert_eq!(0b0000_1000, res);
        let res = super::set_bit(0, 4); assert_eq!(0b0001_0000, res);
        let res = super::set_bit(0, 5); assert_eq!(0b0010_0000, res);
        let res = super::set_bit(0, 6); assert_eq!(0b0100_0000, res);
        let res = super::set_bit(0, 7); assert_eq!(0b1000_0000, res);
    }

    #[test]
    fn test_reset_bit() {
        let res = super::reset_bit(1, 0); assert_eq!(0b0000_0000, res);

        let res = super::reset_bit(0b1111_1111, 0); assert_eq!(0b1111_1110, res);
        let res = super::reset_bit(0b1111_1111, 1); assert_eq!(0b1111_1101, res);
        let res = super::reset_bit(0b1111_1111, 2); assert_eq!(0b1111_1011, res);
        let res = super::reset_bit(0b1111_1111, 3); assert_eq!(0b1111_0111, res);
        let res = super::reset_bit(0b1111_1111, 4); assert_eq!(0b1110_1111, res);
        let res = super::reset_bit(0b1111_1111, 5); assert_eq!(0b1101_1111, res);
        let res = super::reset_bit(0b1111_1111, 6); assert_eq!(0b1011_1111, res);
        let res = super::reset_bit(0b1111_1111, 7); assert_eq!(0b0111_1111, res);
    }
}
