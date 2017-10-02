mod memory;
mod cpu;

use std::io::prelude::*;
use std::io;

use memory::Memory;
use cpu::Cpu;

fn display_menu() -> char {

    let mut input = String::new();

    println!("Menu:");
    println!("-------------------------");
    println!("1. Reset the CPU");
    println!("2. Clear the memory");
    println!("3. Dump the CPU");
    println!("4. Dump the memory");
    println!("5. Dump the heap");
    println!("6. Dump the stack");
    println!("7. Run the CPU");
    println!("8. Load program from memory");
    println!("X. Exit the CPU");
    print!("   Your choice ===> ");

    // print! does not flush the buffer; do it manually
    io::stdout().flush();

    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(error) => println!("\nERROR: Invalid input: {}", error),
    }

    input.chars().nth(0).unwrap()
}

fn dump_heap() {

    println!("in dump_heap");
}

fn dump_stack() {

    println!("in dump_stack");
}

fn run_cpu() {

    println!("in run_cpu");
}

fn load_program() {

    println!("in load_program");
}

fn main() {

    let mut memory = Memory::new();
    let mut cpu = Cpu::new();

    println!("=========================");
    println!("=== Welcome to my CPU ===");
    println!("=========================");

    memory.initialize();

    memory.write(0, 0x01);
    memory.write(10, 0x03);

    println!("Value at 0 = {}", memory.read(0));
    println!("Value at 10 = {}", memory.read(10));

    cpu.increment_program_counter();
    
    loop {
        match display_menu() {
            '1' => cpu.initialize(),
            '2' => memory.initialize(),
            '3' => cpu.dump(),
            '4' => memory.dump(),
            '5' => dump_heap(),
            '6' => dump_stack(),
            '7' => run_cpu(),
            '8' => load_program(),
            'X' | 'x' => break,
            _ => println!("ERROR: Bad selection! Try again!"),
        }
    }
}
