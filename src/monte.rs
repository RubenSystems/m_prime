/*
import random

class MonteCarloRollout:
    def __init__(self, env, num_rollouts=100, max_steps=100):
        self.env = env
        self.num_rollouts = num_rollouts
        self.max_steps = max_steps

    def rollout(self, state, action):
        total_reward = 0.0
        for _ in range(self.num_rollouts):
            total_reward += self.simulate(state, action)
        return total_reward / self.num_rollouts

    def simulate(self, state, action):
        total_reward = 0.0
        current_state = state
        for _ in range(self.max_steps):
            next_state, reward, done, _ = self.env.step(action)
            total_reward += reward
            if done:
                break
            action = self.select_random_action()  # Choose a random action
        self.env.reset()  # Reset the environment for the next rollout
        return total_reward

    def select_random_action(self):
        return random.choice(range(self.env.action_space.n))
*/

use rand::{rngs::ThreadRng, Rng};

use crate::{
    actions::Actions,
    program::{Instruction, Program},
};

pub struct MonteTreeNode {
    state: Program,
    visits: usize,
    wins: usize,
    children: Vec<MonteTreeNode>,
}

impl MonteTreeNode {
    pub fn new(state: Program) -> Self {
        Self {
            state,
            visits: 0,
            wins: 0,
            children: Vec::new(),
        }
    }
}

pub struct Monte {
    rng: ThreadRng,
    required_out: Vec<String>,
    root: MonteTreeNode,
    legal_operations: Vec<Instruction>,
}

impl Monte {
    pub fn new(
        legal_operations: Vec<Instruction>,
        start: Program,
        required_out: Vec<String>,
    ) -> Self {
        let rng = rand::thread_rng();
        Self {
            rng,
            legal_operations,
            required_out,
            root: MonteTreeNode::new(start),
        }
    }

    fn single_rollout(&mut self, mut prev: Program) -> Program {
        let na = self.new_action(&prev);
        match na {
            Actions::Remove(index) => _ = prev.remove(index),
            Actions::Add(index, action) => _ = prev.insert(index, action),
        };

        prev
    }

    fn random(&mut self, start: usize, end: usize) -> usize {
        self.rng.gen_range(start..end)
    }

    fn new_action(&mut self, program: &Program) -> Actions {
        match self.random(0, 2) {
            0 => Actions::Remove(self.random(0, program.len())),
            1 => {
                let injection_point = self.random(0, program.len());
                let new_action_index = self.random(0, self.legal_operations.len());
                let action = self.legal_operations[new_action_index];
                Actions::Add(injection_point, action)
            }
            _ => unreachable!("What! You tried to do something that is uncool"),
        }
    }
}
