use program::{Instruction, Process};

mod program;

fn main() {
    let program = vec![
        Instruction::Var(0),
        Instruction::SetReg {
            register: 0,
            constant: 100,
        },
        Instruction::Store {
            register: 0,
            variable: 0,
        },
        Instruction::Load {
            register: 0,
            variable: 0,
        },
        Instruction::Output(0),
    ];

    let mut process = Process::new(program, 2);
    process.run();
}
