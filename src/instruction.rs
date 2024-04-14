#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
