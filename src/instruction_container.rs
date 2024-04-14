use crate::Instruction;

static mut INSTR_ID: usize = 0;

fn generate_new_id() -> usize {
    let tmp = unsafe { INSTR_ID };
    unsafe { INSTR_ID += 1 };
    return tmp;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InstructionContainer {
    code: Instruction,
    id: usize,
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
            Instruction::VecAdd {
                a1r,
                b1r,
                r1,
                a2r,
                b2r,
                r2,
            } => 1,
        }
    }
}
