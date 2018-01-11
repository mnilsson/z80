#![allow(cast_lossless)]
extern crate z80;

#[cfg(test)]
mod test_io {
    //    use z80::registers::Reg16;
    use z80::cpu::Z80;
    use z80::bus::Bus;
    use z80::cpu::Source;
    use z80::cpu::Dest;


    struct TestBus {
        memory: Vec<u8>,
        pub port_data: Vec<u8>,
        pub m_cycles: u8,
        pub t_states: u8,
    }

    impl TestBus {
        fn new(prg: Vec<u8>) -> TestBus {
            TestBus {
                memory: prg,
                m_cycles: 0,
                t_states: 0,
                port_data: vec![0; 0xff],
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

            self.port_data[port as usize] = byte;
        }

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
        let mut bus = TestBus::new(
            prg
        );
        (Z80::new(), bus)
    }

    #[test]
    fn test_out_n_a () {
        let (mut cpu, mut bus) = new_cpu(vec![
            0xd3, 0x01
        ]);

        cpu.registers.a = 0x23;
        cpu.step(&mut bus, 0);
        // assert_eq!(3, bus.m_cycles);
        // assert_eq!(11, bus.t_states);
        assert_eq!(0x23, bus.port_data[0x01]);
    }

    #[test]
    fn test_out_c_r () {
        let (mut cpu, mut bus) = new_cpu(vec![
            0xed, 0x51
        ]);

        cpu.registers.c = 0x01;
        cpu.registers.d = 0x5a;
        
        cpu.step(&mut bus, 0);
        assert_eq!(3, bus.m_cycles);
        assert_eq!(12, bus.t_states);
        assert_eq!(0x5a, bus.port_data[0x01]);
    }

    #[test]
    fn test_outi () {
        let (mut cpu, mut bus) = new_cpu(vec![
            0xed, 0xa3
        ]);

        cpu.registers.b = 0x10;
        cpu.registers.c = 0x07;
        cpu.registers.d = 0x5a;
        cpu.registers.h = 0x10;
        cpu.registers.l = 0x00;

        bus.memory_write(0x1000, 0x59);
        
        cpu.step(&mut bus, 0);
        assert_eq!(0x0f, cpu.registers.b);
        assert_eq!(0x10, cpu.registers.h);
        assert_eq!(0x01, cpu.registers.l);
        assert_eq!(0x59, bus.port_data[0x07]);
        assert_eq!(4, bus.m_cycles);
        assert_eq!(16, bus.t_states);
        
    }
}