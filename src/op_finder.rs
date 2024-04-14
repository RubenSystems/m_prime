use crate::Program;
use crate::VirtualMachine;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash)]
struct ProgramState {
    program: Program,
    out: (usize, Vec<String>),
}

impl ProgramState {
    fn new(program: Program, vm: &mut VirtualMachine) -> Option<Self> {
        let Ok(out) = vm.exe(&program) else {
            return None;
        };

        Some(Self { out, program })
    }

    fn is_correct(&self, real: &Vec<String>) -> bool {
        if self.out.1.len() != real.len() {
            return false;
        }
        for (x, y) in real.iter().zip(self.out.1.iter()) {
            if x != y {
                return false;
            }
        }
        return true;
    }

    fn is_more_optimal(&self, real: usize) -> bool {
        self.out.0 < real
    }

    pub fn reward(&self, real: &(usize, Vec<String>)) -> isize {
        if self.is_more_optimal(real.0) && self.is_correct(&real.1) {
            println!(
                "== FOUND CORRECT && OPTIMAL ALGO, diff {}\nOUT: {}",
                real.0 - self.out.0,
                self.out.1.join(" | ")
            );
            println!("{}\n==", self.program);
            return (real.0 as isize) - (self.out.0 as isize);
        } else {
            return -100;
        }
    }

    pub fn next_moves(&self, vm: &mut VirtualMachine) -> Vec<Self> {
        let mut new_moves = vec![];

        // removals
        for i in 0..self.program.len() {
            let mut p = self.program.clone();
            p.remove(i);
            if let Some(m) = Self::new(p, vm) {
                new_moves.push(m);
            }
        }

        new_moves
    }
}

struct Node {
    state: ProgramState,
    visits: u32,
    wins: u32,
    children: HashMap<ProgramState, Node>,
}

impl Node {
    fn new(state: ProgramState) -> Self {
        Self {
            state,
            visits: 0,
            wins: 0,
            children: HashMap::new(),
        }
    }

    fn uct_value(&self, parent_visits: u32) -> f64 {
        if self.visits == 0 {
            f64::INFINITY
        } else {
            let exploitation = self.wins as f64 / self.visits as f64;
            let exploration = (2.0 * (parent_visits as f64).ln() / self.visits as f64).sqrt();
            exploitation + exploration
        }
    }

    fn leaf(&self) -> bool {
        self.children.is_empty()
    }
}

pub fn mcts(program: Program, vm: &mut VirtualMachine, real: &(usize, Vec<String>)) {
    let mut root = Node::new(ProgramState::new(program, vm).expect("Error in root"));

    for _ in 1..10_000 {
        mcts_node(&mut root, vm, real);
    }
}

fn mcts_node(node: &mut Node, vm: &mut VirtualMachine, real: &(usize, Vec<String>)) -> u32 {
    let better = if node.leaf() {
        // Expand, simulate
        let new_states = node.state.next_moves(vm);
        for new_state in new_states {
            if !node.children.contains_key(&new_state) {
                node.children
                    .insert(new_state.clone(), Node::new(new_state));
            }
        }
        let random_action = node.children.iter_mut().choose(&mut thread_rng()).unwrap();
        let reward = mcts_simulate(random_action.1, vm, real);
        reward.max(0) as u32
    } else {
        let best_child = node
            .children
            .iter_mut()
            .max_by(|(_, child1), (_, child2)| {
                child1
                    .uct_value(node.visits)
                    .partial_cmp(&child2.uct_value(node.visits))
                    .unwrap()
            })
            .unwrap()
            .1;
        mcts_node(best_child, vm, real)
    };
    node.visits += 1;
    node.wins = better;

    better
}

fn mcts_simulate(node: &mut Node, vm: &mut VirtualMachine, real: &(usize, Vec<String>)) -> isize {
    let mut rollout_state = node.state.clone();
    let mut max_reward = isize::MIN;

    for _ in 0..2000 {
        let next_states = rollout_state.next_moves(vm);

        if next_states.is_empty() {
            break;
        }

        let next_state = next_states.choose(&mut thread_rng()).unwrap().clone();
        // .choose(&mut thread_rng())
        // .unwrap();
        rollout_state = next_state;
        max_reward = max_reward.max(rollout_state.reward(real))
    }

    max_reward
}
