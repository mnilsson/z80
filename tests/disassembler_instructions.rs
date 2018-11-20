

#[cfg(test)]
mod test_disassembler_instructions {
    use z80::disassembler::instruction::*;
    

    use z80::registers::Reg8;

    #[test]
    fn test_output() {
        let instr = Instruction::ADD8(Arg8::Register(Reg8::A), Arg8::Register(Reg8::B));
        assert_eq!("add a,b", format!("{}", instr));
    }
}