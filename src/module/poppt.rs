extern crate bit_vec;

use bit_vec::BitVec;
use super::{cfg::*};

#[derive(Debug)]
pub struct POPPT {pub bit: BitVec, pub label: Vec<u32>,}

impl POPPT {
    pub fn new() -> Self {
        Self {bit: BitVec::new(), label: Vec::new(),}
    }

    pub fn to_grammar(&self, g: &mut Grammar) -> () {
        let mut right_side: Vec<u32> = Vec::new();
        let mut stack: Vec<u32> = Vec::new();
        let mut var: u32 = g.terminal.len() as u32 + 1;
        let mut i = 0;
        for b in &self.bit {
            if b {
                if right_side.len() > 0 {
                    g.rule.push(right_side.iter().rev().map(|x| *x).collect::<Vec<u32>>());
                    stack.push(var);
                    var += 1;
                    right_side = Vec::new();
                }
                else {
                    stack.push(self.label[i]);
                    i += 1;
                }
            }
            else {right_side.push(stack.pop().unwrap());}
        }
        g.sequence = vec![var - 1];

    }
}


