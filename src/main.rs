use crate::instruction_container::InstructionContainer;
use crate::op_finder::mcts;
use crate::program::Program;
use instruction::Instruction;
use vm::VirtualMachine;

mod instruction;
mod instruction_container;
mod op_finder;
mod program;
mod vm;

fn main() {
    let start = vec![
        Instruction::Var(0),
        Instruction::SetReg {
            register: 0,
            constant: 0,
        },
        Instruction::SetReg {
            register: 1,
            constant: 1,
        },
        /*
            Add 1
        */
        Instruction::Add {
            rega: 0,
            regb: 1,
            outreg: 0,
        },
        Instruction::Store {
            register: 0,
            variable: 0,
        },
        /*
            Add 2
        */
        Instruction::Load {
            register: 0,
            variable: 0,
        },
        Instruction::Add {
            rega: 0,
            regb: 1,
            outreg: 0,
        },
        Instruction::Store {
            register: 0,
            variable: 0,
        },
        /*
            Output val
        */
        Instruction::Load {
            register: 0,
            variable: 0,
        },
        Instruction::Output(0),
    ];
    let program = Program::new(start);

    let mut process = VirtualMachine::new(3);
    let basis = process.exe(&program).expect("Error in root");
    println!("{basis:?}");
    println!("{program}");

    let op = mcts(program, &mut process, &basis);
    println!("=============");
    if let Some(op) = op {
        println!("Optimised!");
        println!("Optimisation amount: {}", op.0);
        println!("Optimised program: \n\n{}\n", op.1.unwrap());
    }
}
