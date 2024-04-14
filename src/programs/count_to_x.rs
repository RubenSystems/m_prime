use crate::Instruction;
use crate::Program;

pub fn prog(counter: i32) -> Program {
    let start = vec![
        Instruction::Var(0),
        Instruction::SetReg {
            register: 0,
            constant: 0,
        },
        // Startof loop
        Instruction::SetReg {
            register: 1,
            constant: 1,
        },
        Instruction::Add {
            rega: 0,
            regb: 1,
            outreg: 0,
        },
        // Loop bound checking
        Instruction::SetReg {
            register: 1,
            constant: counter,
        },
        Instruction::Sub {
            rega: 0,
            regb: 1,
            outreg: 1,
        },
        Instruction::PCSetIfNotZero {
            register: 1,
            jump_point: 2,
        },
        Instruction::Output(0),
    ];
    Program::new(start)
}
