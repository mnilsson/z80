extern crate z80;

#[cfg(test)]
mod test_z80 {

    use z80::cpu::Z80;
    use z80::bus::Bus;
    use z80::cpu::*;
    use z80::registers::Reg16;


    struct TestBus {
        memory: Vec<u8>,
        pub m_cycles: u8,
        pub t_states: u8,
        pub ticks: u8,
    }

    impl TestBus {
        fn new(prg: Vec<u8>) -> TestBus {
            TestBus {
                memory: prg,
                m_cycles: 0,
                t_states: 0,
                ticks: 0,
            }
        }
    }

    impl Bus for TestBus {
        fn memory_read(&self, address: usize) -> u8 {
            self.memory[address]
        }

        fn memory_read_word(&self, address: usize) -> u16 {
            self.memory[address] as u16 | ((self.memory[address + 1] as u16) << 8)
        }

        fn memory_write(&mut self, address: usize, value: u8) {
            self.memory[address] = value;
        }

        fn memory_write_word(&mut self, address: usize, value: u16) {
            self.memory[address] = value as u8;
            self.memory[address + 1] = (value >> 8) as u8;
        }

        #[allow(unused_variables)]
        fn port_write(&mut self, port: u8, byte: u8) {

        }

        #[allow(unused_variables)]
        fn port_read(&mut self, port: u8) -> u8 {
            0xff
        }

        fn tick(&mut self, machine_cycles: u8, t_states: u8) {
            self.m_cycles += machine_cycles;
            self.t_states += t_states;
        }
    }

    fn new_cpu(mut prg: Vec<u8>) -> (Z80, TestBus) {

        prg.resize(0xff00, 0);
        let bus = TestBus::new(
            prg
        );
        (Z80::new(), bus)
    }
   const CARRY: u8 =       0b0000_0001;//1 << 0;
   const SUBTRACT: u8 =    0b0000_0010;//1 << 1;
   const PARITY: u8 =      0b0000_0100;//1 << 2;
   const HALFCARRY: u8 =   0b0001_0000;//1 << 4;
   const ZERO: u8 =        0b0100_0000;//1 << 6;
   const SIGN: u8 =        0b1000_0000;//1 << 7;

    const XYMASK: u8 = 0b1101_0111;
   fn new_bus_and_cpu_with_prg(prg: Vec<u8>) -> (Z80, TestBus) {
       new_cpu(prg)
   }

   #[test]
   fn test_mem_ix_n_n() {

       let (mut cpu, mut bus) = new_bus_and_cpu_with_prg(
           vec![
               0xdd,
               0x36,
               0x05,
               0x5a,
           ]
       );

       cpu.registers.ix = 0x219a;
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x5a, bus.memory_read(0x219f));

   }
  
   #[test]
   fn test_mem_iy_n_n() {

       let (mut cpu, mut bus) = new_bus_and_cpu_with_prg(
           vec![
               0xfd,
               0x36,
               0x10,
               0x97,
           ]
       );

       cpu.registers.iy = 0xa940;
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x97, bus.memory_read(0xa950));

   }

   #[test]
   fn test_cpl() {

       let (mut cpu, mut bus) = new_bus_and_cpu_with_prg(
           vec![
               0x2f
           ]
       );

       cpu.registers.a = 0b1011_0100;
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0b0100_1011, cpu.registers.a);

   }

   #[test]
   fn test_scf() {
       let (mut cpu, mut bus) = new_bus_and_cpu_with_prg(
           vec![
               0x37
           ]
       );
       cpu.registers.f = SIGN | ZERO | HALFCARRY | PARITY | SUBTRACT;
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(
           SIGN | ZERO | PARITY | CARRY,
           cpu.registers.f
       );
   }

   #[test]
   fn test_adc_hl_ss() {
       let (mut cpu, mut bus) = new_bus_and_cpu_with_prg(
           vec![
               0x21, 0xFC, 0x00,       // LD HL,0x00FC
               0x01, 0x08, 0x00,       // LD BC,0x0008
               0x11, 0xFF, 0xFF,       // LD DE,0xFFFF
               0x09,                   // ADD HL,BC
               0x19,                   // ADD HL,DE
               0xED, 0x4A,             // ADC HL,BC
               0x29,                   // ADD HL,HL
               0x19,                   // ADD HL,DE
               0xED, 0x42,             // SBD HL,BC
               0xDD, 0x21, 0xFC, 0x00, // LD IX,0x00FC
               0x31, 0x00, 0x10,       // LD SP,0x1000
               0xDD, 0x09,             // ADD IX, BC
               0xDD, 0x19,             // ADD IX, DE
               0xDD, 0x29,             // ADD IX, IX
               0xDD, 0x39,             // ADD IX, SP
               0xFD, 0x21, 0xFF, 0xFF, // LD IY,0xFFFF
               0xFD, 0x09,             // ADD IY,BC
               0xFD, 0x19,             // ADD IY,DE
               0xFD, 0x29,             // ADD IY,IY
               0xFD, 0x39,             // ADD IY,SP
           ]
       );



       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x00fc, Reg16::HL.read16(&mut cpu, &mut bus));
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x0008, Reg16::BC.read16(&mut cpu, &mut bus));
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0xffff, Reg16::DE.read16(&mut cpu, &mut bus));

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x0104, Reg16::HL.read16(&mut cpu, &mut bus));
       assert_eq!(0, cpu.registers.f & XYMASK);

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x0103, Reg16::HL.read16(&mut cpu, &mut bus));
       assert_eq!(HALFCARRY | CARRY, cpu.registers.f & XYMASK);

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x010c, Reg16::HL.read16(&mut cpu, &mut bus));
       assert_eq!(0, cpu.registers.f & XYMASK);

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x0218, Reg16::HL.read16(&mut cpu, &mut bus));
       assert_eq!(0, cpu.registers.f & XYMASK);

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x217, Reg16::HL.read16(&mut cpu, &mut bus));
       assert_eq!(HALFCARRY | CARRY, cpu.registers.f & XYMASK);

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x20e, Reg16::HL.read16(&mut cpu, &mut bus));
       assert_eq!(SUBTRACT, cpu.registers.f & XYMASK);

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x20e, Reg16::HL.read16(&mut cpu, &mut bus));
   }

   #[test]
   fn test_sbc_hl_ss() {
       let (mut cpu, mut bus) = new_bus_and_cpu_with_prg(
           vec![
               0xed, 0x52, // sbc hl, ed
           ]
       );
       cpu.registers.e = 0x01;
       cpu.registers.d = 0x00;
       cpu.registers.h = 0x40;
       cpu.registers.l = 0x00;

       cpu.registers.f = SIGN | ZERO | HALFCARRY | PARITY | SUBTRACT | CARRY;
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(
           0x3ffe,
           Reg16::HL.read16(&mut cpu, &mut bus)
       );

        let (mut cpu, mut bus) = new_bus_and_cpu_with_prg(
           vec![
               0xed, 0x52, // sbc hl, ed
           ]
       );
       cpu.registers.e = 0x11;
       cpu.registers.d = 0x11;
       cpu.registers.h = 0x99;
       cpu.registers.l = 0x99;

       cpu.registers.f = SIGN | ZERO | HALFCARRY | PARITY | SUBTRACT | CARRY;
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(
           0x8887,
           Reg16::HL.read16(&mut cpu, &mut bus)
       );

   }

    #[test]
    fn test_add_adc_sbc_16() {
        let (mut cpu, mut bus) = new_bus_and_cpu_with_prg(vec![
            0x21, 0xFC, 0x00,       // LD HL,0x00FC
            0x01, 0x08, 0x00,       // LD BC,0x0008
            0x11, 0xFF, 0xFF,       // LD DE,0xFFFF
            0x09,                   // ADD HL,BC
            0x19,                   // ADD HL,DE
            0xED, 0x4A,             // ADC HL,BC
            0x29,                   // ADD HL,HL
            0x19,                   // ADD HL,DE
            0xED, 0x42,             // SBD HL,BC
            0xDD, 0x21, 0xFC, 0x00, // LD IX,0x00FC
            0x31, 0x00, 0x10,       // LD SP,0x1000
            0xDD, 0x09,             // ADD IX, BC
            0xDD, 0x19,             // ADD IX, DE
            0xDD, 0x29,             // ADD IX, IX
            0xDD, 0x39,             // ADD IX, SP
            0xFD, 0x21, 0xFF, 0xFF, // LD IY,0xFFFF
            0xFD, 0x09,             // ADD IY,BC
            0xFD, 0x19,             // ADD IY,DE
            0xFD, 0x29,             // ADD IY,IY
            0xFD, 0x39,             // ADD IY,SP
        ]);
        for _ in 0..9 {
            cpu.execute_next_instruction(&mut bus);
        }
        assert_eq!(
           0x020e,
           Reg16::HL.read16(&mut cpu, &mut bus)
       );

        // assert_eq!(10, cpu.step(bus)); assert_eq!(0x00FC, cpu.reg.hl());
        // assert_eq!(10, cpu.step(bus)); assert_eq!(0x0008, cpu.reg.bc());
        // assert_eq!(10, cpu.step(bus)); assert_eq!(0xFFFF, cpu.reg.de());
        // assert_eq!(11, cpu.step(bus)); assert_eq!(0x0104, cpu.reg.hl()); assert!(flags(&cpu, 0));
        // assert_eq!(11, cpu.step(bus)); assert_eq!(0x0103, cpu.reg.hl()); assert!(flags(&cpu, HF|CF));
        // assert_eq!(15, cpu.step(bus)); assert_eq!(0x010C, cpu.reg.hl()); assert!(flags(&cpu, 0));
        // assert_eq!(11, cpu.step(bus)); assert_eq!(0x0218, cpu.reg.hl()); assert!(flags(&cpu, 0));
        // assert_eq!(11, cpu.step(bus)); assert_eq!(0x0217, cpu.reg.hl()); assert!(flags(&cpu, HF|CF));
        // assert_eq!(15, cpu.step(bus)); assert_eq!(0x020E, cpu.reg.hl()); assert!(flags(&cpu, NF));
    }

   #[test]
   fn test_daa() {
       let (mut cpu, mut bus) = new_bus_and_cpu_with_prg(
           vec![
               0x3E, 0x15,     // LD A,0x15
               0x06, 0x27,     // LD B,0x27
               0x80,           // ADD A,B
               0x27,           // DAA
               0x90,           // SUB B
               0x27,           // DAA
//                0x3E, 0x90,     // LD A,0x90
//                0x06, 0x15,     // LD B,0x15
               0x80,           // ADD A,B
               0x27,           // DAA
               0x90,           // SUB B
               0x27,           // DAA
           ]
       );

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x15, cpu.registers.a);
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x27, cpu.registers.b);
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x3c, cpu.registers.a);
       assert_eq!(0x0, cpu.registers.f & XYMASK);
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x42, cpu.registers.a);
       assert_eq!(HALFCARRY | PARITY, cpu.registers.f & XYMASK);
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x1b, cpu.registers.a);
       assert_eq!(HALFCARRY | SUBTRACT, cpu.registers.f & XYMASK);
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x15, cpu.registers.a);
       assert_eq!(SUBTRACT, cpu.registers.f & XYMASK);

       cpu.registers.a = 0x90;
       cpu.registers.b = 0x15;

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0xa5, cpu.registers.a);
       assert_eq!(SIGN, cpu.registers.f & XYMASK);

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x05, cpu.registers.a);
       assert_eq!(PARITY | CARRY, cpu.registers.f & XYMASK);

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0xf0, cpu.registers.a);
       assert_eq!(SIGN | SUBTRACT | CARRY, cpu.registers.f & XYMASK);

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x90, cpu.registers.a);
       assert_eq!(SIGN | PARITY | SUBTRACT | CARRY, cpu.registers.f & XYMASK);
   }
   #[test]
   fn test_push_pop() {
       let (mut cpu, mut bus) = new_bus_and_cpu_with_prg(
           vec![
               0xf5,           // push af
               0xc5,
               0xc1,
               0xf1,           // pop af
               
           ]
       );

       cpu.registers.a = 0x04;
       cpu.registers.f = 0x08;

       cpu.execute_next_instruction(&mut bus);
       cpu.registers.a = 0x0;
       cpu.registers.f = 0x0;
       cpu.execute_next_instruction(&mut bus);
       cpu.execute_next_instruction(&mut bus);

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x08, cpu.registers.f);
       assert_eq!(0x04, cpu.registers.a);
   }

   #[test]
   fn test_8b_add() {
       let (mut cpu, mut bus) = new_bus_and_cpu_with_prg(
           vec![
               0x81, // ADD A, C
               0xC6, 0x33, // ADD A, $33
               0x86, // ADD A, (HL)
               0xdd, 0x86, 0x05, //ADD A, (IX + $5)

//                0x3E, 0x0F,     // LD A,0x0F
               0x87,           // ADD A,A
//                0x06, 0xE0,     // LD B,0xE0
               0x80,           // ADD A,B
//                0x3E, 0x81,     // LD A,0x81
//                0x0E, 0x80,     // LD C,0x80
               0x81,           // ADD A,C
//                0x16, 0xFF,     // LD D,0xFF
               0x82,           // ADD A,D
//                0x1E, 0x40,     // LD E,0x40
               0x83,           // ADD A,E
//                0x26, 0x80,     // LD H,0x80
               0x84,           // ADD A,H
//                0x2E, 0x33,     // LD L,0x33
               0x85,           // ADD A,L
               0xC6, 0x44,     // ADD A,0x44
           ]
       );

       // ADD A, r
       cpu.registers.a = 0x44;
       cpu.registers.c = 0x11;
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x55, cpu.registers.a);

       // ADD A, n
       cpu.registers.a = 0x23;
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x56, cpu.registers.a);

       // ADD A, (HL)
       cpu.registers.a = 0xa0;
       Reg16::HL.write16(&mut cpu, &mut bus, 0x2323);
       bus.memory_write(0x2323, 0x08);
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0xa8, cpu.registers.a);

       // ADD A, (IX + $5)
       cpu.registers.a = 0x11;
       cpu.registers.ix = 0x1000;
       bus.memory_write(0x1005, 0x22);
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x33, cpu.registers.a);


       cpu.registers.a = 0x0f;
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x1e, cpu.registers.a);
       assert_eq!(HALFCARRY, cpu.registers.f & XYMASK);

       cpu.registers.b = 0xe0;
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0xfe, cpu.registers.a);
       assert_eq!(SIGN, cpu.registers.f & XYMASK);

       cpu.registers.a = 0x81;
       cpu.registers.c = 0x80;
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x01, cpu.registers.a);
       assert_eq!(PARITY | CARRY, cpu.registers.f & XYMASK);

       cpu.registers.d = 0xff;
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x00, cpu.registers.a);
       assert_eq!(ZERO | HALFCARRY | CARRY, cpu.registers.f & XYMASK);

       cpu.registers.e = 0x40;
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x40, cpu.registers.a);
       assert_eq!(0, cpu.registers.f & XYMASK);

       cpu.registers.h = 0x80;
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0xc0, cpu.registers.a);
       assert_eq!(SIGN, cpu.registers.f & XYMASK);

       cpu.registers.l = 0x33;
       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0xf3, cpu.registers.a);
       assert_eq!(SIGN, cpu.registers.f & XYMASK);

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x37, cpu.registers.a);
       assert_eq!(CARRY, cpu.registers.f & XYMASK);
   }

   #[test]
   fn test_8b_sub_r() {
       let (mut cpu, mut bus) = new_bus_and_cpu_with_prg(
           vec![
               0x97,           // SUB A,A
               0x90,           // SUB A,B
               0x91,           // SUB A,C
               0x92,           // SUB A,D
               0x93,           // SUB A,E
               0x94,           // SUB A,H
               0x95,           // SUB A,L
               0xD6, 0x01,     // SUB A,0x01
               0xD6, 0xFE,     // SUB A,0xFE
           ]
       );

       cpu.registers.a = 0x04;
       cpu.registers.b = 0x01;
       cpu.registers.c = 0xf8;
       cpu.registers.d = 0x0f;
       cpu.registers.e = 0x79;
       cpu.registers.h = 0xc0;
       cpu.registers.l = 0xbf;

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x00, cpu.registers.a);
       assert_eq!(ZERO|SUBTRACT, cpu.registers.f & XYMASK);

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0xff, cpu.registers.a);
       assert_eq!(SIGN|HALFCARRY|SUBTRACT|CARRY, cpu.registers.f & XYMASK);

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x07, cpu.registers.a);
       assert_eq!(SUBTRACT, cpu.registers.f & XYMASK);

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0xf8, cpu.registers.a);
       assert_eq!(SIGN|HALFCARRY|SUBTRACT|CARRY, cpu.registers.f & XYMASK);

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x7f, cpu.registers.a);
       assert_eq!(HALFCARRY|PARITY|SUBTRACT, cpu.registers.f & XYMASK);

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0xbf, cpu.registers.a);
       assert_eq!(SIGN|PARITY|SUBTRACT|CARRY, cpu.registers.f & XYMASK);

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x00, cpu.registers.a);
       assert_eq!(ZERO|SUBTRACT, cpu.registers.f & XYMASK);

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0xff, cpu.registers.a);
       assert_eq!(SIGN|HALFCARRY|SUBTRACT|CARRY, cpu.registers.f & XYMASK);

       cpu.execute_next_instruction(&mut bus);
       assert_eq!(0x01, cpu.registers.a);
       assert_eq!(SUBTRACT, cpu.registers.f & XYMASK);

   }
}