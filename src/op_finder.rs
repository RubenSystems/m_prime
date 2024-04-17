use crate::instruction::Instruction;
use crate::instruction_container::InstructionContainer;
use crate::vm::ExecutionError;
use crate::Program;
use crate::VirtualMachine;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash)]
struct ProgramState {
    program: Program,
    out: Option<(usize, Vec<String>)>,
}

// Diffing representation
//

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Action {
    Remove(usize),
    Replace(usize, Instruction),
    Add(usize, Instruction),
    Move(usize, usize),
    Nothing,
}

impl Instruction {
    fn instruction_replacements(&self) -> Vec<Instruction> {
        match self {
            Instruction::Add { rega, regb, outreg } => (0..4)
                .flat_map(|a| {
                    (0..4)
                        .flat_map(|b| {
                            (0..4)
                                .map(|o| Instruction::Add {
                                    rega: a,
                                    regb: b,
                                    outreg: 0,
                                })
                                .collect::<Vec<Instruction>>()
                        })
                        .collect::<Vec<Instruction>>()
                })
                .collect(),
            Instruction::Sub { rega, regb, outreg } => (0..4)
                .flat_map(|a| {
                    (0..4)
                        .flat_map(|b| {
                            (0..4)
                                .map(|o| Instruction::Sub {
                                    rega: a,
                                    regb: b,
                                    outreg: 0,
                                })
                                .collect::<Vec<Instruction>>()
                        })
                        .collect::<Vec<Instruction>>()
                })
                .collect(),
            Instruction::Var(a) => vec![Instruction::Var(*a)],
            Instruction::Load { register, variable } => (0..4)
                .map(|i| Instruction::Load {
                    register: i,
                    variable: *variable,
                })
                .collect(),
            Instruction::Store { register, variable } => (0..4)
                .map(|i| Instruction::Store {
                    register: i,
                    variable: *variable,
                })
                .collect(),
            Instruction::SetReg { register, constant } => (0..4)
                .map(|i| Instruction::SetReg {
                    register: i,
                    constant: *constant,
                })
                .collect(),
            Instruction::VecAdd {
                a1r,
                b1r,
                r1,
                a2r,
                b2r,
                r2,
            } => vec![],
            Instruction::PCSetIfNotZero {
                register,
                jump_point,
            } => vec![Instruction::PCSetIfNotZero {
                register: *register,
                jump_point: *jump_point,
            }],
            Instruction::Output(_) => vec![
                Instruction::Output(0),
                Instruction::Output(1),
                Instruction::Output(2),
                Instruction::Output(3),
            ],
        }
    }
}

impl Program {
    fn apply_action(mut self, action: &Action) -> Self {
        match action {
            Action::Remove(idx) => {
                self.remove(*idx);
            }
            Action::Replace(idx, new) => {
                self.insert(*idx, InstructionContainer::new(*new));
            }
            Action::Add(idx, new) => {
                self.insert(*idx, InstructionContainer::new(*new));
            }
            Action::Nothing => {}
            Action::Move(from, to) => {
                let rem = InstructionContainer::new(self.remove(*from));
                self.insert(*to, rem);
            }
        };
        self
    }

    fn apply(mut self, actions: &Vec<Action>) -> Self {
        for action in actions {
            self = self.apply_action(action)
        }

        self
    }
}

impl ProgramState {
    fn new(program: Program) -> Self {
        Self { out: None, program }
    }

    fn exe(&mut self, vm: &mut VirtualMachine) -> Result<(), ExecutionError> {
        match vm.exe(&self.program) {
            Ok(o) => {
                self.out = Some(o);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn applying(&self, actions: &Vec<Action>) -> Self {
        let mut new = self.clone();
        new.program = new.program.apply(actions);
        new
    }

    fn requires_exe(&self) -> bool {
        self.out.is_none()
    }

    fn is_correct(&self, real: &Vec<String>) -> bool {
        let Some(out) = &self.out else {
            unreachable!("Attempted to eval an unexecuted program")
        };

        if out.1.len() != real.len() {
            return false;
        }
        for (x, y) in real.iter().zip(out.1.iter()) {
            if x != y {
                return false;
            }
        }
        return true;
    }

    fn is_more_optimal(&self, real: usize) -> bool {
        let Some(out) = &self.out else {
            unreachable!("Attempted to eval an unexecuted program")
        };
        out.0 < real
    }

    pub fn reward(&self, real: &(usize, Vec<String>)) -> isize {
        let Some(out) = &self.out else {
            unreachable!("Attempted to eval an unexecuted program")
        };

        if self.is_more_optimal(real.0) && self.is_correct(&real.1) {
            return (real.0 as isize) - (out.0 as isize);
        } else {
            return -100;
        }
    }

    pub fn next_moves(&self) -> Vec<Action> {
        let mut new_moves = vec![];

        new_moves.extend(
            (0..self.program.len())
                .map(|idx| Action::Remove(idx))
                .collect::<Vec<Action>>(),
        );
        // Replacements
        // new_moves.extend(
        //     (0..self.program.len())
        //         .flat_map(|idx| {
        //             self.program
        //                 .get(idx)
        //                 .unwrap()
        //                 .code()
        //                 .instruction_replacements()
        //                 .into_iter()
        //                 .map(|rep| Action::Replace(idx, rep))
        //                 .collect::<Vec<Action>>()
        //         })
        //         .collect::<Vec<Action>>(),
        // );

        // Additions
        //
        // come on this is a mess
        // This is probally the wors
        // code i have ever written and
        // i will change it soon. Please
        // do not look at this and instead
        // go directly to the end of
        // this code block
        // let mut vec_instr = vec![];
        // for a in 0..4 {
        //     for b in 0..4 {
        //         for c in 0..4 {
        //             for d in 0..4 {
        //                 for e in 0..2 {
        //                     for f in 0..2 {
        //                         vec_instr.push(Instruction::VecAdd {
        //                             a1r: a,
        //                             b1r: b,
        //                             r1: e,
        //                             a2r: c,
        //                             b2r: d,
        //                             r2: f,
        //                         });
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
        // new_moves.extend(
        //     (0..self.program.len())
        //         .flat_map(|x| {
        //             vec_instr
        //                 .iter()
        //                 .map(|y| Action::Add(x, y.clone()))
        //                 .collect::<Vec<Action>>()
        //         })
        //         .collect::<Vec<Action>>(),
        // );

        // Shifting around
        // new_moves.extend(
        //     (0..self.program.len())
        //         .flat_map(|from| {
        //             (0..self.program.len())
        //                 .map(|to| Action::Move(from, to))
        //                 .collect::<Vec<Action>>()
        //         })
        //         .collect::<Vec<Action>>(),
        // );

        new_moves
    }
}

struct Node {
    action: Action,
    visits: u32,
    wins: u32,
    children: HashMap<Action, Node>,
}

impl Node {
    fn new(action: Action) -> Self {
        Self {
            action,
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

pub fn mcts(
    program: Program,
    vm: &mut VirtualMachine,
    real: &(usize, Vec<String>),
    epochs: usize,
    rollout_count: usize,
) -> Option<(u32, Option<Program>)> {
    let mut root_program = ProgramState::new(program);
    root_program.exe(vm).expect("Error in root");
    let mut root = Node::new(Action::Nothing);
    let mut best_run = u32::MIN;
    let mut best_out: Option<(u32, Option<Program>)> = None;
    for epoch in 1..epochs {
        let run = mcts_node(&mut root, vm, real, &root_program, vec![], rollout_count);
        println!("Epoch: {}, Op amount {}", epoch, best_run);
        if run.0 > best_run {
            best_run = run.0;
            best_out = Some(run);
        }
        // if epoch % 100 == 0 {
        // }
    }
    best_out
}

fn mcts_node(
    node: &mut Node,
    vm: &mut VirtualMachine,
    real: &(usize, Vec<String>),
    base_state: &ProgramState,
    mut action_chain: Vec<Action>,
    rollout_count: usize,
) -> (u32, Option<Program>) {
    let better = if node.leaf() {
        // Expand, simulate
        // if node.state.is_none() {
        //     node.state = Some(base_state.applying(&action_chain));
        // }
        let node_state = base_state.applying(&action_chain); // can now be unwraped

        let new_states = node_state.next_moves();
        for new_state in new_states {
            if !node.children.contains_key(&new_state) {
                node.children
                    .insert(new_state.clone(), Node::new(new_state));
            }
        }
        let random_action = node.children.iter_mut().choose(&mut thread_rng()).unwrap();
        let reward = mcts_simulate(
            random_action.1,
            vm,
            real,
            base_state,
            action_chain,
            rollout_count,
        );
        (reward.0.max(0) as u32, reward.1)
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

        action_chain.push(best_child.action);
        mcts_node(
            best_child,
            vm,
            real,
            base_state,
            action_chain,
            rollout_count,
        )
    };
    node.visits += 1;
    node.wins = if better.0 >= 1 { 1 } else { 0 };

    better
}

fn mcts_simulate(
    node: &mut Node,
    vm: &mut VirtualMachine,
    real: &(usize, Vec<String>),
    base_state: &ProgramState,
    action_chain: Vec<Action>,
    rollout_count: usize,
) -> (isize, Option<Program>) {
    // if node.state.is_none() {
    //     node.state = Some(base_state.applying(&action_chain));
    // }
    let mut rollout_state = base_state.applying(&action_chain);
    let mut max_reward = isize::MIN;

    let mut max_program: Option<Program> = None;
    for _ in 0..rollout_count {
        let next_states = rollout_state.next_moves();
        if next_states.is_empty() {
            break;
        }

        let next_state = next_states.choose(&mut thread_rng()).unwrap().clone();

        rollout_state = rollout_state.applying(&vec![next_state]);

        if rollout_state.exe(vm).is_err() {
            continue;
        }
        let new_r = rollout_state.reward(real);
        if new_r > max_reward && new_r > 0 {
            max_reward = new_r;
            max_program = Some(rollout_state.program.clone());
        } else if new_r > max_reward {
            max_reward = new_r
        }
    }

    (max_reward, max_program)
}
