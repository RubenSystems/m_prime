use std::collections::HashMap;

pub enum Instruction {
    Var(usize),
    Add(usize, u32),
    Sub(usize, u32),
    PCSetIf {
        variable_name: usize,
        predicate: fn(u32) -> bool,
        jump_point: usize,
    },
    OutputVar(usize),
}

pub struct Process {
    instructions: Vec<Instruction>,
    pc: usize,
    memory: HashMap<usize, u32>,
}

impl Process {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            pc: 0,
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
                Instruction::Var(name) => {
                    self.memory.insert(*name, 0);
                }
                Instruction::Add(varname, inc_value) => {
                    let current_value = self
                        .memory
                        .get_mut(varname)
                        .expect(&format!("Undefined variable reference {varname}"));
                    *current_value += inc_value;
                }
                Instruction::Sub(varname, dec_value) => {
                    let current_value = self
                        .memory
                        .get_mut(varname)
                        .expect(&format!("Undefined variable reference {varname}"));
                    *current_value += dec_value;
                }
                Instruction::PCSetIf {
                    variable_name,
                    predicate,
                    jump_point,
                } => {
                    let current_value = self
                        .memory
                        .get(variable_name)
                        .expect(&format!("Undefined variable reference {variable_name}"));
                    if predicate(*current_value) {
                        self.pc = *jump_point;
                    }
                }
                Instruction::OutputVar(varname) => {
                    let current_value = self
                        .memory
                        .get(varname)
                        .expect(&format!("Undefined variable reference {varname}"));
                    println!("Variable: {varname} = {current_value}")
                }
            }
        }
    }
}
