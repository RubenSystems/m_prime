use program::{Instruction, Process};

mod program;

fn main() {
    let program = vec![
        Instruction::Var(0),
        Instruction::Add(0, 1),
        Instruction::OutputVar(0),
        Instruction::PCSetIf {
            variable_name: 0,
            predicate: |x| x < 100,
            jump_point: 1,
        },
    ];

    let mut process = Process::new(program);
    process.run();
}
