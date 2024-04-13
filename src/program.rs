use crate::InstructionContainer;
use std::{
    collections::HashMap,
    fmt::{format, write, Debug, Display},
};

#[derive(Clone)]
pub struct Program(Vec<InstructionContainer>);

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: Vec<String> = self.0.iter().map(|x| format!("{:?}", x.code())).collect();
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
