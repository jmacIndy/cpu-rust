use std::io;

const MEMORY_SIZE: usize = 256;

pub struct Memory {
    memory:[u8;MEMORY_SIZE],
}

impl Memory {

    pub fn new() -> Memory {

        let memory = [0; MEMORY_SIZE];
        Memory{memory}
    }

    pub fn initialize(&mut self) {

        for i in 0..MEMORY_SIZE {
            self.memory[i] = 0x00;
        }
    }

    pub fn write(&mut self, address: usize, value: u8) {

       self.memory[address] = value;
    }

    pub fn read(&self, address: usize) -> u8 {

        self.memory[address]
    }

    pub fn dump(&self) {

        let mut address_counter = 0;
        let mut j = 0;

        println!("Memory Contents:");
        println!("----------------");

        for i in 0..MEMORY_SIZE {
            if j % 16 == 0 {
                print!("\nAddress({:2X}) ", address_counter);
                address_counter += 16;
            }

            print!("{:X} ", self.memory[i]);
            j += 1;
        }

        println!("\n");
    }
}
