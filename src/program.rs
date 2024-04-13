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

pub struct VirtualMachine {
    registers: Vec<i32>,
    memory: HashMap<usize, i32>,
    pub instruction_counter: HashMap<usize, i32>,
}

impl VirtualMachine {
    pub fn new(register_count: usize) -> Self {
        Self {
            registers: vec![0; register_count],
            memory: HashMap::new(),
            instruction_counter: HashMap::new(),
        }
    }

    pub fn exe(&mut self, instructions: &Program) -> (usize, Vec<String>) {
        let mut pc = 0;
        let mut cost = 0;
        let mut output = vec![];
        let mut its = 0;
        loop {
            let Some(instruction) = instructions.get(pc) else {
                break;
            };
            if its > 10000 {
                println!("[VM] - killed: too many runs");
                break;
            }

            pc += 1;
            its += 1;
            cost += instruction.cost();

            self.instruction_counter.insert(
                instruction.id,
                self.instruction_counter.get(&instruction.id).unwrap_or(&0) + 1,
            );

            match &instruction.code {
                Instruction::Add { rega, regb, outreg } => {
                    self.registers[*outreg] = self.registers[*rega] + self.registers[*regb]
                }
                Instruction::SetReg { register, constant } => self.registers[*register] = *constant,
                Instruction::Sub { rega, regb, outreg } => {
                    let ina = self.registers[*rega];
                    let inb = self.registers[*regb];
                    self.registers[*outreg] = ina - inb;
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
                Instruction::PCSetIfNotZero {
                    register,
                    jump_point,
                } => {
                    if self.registers[*register] != 0 {
                        pc = *jump_point;
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
