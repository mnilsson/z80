

#[cfg(test)]
mod test_z80 {

    use z80::cpu::Z80;
    use z80::bus::Bus;
    use z80::flags::Flag;


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

    #[test]
    fn ld_b_b() {
        let (mut cpu, mut bus) = new_cpu(vec![0x40]);
        cpu.step(&mut bus, 0);
        assert_eq!(1, bus.m_cycles);
        assert_eq!(4, bus.t_states);
    }

    #[test]
    fn ld_a_n() {
        let (mut cpu, mut bus) = new_cpu(vec![0x3e]);
        cpu.step(&mut bus, 0);
        assert_eq!(2, bus.m_cycles);
        assert_eq!(7, bus.t_states);
    }

    #[test]
    fn ld_a_mem_hl() {
        let (mut cpu, mut bus) = new_cpu(vec![0x7e]);
        cpu.step(&mut bus, 0);
        assert_eq!(2, bus.m_cycles);
        assert_eq!(7, bus.t_states);
    }

    #[test]
    fn ld_mem_hl_b() {
        let (mut cpu, mut bus) = new_cpu(vec![0x70]);
        cpu.step(&mut bus, 0);
        assert_eq!(2, bus.m_cycles);
        assert_eq!(7, bus.t_states);
    }

    #[test]
    fn ld_a_ix_d() {
        let (mut cpu, mut bus) = new_cpu(vec![0xdd, 0x86, 0x05]);
        cpu.registers.a = 0x11;
        cpu.registers.ix = 0x1000;
        bus.memory_write(0x1005, 0x22);
        cpu.step(&mut bus, 0);
        assert_eq!(0x33, cpu.registers.a);
        assert_eq!(5, bus.m_cycles);
        assert_eq!(19, bus.t_states);
    }

    #[test]
    fn test_jr() {
        let (mut cpu, mut bus) = new_cpu(vec![0x18]);
        cpu.step(&mut bus, 0);
        assert_eq!(3, bus.m_cycles);
        assert_eq!(12, bus.t_states);
    }

    #[test]
    fn test_jr_cond_true() {
        let (mut cpu, mut bus) = new_cpu(vec![0x28]);
        cpu.registers.set_flag(Flag::Zero, true);
        cpu.step(&mut bus, 0);
        assert_eq!(3, bus.m_cycles);
        assert_eq!(12, bus.t_states); 
    }

    #[test]
    fn test_jr_cond_false() {
        let (mut cpu, mut bus) = new_cpu(vec![0x28]);
        cpu.registers.set_flag(Flag::Zero, false);
        cpu.step(&mut bus, 0);
        assert_eq!(2, bus.m_cycles);
        assert_eq!(7, bus.t_states); 
    }

     #[test]
    fn test_ret_cond_true() {
        let (mut cpu, mut bus) = new_cpu(vec![0xc8]);
        cpu.registers.set_flag(Flag::Zero, true);
        cpu.step(&mut bus, 0);
        assert_eq!(3, bus.m_cycles);
        assert_eq!(11, bus.t_states); 
    }

    #[test]
    fn test_ret_cond_false() {
        let (mut cpu, mut bus) = new_cpu(vec![0xc8]);
        cpu.registers.set_flag(Flag::Zero, false);
        cpu.step(&mut bus, 0);
        assert_eq!(1, bus.m_cycles);
        assert_eq!(5, bus.t_states); 
    }

    #[test]
    fn test_ret() {
        let (mut cpu, mut bus) = new_cpu(vec![0xc9]);
        cpu.registers.set_flag(Flag::Zero, false);
        cpu.step(&mut bus, 0);
        assert_eq!(3, bus.m_cycles);
        assert_eq!(10, bus.t_states); 
    }


     #[test]
    fn test_nop() {
        let (mut cpu, mut bus) = new_cpu(vec![0x00]);
        cpu.step(&mut bus, 0);
        assert_eq!(1, cpu.pc);
        assert_eq!(1, bus.m_cycles);
        assert_eq!(4, bus.t_states); 
    }
}