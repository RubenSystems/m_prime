use crate::program::Instruction;

pub enum Actions {
    Remove(usize),
    Add(usize, Instruction),
}
