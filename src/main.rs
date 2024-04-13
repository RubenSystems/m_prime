use crate::instruction_container::InstructionContainer;
use crate::program::Program;
use instruction::Instruction;
use vm::VirtualMachine;

mod instruction;
mod instruction_container;
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
    let program = Program::new(
        start
            .into_iter()
            .map(|x| InstructionContainer::new(x))
            .collect(),
    );

    let mut process = VirtualMachine::new(3);
    let basis = process.exe(&program);
    println!("{basis:?}");
    println!("{program}");
    println!("{:?}", process.instruction_counter);
}
