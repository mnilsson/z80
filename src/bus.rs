pub trait Bus {
    fn memory_read(&self, address: usize) -> u8;
    fn memory_read_word(&self, address: usize) -> u16;
    fn memory_write(&mut self, address: usize, value: u8);
    fn memory_write_word(&mut self, address: usize, value: u16);


    fn port_read(&mut self, port: u8) -> u8;
    fn port_write(&mut self, port: u8, value: u8);

    fn tick(&mut self, machine_cycles: u8, t_states: u8);
}