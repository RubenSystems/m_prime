use std::{
    collections::HashMap,
};

use crate::Instruction;
use crate::Program;

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
                instruction.id(),
                self.instruction_counter
                    .get(&instruction.id())
                    .unwrap_or(&0)
                    + 1,
            );

            match &instruction.code() {
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
