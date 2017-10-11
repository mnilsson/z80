extern crate z80;

#[cfg(test)]
mod test_z80 {
    use z80::cpu::Z80;
    use z80::bus::Bus;
    use std::io;
    use std::io::Write;

    #[derive(Clone)]
    struct TestBus {
        memory: Vec<u8>,
        pub m_cycles: usize,
        pub t_states: usize,
        pub ticks: usize,
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
            print!("{}", byte as char);
            io::stdout().flush().unwrap();
        }

        #[allow(unused_variables)]
        fn port_read(&mut self, port: u8) -> u8 {
            0xff
        }

        fn tick(&mut self, machine_cycles: u8, _: u8) {
            self.m_cycles += machine_cycles as usize;
        }
    }

    fn new_cpu(mut prg: Vec<u8>) -> (Z80, TestBus) {
        prg.resize(0xffff, 0);
        let mut bus = TestBus::new(prg);
        (Z80::new(), bus)
    }



    #[test]
    #[ignore]
    fn run_functional_tests() {
        let prog = include_bytes!("../roms/zexdoc.com");

        let mut p = vec![0xff; 1 << 16];

        let mut offset = 0;


        let (mut cpu, mut bus) = new_cpu(p);
        for b in prog.iter() {
            bus.memory_write(0x100 + offset, (*b) as u8);
            offset += 1;
        }
        cpu.pc = 0x100;
        cpu.sp = 0xf000;

        bus.memory_write(0, 0xc3);
        bus.memory_write(1, 0x00);
        bus.memory_write(2, 0xf0);

        bus.memory_write(5, 0xc3);
        bus.memory_write(6, 0x00);
        bus.memory_write(7, 0xf0);




        let mut num_ops: u64 = 0;
        let mut num_cycles: u64 = 0;

        loop {
            num_ops += 1;
            cpu.step(&mut bus, 0);
            num_cycles += bus.t_states as u64;

            match cpu.pc {
                0x0000 => {
                    break;
                }
                0x0005 => {
                    match cpu.registers.c {
                        2 => {
                            // output a character
                            print!("{}", cpu.registers.e as u8 as char);
                            io::stdout().flush().unwrap();
                        }
                        9 => {
                            // output a string
                            let d = cpu.registers.d;
                            let e = cpu.registers.e;
                            let de = ((d as u16) << 8) | (e as u16);


                            let mut addr = de;
                            loop {
                                let c = bus.memory_read(addr as usize) as u8;
                                addr = (addr + 1) & 0xFFFF;
                                if c != b'$' {
                                    print!("{}", c as char);
                                    io::stdout().flush().unwrap();
                                } else {
                                    break;
                                }
                            }
                        }
                        _ => {
                            panic!("Unknown CP/M call {}!", cpu.registers.c);
                        }
                    }
                    cpu.ret(&mut bus);
                }
                _ => {}
            }
        }
        println!("{} operations tested", num_ops);
    }

}
