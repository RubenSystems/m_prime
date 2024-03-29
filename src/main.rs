use crate::{monte::Monte, program::Program};
use program::{Instruction, Process};

mod actions;
mod monte;
mod program;
mod reward;

fn main() {
    let program = Program::new(vec![
        Instruction::SetReg {
            register: 0,
            constant: 0,
        },
        Instruction::SetReg {
            register: 1,
            constant: 1,
        },
        Instruction::Add {
            rega: 0,
            regb: 1,
            outreg: 0,
        },
        Instruction::PCSetIf {
            register: 0,
            predicate: |x| x < 1000,
            jump_point: 2,
        },
        Instruction::Output(0),
    ]);

    let mut process = Process::new(2);
    let basis = process.exe(&program);
    println!("{basis:?}");

    let legal_operations = vec![
        Instruction::Add {
            rega: 0,
            regb: 1,
            outreg: 0,
        },
        Instruction::SetReg {
            register: 0,
            constant: 10,
        },
        Instruction::SetReg {
            register: 1,
            constant: 10,
        },
        Instruction::SetReg {
            register: 0,
            constant: 1,
        },
        Instruction::SetReg {
            register: 1,
            constant: 1,
        },
        Instruction::SetReg {
            register: 0,
            constant: 0,
        },
        Instruction::SetReg {
            register: 1,
            constant: 0,
        },
    ];

    let _monte = Monte::new(legal_operations, program, basis.1);

    // let new_prog: Vec<String> = trace.0.iter().map(|x| format!("{x:?}")).collect();
    // println!("{}\n==\n {}", new_prog.join("\n"), trace.1);
}
