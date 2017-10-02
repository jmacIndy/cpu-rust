use std::io;

const GREATER_THAN_SET:u8 = 0x80;
const LESS_THAN_SET:u8 = 0x40;
const ZERO_SET:u8 = 0x20;
const NON_ZERO_SET:u8 = 0x10;
const OVERFLOW_SET:u8 = 0x08;
const UNDERFLOW_SET:u8 = 0x04;
const SIGNING_SET:u8 = 0x02;
const HALT_SET:u8 = 0x01;

const GREATER_THAN_RESET:u8 = 0x7F;
const LESS_THAN_RESET:u8 = 0xBF;
const ZERO_RESET:u8 = 0xDF;
const NON_ZERO_RESET:u8 = 0xEF;
const OVERFLOW_RESET:u8 = 0xF7;
const UNDERFLOW_RESET:u8 = 0xFB;
const SIGNING_RESET:u8 = 0xFD;
const HALT_RESET:u8 = 0xFE;

pub struct Cpu {
    register0: u8,
    register1: u8,
    program_counter: u8,
    heap_pointer: u8,
    flags: u8,
}

impl Cpu {

    pub fn new() -> Cpu {
        let register0 = 0x00;
        let register1 = 0x00;
        let program_counter = 0x00;
        let heap_pointer = 0x00;
        let flags = 0x00;

        Cpu{ register0, register1, program_counter, 
            heap_pointer, flags }
    }

    pub fn initialize(&mut self) {
        self.register0 = 0x00;
        self.register1 = 0x00;
        self.program_counter = 0x00;
        self.heap_pointer = 0x00;
        self.flags = 0x00;
    }

    pub fn set_greater_than(&mut self) {
        self.flags |= GREATER_THAN_SET;
    }

    pub fn set_less_than(&mut self) {
        self.flags |= LESS_THAN_SET;
    }

    pub fn set_zero(&mut self) {
        self.flags |= ZERO_SET;
    }

    pub fn set_non_zero(&mut self) {
        self.flags |= NON_ZERO_SET;
    }

    pub fn set_overflow(&mut self) {
        self.flags |= OVERFLOW_SET;
    }

    pub fn set_underflow(&mut self) {
        self.flags |= UNDERFLOW_SET;
    }

    pub fn set_signing(&mut self) {
        self.flags |= SIGNING_SET;
    }

    pub fn set_halt(&mut self) {
        self.flags |= HALT_SET;
    }

    pub fn reset_greater_than(&mut self) {
        self.flags &= GREATER_THAN_RESET;
    }

    pub fn reset_less_than(&mut self) {
        self.flags &= LESS_THAN_RESET;
    }

    pub fn reset_zero(&mut self) {
        self.flags &= ZERO_RESET;
    }

    pub fn reset_non_zero(&mut self) {
        self.flags &= NON_ZERO_RESET;
    }

    pub fn reset_overflow(&mut self) {
        self.flags &= OVERFLOW_RESET;
    }

    pub fn reset_underflow(&mut self) {
        self.flags &= UNDERFLOW_RESET;
    }

    pub fn reset_signing(&mut self) {
        self.flags &= SIGNING_RESET;
    }

    pub fn reset_halt(&mut self) {
        self.flags &= HALT_RESET;
    }

    pub fn is_greater_than(&self) -> bool {
        (self.flags & GREATER_THAN_SET) == 0x00
    }

    pub fn is_less_than(&self) -> bool {
        (self.flags & LESS_THAN_SET) == 0x00
    }

    pub fn is_zero(&self) -> bool {
        (self.flags & ZERO_SET) == 0x00
    }

    pub fn is_non_zero(&self) -> bool {
        (self.flags & NON_ZERO_SET) == 0x00
    }

    pub fn is_overflow(&self) -> bool {
        (self.flags & OVERFLOW_SET) == 0x00
    }

    pub fn is_underflow(&self) -> bool {
        (self.flags & UNDERFLOW_SET) == 0x00
    }

    pub fn is_signing(&self) -> bool {
        (self.flags & SIGNING_SET) == 0x00
    }

    pub fn is_halt(&self) -> bool {
        (self.flags & HALT_SET) == 0x00
    }

    pub fn increment_program_counter(&mut self) {
        self.program_counter += 1;
    }

    pub fn dump(&self) {

        println!("\nCPU Contents:");
        println!("-------------");

        println!("Register 0      = {:2X}", self.register0);
        println!("Register 1      = {:2X}", self.register1);
        println!("Program Counter = {:2X}", self.program_counter);
        println!("Heap Pointer    = {:2X}", self.heap_pointer);
    }
}
