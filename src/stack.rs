use std::io;

const STACK_SIZE: usize = 256;

pub struct Stack {
    stack: [u8;STACK_SIZE],
    stack_pointer: usize,
}

impl Stack {

    pub fn new() -> Stack {

        let stack = [0; STACK_SIZE];
        let stack_pointer = 0x00;
        Stack{stack, stack_pointer}
    }

    pub fn initialize(&mut self) {

        for i in 0..STACK_SIZE {
            self.stack[i] = 0x00;
        }

        self.stack_pointer = 0x00;
    }

    pub fn push(&mut self, value: u8) {
        self.stack[self.stack_pointer] = value;
        self.stack_pointer += 1;
    }

    pub fn pop(&mut self) -> u8 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer]
    }

    pub fn dump(&self) {

        let mut address_counter = 0;
        let mut j = 0;

        println!("Stack Contents:");
        println!("---------------");
        println!("Stack Pointer = {:2X}", self.stack_pointer);

        for i in 0..STACK_SIZE {
            if j % 16 == 0 {
                print!("\nAddress({:2X}) ", address_counter);
                address_counter += 16;
            }

            print!("{:X} ", self.stack[i]);
            j += 1;
        }

        println!("\n");
    }
}
