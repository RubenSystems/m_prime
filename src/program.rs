use std::{
    collections::HashMap,
    fmt::{Debug},
};

const EXEC_AVG_COUNT: u128 = 10;

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
    PCSetIf {
        register: usize,
        predicate: fn(i32) -> bool,
        jump_point: usize,
    },

    // IO
    Output(usize),
}

impl Instruction {
    pub fn cost(&self) -> usize {
        match self {
            Instruction::Add { rega: _, regb: _, outreg: _ } => 1,
            Instruction::Sub { rega: _, regb: _, outreg: _ } => 1,
            Instruction::Var(_) => 1,
            Instruction::Load { register: _, variable: _ } => 2,
            Instruction::Store { register: _, variable: _ } => 2,
            Instruction::SetReg { register: _, constant: _ } => 1,
            Instruction::PCSetIf {
                register: _,
                predicate: _,
                jump_point: _,
            } => 10,
            Instruction::Output(_) => 1,
        }
    }
}

pub struct Program(Vec<Instruction>);

impl Program {
    pub fn new(start: Vec<Instruction>) -> Self {
        Program(start)
    }

    pub fn get(&self, index: usize) -> Option<&Instruction> {
        self.0.get(index)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, index: usize, element: Instruction) {
        self.0.insert(index, element)
    }

    pub fn remove(&mut self, index: usize) {
        self.0.remove(index);
    }
}

pub struct Process {
    pc: usize,
    registers: Vec<i32>,
    memory: HashMap<usize, i32>,
}

impl Process {
    pub fn new(register_count: usize) -> Self {
        Self {
            pc: 0,
            registers: vec![0; register_count],
            memory: HashMap::new(),
        }
    }

    pub fn exe(&mut self, instructions: &Program) -> (usize, Vec<String>) {
        self.pc = 0;
        let mut cost = 0;
        let mut output = vec![];
        let mut its = 0;
        loop {
            let Some(instruction) = instructions.get(self.pc) else {
                break;
            };
            if its > 10000 {
                break;
            }

            self.pc += 1;
            its += 1;
            cost += instruction.cost();
            match instruction {
                Instruction::Add { rega, regb, outreg } => {
                    self.registers[*outreg] = self.registers[*rega] + self.registers[*regb]
                }
                Instruction::SetReg { register, constant } => self.registers[*register] = *constant,
                Instruction::Sub { rega, regb, outreg } => {
                    self.registers[*outreg] = self.registers[*rega] - self.registers[*regb]
                }
                Instruction::Var(name) => {
                    self.memory.insert(*name, 0);
                }
                Instruction::Load { register, variable } => {
                    self.registers[*register] = *self
                        .memory
                        .get(variable)
                        .expect("Could not find variable {variable}")
                }
                Instruction::Store { register, variable } => {
                    *self
                        .memory
                        .get_mut(variable)
                        .expect("Could not find variable {variable}") = self.registers[*register];
                }
                Instruction::PCSetIf {
                    register,
                    predicate,
                    jump_point,
                } => {
                    if predicate(self.registers[*register]) {
                        self.pc = *jump_point;
                    }
                }
                Instruction::Output(register) => {
                    output.push(format!(
                        "Register: {register} = {}",
                        self.registers[*register]
                    ));
                }
            }
        }
        (cost, output)
    }
}
