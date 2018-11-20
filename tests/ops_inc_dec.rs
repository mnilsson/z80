

#[cfg(test)]
mod test_z80 {
    use z80::cpu::Z80;
    use z80::bus::Bus;


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
    fn test_inc8() {
        let (mut cpu, mut bus) = new_cpu(vec![
            0x04
        ]);

        cpu.registers.b = 0x10;
        cpu.step(&mut bus, 0);
        assert_eq!(0x11, cpu.registers.b);
        assert_eq!(1, bus.m_cycles);
        assert_eq!(4, bus.t_states);
    }

    #[test]
    fn test_inc16() {
        let (mut cpu, mut bus) = new_cpu(vec![
            0x03
        ]);

        cpu.registers.b = 0x14;
        cpu.registers.c = 0x7;
        cpu.step(&mut bus, 0);
        assert_eq!(0x8, cpu.registers.c);
        assert_eq!(1, bus.m_cycles);
        assert_eq!(6, bus.t_states);
    }
}