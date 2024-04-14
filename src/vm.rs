use std::collections::HashMap;

use crate::Instruction;
use crate::Program;

pub struct VirtualMachine {
    register_count: usize,
}

#[derive(Debug)]
pub enum ExecutionError {
    VariableNotFound,
    Timeout,
}

impl VirtualMachine {
    pub fn new(register_count: usize) -> Self {
        Self { register_count }
    }

    pub fn exe(&mut self, instructions: &Program) -> Result<(usize, Vec<String>), ExecutionError> {
        let mut pc = 0;
        let mut cost = 0;
        let mut output = vec![];
        let mut its = 0;

        let mut registers = vec![0; self.register_count];
        let mut memory = HashMap::new();
        let mut instruction_counter = HashMap::new();

        loop {
            let Some(instruction) = instructions.get(pc) else {
                break;
            };
            if its > 10000 {
                return Err(ExecutionError::Timeout);
            }

            pc += 1;
            its += 1;
            cost += instruction.cost();

            instruction_counter.insert(
                instruction.id(),
                instruction_counter.get(&instruction.id()).unwrap_or(&0) + 1,
            );

            match &instruction.code() {
                Instruction::Add { rega, regb, outreg } => {
                    registers[*outreg] = registers[*rega] + registers[*regb]
                }
                Instruction::SetReg { register, constant } => registers[*register] = *constant,
                Instruction::Sub { rega, regb, outreg } => {
                    let ina = registers[*rega];
                    let inb = registers[*regb];

                    registers[*outreg] = ina - inb;
                }
                Instruction::Var(name) => {
                    memory.insert(*name, 0);
                }
                Instruction::Load { register, variable } => {
                    registers[*register] = match memory.get(variable) {
                        Some(e) => *e,
                        None => return Err(ExecutionError::VariableNotFound),
                    }
                }
                Instruction::Store { register, variable } => {
                    // *memory
                    //     .get_mut(variable)
                    //     .expect("Could not find variable {variable}") = registers[*register];
                    match memory.get_mut(variable) {
                        Some(e) => *e = registers[*register],
                        None => return Err(ExecutionError::VariableNotFound),
                    }
                }
                Instruction::PCSetIfNotZero {
                    register,
                    jump_point,
                } => {
                    if registers[*register] != 0 {
                        pc = *jump_point;
                    }
                }
                Instruction::Output(register) => {
                    output.push(format!("Register: {register} = {}", registers[*register]));
                }
            }
        }
        Ok((cost, output))
    }
}
