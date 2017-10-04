use std::io;

use cpu::Cpu;
use memory::Memory;
use heap::Heap;
use stack::Stack;

// FUNCTION: op_halt (HALT op code 0x00)
pub fn op_halt(cpu: &mut Cpu) {
    cpu.set_halt();
    cpu.increment_program_counter();
}

// FUNCTION: op_set0 (SET0 op code 0x01)
pub fn op_set0(cpu: &mut Cpu, memory: &Memory) {
    cpu.increment_program_counter();
    let pc = cpu.get_program_counter();
    cpu.set_register0(memory.read(pc as usize));
    cpu.increment_program_counter();
}

// FUNCTION: op_set1 (SET1 op code 0x02)
pub fn op_set1(cpu: &mut Cpu, memory: &Memory) {
    cpu.increment_program_counter();
    let pc = cpu.get_program_counter();
    cpu.set_register1(memory.read(pc as usize));
    cpu.increment_program_counter();
}

// FUNCTION: op_add (ADD op code 0x03)
pub fn op_add(cpu: &mut Cpu) {
    let sum = cpu.get_register0() + cpu.get_register1();
    cpu.set_register0(sum);
    cpu.increment_program_counter();
}

// FUNCTION: op_store (STOR op code 0x04)
pub fn op_store(cpu: &mut Cpu, memory: &Memory, heap: &mut Heap) {
    cpu.increment_program_counter();
    let pc = cpu.get_program_counter();
    cpu.set_heap_pointer(memory.read(pc as usize));
    heap.write(cpu.get_heap_pointer() as usize, cpu.get_register0());
    cpu.increment_program_counter();
}

// FUNCTION: op_print (PRT op code 0x05)
pub fn op_print(cpu: &mut Cpu, memory: &Memory, heap: &mut Heap) {
    cpu.increment_program_counter();
    let pc = cpu.get_program_counter();
    cpu.set_heap_pointer(memory.read(pc as usize));
    let hp = cpu.get_heap_pointer();
    println!("HEAP ADDRESS = {:2X} VALUE: {:2X}", hp, heap.read(hp as usize));
    cpu.increment_program_counter();
}

// FUNCTION: op_multiply (MULT op code 0x07)
pub fn op_multiply(cpu: &mut Cpu) {
    let product = cpu.get_register0() * cpu.get_register1();
    cpu.set_register0(product);
    cpu.increment_program_counter();
}

// FUNCTION: op_divide (DIV op code 0x08)
pub fn op_divide(cpu: &mut Cpu) {
    let quotient = cpu.get_register0() / cpu.get_register1();
    cpu.set_register0(quotient);
    cpu.increment_program_counter();
}

// FUNCTION: op_subtract (SUB op code 0x09)
pub fn op_subtract(cpu: &mut Cpu) {
    let difference = cpu.get_register0() - cpu.get_register1();
    cpu.set_register0(difference);
    cpu.increment_program_counter();
}

// FUNCTION: run
pub fn run(cpu: &mut Cpu, memory: &mut Memory, heap: &mut Heap,
        stack: &mut Stack)
{
    println!("... Running ...");

    loop {

        let pc = cpu.get_program_counter();

        match memory.read(pc as usize) {
            0x00 => { op_halt(cpu); break; },
            0x01 => op_set0(cpu, memory),
            0x02 => op_set1(cpu, memory),
            0x03 => op_add(cpu),
            0x04 => op_store(cpu, memory, heap),
            0x05 => op_print(cpu, memory, heap),
            0x07 => op_multiply(cpu),
            0x08 => op_divide(cpu),
            0x09 => op_subtract(cpu),
            _ => break,
        }
    }
}
