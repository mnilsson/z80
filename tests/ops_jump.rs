

#[cfg(test)]
mod test_z80 {
    //    use z80::registers::Reg16;
    use z80::cpu::Z80;
    use z80::bus::Bus;
    use z80::cpu::Source;
    use z80::cpu::Dest;


    struct TestBus {
        memory: Vec<u8>,
        pub m_cycles: u8,
        pub t_states: u8,
    }

    impl TestBus {
        fn new(prg: Vec<u8>) -> TestBus {
            TestBus {
                memory: prg,
                m_cycles: 0,
                t_states: 0,
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
        fn port_write(&mut self, port: u8, byte: u8) {}

        #[allow(unused_variables)]
        fn port_read(&mut self, port: u8) -> u8 {
            0xff
        }

        fn tick(&mut self, machine_cycles: u8, t_states: u8) {
            self.m_cycles += machine_cycles;
            self.t_states += t_states;
            println!("tick {} {}", machine_cycles, t_states);
        }
    }

    fn new_cpu(mut prg: Vec<u8>) -> (Z80, TestBus) {

        prg.resize(0x4000, 0);
        let bus = TestBus::new(
            prg
        );
        (Z80::new(), bus)
    }

    #[test]
    fn test_jp_hl() {
        let (mut cpu, mut bus) = new_cpu(vec![
            0xe9
        ]);

        cpu.registers.l = 0xff;
        cpu.step(&mut bus, 0);
        assert_eq!(0xff, cpu.pc);
        assert_eq!(1, bus.m_cycles);
        assert_eq!(4, bus.t_states);
    }
    #[test]
    fn test_jp_nn() {
        let (mut cpu, mut bus) = new_cpu(vec![
            0xc3, 0xff, 0x00
        ]);


        cpu.step(&mut bus, 0);
        assert_eq!(0xff, cpu.pc);
        assert_eq!(3, bus.m_cycles);
        assert_eq!(10, bus.t_states);
    }

    #[test]
    fn call_returns_to_correct_place() {
        let (mut cpu, mut bus) = new_cpu(vec![
            0xcd, 0xff, 0x00
        ]);

        cpu.sp = 0x2000;
        bus.memory_write_word(0x00ff, 0xc9);
        cpu.step(&mut bus, 0);
        assert_eq!(0xff, cpu.pc);
        assert_eq!(5, bus.m_cycles);
        assert_eq!(17, bus.t_states); 
        cpu.step(&mut bus, 0);
        assert_eq!(0x03, cpu.pc);
        assert_eq!(0x2000, cpu.sp);
    }


    #[test]
    fn test_rst() {
        let (mut cpu, mut bus) = new_cpu(vec![
            0xff,
        ]);

        cpu.sp = 0x2000;

        cpu.step(&mut bus, 0);
        assert_eq!(0x38, cpu.pc);
        // assert_eq!(3, bus.m_cycles);
        // assert_eq!(11, bus.t_states); 
        let val = bus.memory_read_word(0x1ffe);
        assert_eq!(0x01, val);
    }
}