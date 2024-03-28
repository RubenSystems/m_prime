use std::collections::HashMap;

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

pub struct Process {
    instructions: Vec<Instruction>,
    pc: usize,
    registers: Vec<i32>,
    memory: HashMap<usize, i32>,
}

impl Process {
    pub fn new(instructions: Vec<Instruction>, register_count: usize) -> Self {
        Self {
            instructions,
            pc: 0,
            registers: vec![0; register_count],
            memory: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let Some(instruction) = self.instructions.get(self.pc) else {
                break;
            };
            self.pc += 1;
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
                    println!("Register: {register} = {}", self.registers[*register]);
                }
            }
        }
    }
}
