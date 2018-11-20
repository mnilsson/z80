extern crate z80;

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

    #[test]
    fn test_ld_r_r() {
        let (mut cpu, mut bus) = new_cpu(vec![
            0x41
        ]);

        cpu.registers.c = 0x10;
        cpu.step(&mut bus, 0);
        assert_eq!(0x10, cpu.registers.b);
        assert_eq!(1, bus.m_cycles);
        assert_eq!(4, bus.t_states);

    }

    #[test]
    fn test_ld_r_memhl() {
        let (mut cpu, mut bus) = new_cpu(vec![
            0x4e
        ]);

        bus.memory_write(0xff, 0x20);
        cpu.registers.l = 0xff;
        cpu.registers.c = 0x10;
        cpu.step(&mut bus, 0);
        assert_eq!(0x20, cpu.registers.c);
        assert_eq!(2, bus.m_cycles);
        assert_eq!(7, bus.t_states);

    }

    #[test]
    fn test_ld_memhl_r() {
        let (mut cpu, mut bus) = new_cpu(vec![
            0x70
        ]);

        bus.memory_write(0xff, 0x20);
        cpu.registers.l = 0xff;
        cpu.registers.b = 0x15;

        bus.memory_read(0xff);
        cpu.step(&mut bus, 0);
        assert_eq!(0x15, bus.memory_read(0xff));
        assert_eq!(7, bus.t_states);
        assert_eq!(2, bus.m_cycles);

    }

    #[test]
    fn test_ld_r_mem_immword() {
        let (mut cpu, mut bus) = new_cpu(vec![
            0x3a,0x11, 0x00
        ]);



        bus.memory_write(0x0011, 0x1);
        cpu.registers.a = 0;
        

        cpu.step(&mut bus, 0);
        assert_eq!(0x1, cpu.registers.a);
        // assert_eq!(7, bus.t_states);
        // assert_eq!(2, bus.m_cycles);

    }


    #[test]
    fn test_ld_r_immbyte() {
        let (mut cpu, mut bus) = new_cpu(vec![
            0x06, 0x11
        ]);



        cpu.registers.b = 0x06;


        cpu.step(&mut bus, 0);
        assert_eq!(0x11, cpu.registers.b);
        assert_eq!(2, bus.m_cycles);
        assert_eq!(7, bus.t_states);
    }

    #[test]
    fn test_ld_memhl_immbyte() {
        let (mut cpu, mut bus) = new_cpu(vec![
            0x36, 0x11
        ]);

        cpu.registers.l = 0xff;




        cpu.step(&mut bus, 0);
        assert_eq!(0x11, bus.memory_read(0xff));
        assert_eq!(3, bus.m_cycles);
        assert_eq!(10, bus.t_states);
    }

    #[test]
    fn test_ld_rr_immword() {
        let (mut cpu, mut bus) = new_cpu(vec![
            0x11, 0x12, 0x34
        ]);


        cpu.registers.d = 0x1;
        cpu.registers.e = 0x2;

        cpu.step(&mut bus, 0);
        assert_eq!(0x34, cpu.registers.d);
        assert_eq!(0x12, cpu.registers.e);

        // according to z80cpu_um.pdf m_cycles should be 2
        // this is inconsistent with other imm u16 reads. (z80-mostek.pdf says 3)
        assert_eq!(3, bus.m_cycles);
        assert_eq!(10, bus.t_states);
    }

    #[test]
    fn test_ld_rr_memimmword() {
        let (mut cpu, mut bus) = new_cpu(vec![
            0x2a, 0x34, 0x12
        ]);


        cpu.registers.h = 0x1;
        cpu.registers.l = 0x1;

        bus.memory_write(0x1234, 0x78);
        bus.memory_write(0x1235, 0x56);
        cpu.step(&mut bus, 0);
        assert_eq!(0x78, cpu.registers.l);
        assert_eq!(0x56, cpu.registers.h);
        assert_eq!(16, bus.t_states);
        assert_eq!(5, bus.m_cycles);
    }

    #[test]
    fn test_ld_memimmword_rr() {
        let (mut cpu, mut bus) = new_cpu(vec![
            0x22, 0x34, 0x12
        ]);


        cpu.registers.h = 0x1;
        cpu.registers.l = 0x1;

        bus.memory_write(0x1234, 0x78);
        bus.memory_write(0x1235, 0x56);
        cpu.step(&mut bus, 0);

        assert_eq!(0x1, bus.memory_read(0x1234));
        assert_eq!(0x1, bus.memory_read(0x1235));
        assert_eq!(16, bus.t_states);
        assert_eq!(5, bus.m_cycles);
    }

    fn new_cpu(mut prg: Vec<u8>) -> (Z80, TestBus) {

        prg.resize(0x4000, 0);
        let bus = TestBus::new(
            prg
        );
        (Z80::new(), bus)
    }
}