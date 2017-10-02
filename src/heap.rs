use std::io;

const HEAP_SIZE: usize = 256;

pub struct Heap {
    heap: [u8;HEAP_SIZE],
}

impl Heap {

    pub fn new() -> Heap {

        let heap = [0; HEAP_SIZE];
        Heap{heap}
    }

    pub fn initialize(&mut self) {

        for i in 0..HEAP_SIZE {
            self.heap[i] = 0x00;
        }
    }

    pub fn write(&mut self, address: usize, value: u8) {

       self.heap[address] = value;
    }

    pub fn read(&self, address: usize) -> u8 {

        self.heap[address]
    }

    pub fn dump(&self) {

        let mut address_counter = 0;
        let mut j = 0;

        println!("Heap Contents:");
        println!("--------------");

        for i in 0..HEAP_SIZE {
            if j % 16 == 0 {
                print!("\nAddress({:2X}) ", address_counter);
                address_counter += 16;
            }

            print!("{:X} ", self.heap[i]);
            j += 1;
        }

        println!("\n");
    }
}
