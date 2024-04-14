use crate::Instruction;
use crate::Program;

pub fn create_vector(offset: usize, elements: Vec<i32>) -> Vec<Instruction> {
    elements
        .into_iter()
        .enumerate()
        .flat_map(|(idx, elem)| {
            vec![
                Instruction::Var(idx + offset),
                Instruction::SetReg {
                    register: 0,
                    constant: elem,
                },
                Instruction::Store {
                    register: 0,
                    variable: idx + offset,
                },
            ]
        })
        .collect()
}

pub fn add_and_store(var_a: usize, var_b: usize, out_var: usize) -> Vec<Instruction> {
    vec![
        Instruction::Var(out_var),
        Instruction::Load {
            register: 0,
            variable: var_a,
        },
        Instruction::Load {
            register: 1,
            variable: var_b,
        },
        Instruction::Add {
            rega: 0,
            regb: 1,
            outreg: 0,
        },
        Instruction::Store {
            register: 0,
            variable: out_var,
        },
        Instruction::Output(0),
    ]
}

pub fn prog() -> Program {
    let vec_a_offset = 0;
    let vec_b_offset = 10;
    let vec_c_offset = 20;
    let mut a_init = create_vector(vec_a_offset, vec![1, 2, 3, 4, 5]);
    let b_init = create_vector(vec_b_offset, vec![1, 2, 3, 4, 5]);
    let zip: Vec<Instruction> = (0..5)
        .flat_map(|x| add_and_store(x + vec_a_offset, x + vec_b_offset, x + vec_c_offset))
        .collect();

    a_init.extend(b_init);
    a_init.extend(zip);
    // a_init.extend(prog);
    Program::new(a_init)
}
