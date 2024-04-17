use crate::Instruction;
use crate::Program;
use std::collections::HashMap;

const TIMEOUT: usize = 10000;

pub struct VirtualMachine {
    register_count: usize,
    base_memory: HashMap<usize, i32>,
}

#[derive(Debug)]
pub enum ExecutionError {
    VariableNotFound,
    Timeout,
    OverflowArithmatic,
}

impl VirtualMachine {
    pub fn new(register_count: usize) -> Self {
        Self {
            register_count,
            base_memory: HashMap::new(),
        }
    }

    pub fn from_memory_state(register_count: usize, base_memory: HashMap<usize, i32>) -> Self {
        Self {
            register_count,
            base_memory,
        }
    }

    pub fn exe(&mut self, instructions: &Program) -> Result<(usize, Vec<String>), ExecutionError> {
        let mut pc = 0;
        let mut cost = 0;
        let mut output = vec![];
        let mut its = 0;

        let mut registers = vec![0; self.register_count];
        let mut memory = self.base_memory.clone();
        let mut instruction_counter = HashMap::new();

        loop {
            let Some(instruction) = instructions.get(pc) else {
                break;
            };
            if its > TIMEOUT {
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
                    let ra: i32 = registers[*rega];
                    let x: i32 = match ra.checked_add(registers[*regb]) {
                        Some(v) => v,
                        None => return Err(ExecutionError::OverflowArithmatic),
                    };
                    registers[*outreg] = x;
                }
                Instruction::SetReg { register, constant } => registers[*register] = *constant,
                Instruction::Sub { rega, regb, outreg } => {
                    let ra: i32 = registers[*rega];
                    let x: i32 = match ra.checked_sub(registers[*regb]) {
                        Some(v) => v,
                        None => return Err(ExecutionError::OverflowArithmatic),
                    };
                    registers[*outreg] = x;
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
                Instruction::Store { register, variable } => match memory.get_mut(variable) {
                    Some(e) => *e = registers[*register],
                    None => return Err(ExecutionError::VariableNotFound),
                },
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
                Instruction::VecAdd {
                    a1r,
                    b1r,
                    r1,
                    a2r,
                    b2r,
                    r2,
                } => {
                    let ra: i32 = registers[*a1r];
                    let res1: i32 = match ra.checked_sub(registers[*b1r]) {
                        Some(v) => v,
                        None => return Err(ExecutionError::OverflowArithmatic),
                    };

                    let rb: i32 = registers[*a2r];
                    let res2: i32 = match rb.checked_sub(registers[*b2r]) {
                        Some(v) => v,
                        None => return Err(ExecutionError::OverflowArithmatic),
                    };

                    registers[*r1] = res1;
                    registers[*r2] = res2;
                }
            }
        }
        Ok((cost, output))
    }
}
