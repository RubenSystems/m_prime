use std::collections::HashMap;

use crate::instruction_container::InstructionContainer;
use crate::op_finder::mcts;
use crate::program::Program;
use crate::programs::{add_two, count_to_x, vecmul};
use instruction::Instruction;
use vm::VirtualMachine;

mod instruction;
mod instruction_container;
mod op_finder;
mod program;
mod programs;
mod vm;

fn main() {
    let program = count_to_x::prog(1000);

    // let mut base_memory = HashMap::<usize, i32>::new();
    // for i in 0..5 {
    //     base_memory.insert(i, (i + 1) as i32);
    // }

    // for i in 0..5 {
    //     base_memory.insert(i + 10, (i + 1) as i32);
    // }

    let mut process = VirtualMachine::new(4);
    let basis = process.exe(&program).expect("Error in root");
    println!("{basis:?}");
    println!("{program}");

    let op = mcts(program, &mut process, &basis, 50_000, 20_000);
    println!("=============");
    if let Some(op) = op {
        println!("Optimised!");
        println!("Optimisation amount: {}", op.0);
        println!("Optimised program: \n\n{}\n", op.1.unwrap());
    }
}
