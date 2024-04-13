use std::{
    collections::HashMap,
    fmt::{format, write, Debug, Display},
};

static mut INSTR_ID: usize = 0;

fn generate_new_id() -> usize {
    let tmp = unsafe { INSTR_ID };
    unsafe { INSTR_ID += 1 };
    return tmp;
}

#[derive(Clone, Copy, Debug)]
pub struct InstructionContainer {
    code: Instruction,
    id: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    // Math
    Add {
        rega: usize,
        regb: usize,
        outreg: usize,
    },
    Sub {
        rega: usize,
        regb: usize,
        outreg: usize,
    },

    // Memory operations
    Var(usize),
    Load {
        register: usize,
        variable: usize,
    },
    Store {
        register: usize,
        variable: usize,
    },
    SetReg {
        register: usize,
        constant: i32,
    },

    // Branching
    PCSetIfNotZero {
        register: usize,
        jump_point: usize,
    },

    // IO
    Output(usize),
}

impl InstructionContainer {
    pub fn new(code: Instruction) -> Self {
        Self {
            code,
            id: generate_new_id(),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn code(&self) -> Instruction {
        self.code
    }

    pub fn cost(&self) -> usize {
        match self.code {
            Instruction::Add {
                rega: _,
                regb: _,
                outreg: _,
            } => 1,
            Instruction::Sub {
                rega: _,
                regb: _,
                outreg: _,
            } => 1,
            Instruction::Var(_) => 1,
            Instruction::Load {
                register: _,
                variable: _,
            } => 2,
            Instruction::Store {
                register: _,
                variable: _,
            } => 2,
            Instruction::SetReg {
                register: _,
                constant: _,
            } => 1,
            Instruction::PCSetIfNotZero {
                register: _,
                jump_point: _,
            } => 10,
            Instruction::Output(_) => 1,
        }
    }
}

#[derive(Clone)]
pub struct Program(Vec<InstructionContainer>);

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: Vec<String> = self.0.iter().map(|x| format!("{:?}", x.code)).collect();
        write!(f, "==\n{}\n==", msg.join("\n"))
    }
}

impl Program {
    pub fn new(start: Vec<InstructionContainer>) -> Self {
        Program(start)
    }

    pub fn get(&self, index: usize) -> Option<&InstructionContainer> {
        self.0.get(index)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, index: usize, element: InstructionContainer) {
        self.0.insert(index, element)
    }

    pub fn remove(&mut self, index: usize) {
        self.0.remove(index);
    }
}
