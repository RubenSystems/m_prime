use crate::Instruction;
use crate::Program;

pub fn prog() -> Program {
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
    Program::new(start)
}
