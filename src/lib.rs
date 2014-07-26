#![feature(phase)]

extern crate contenttype;

use std::collections::HashMap;

pub struct Accepts {
    priorities: HashMap<String, f64>
}

impl Accepts {
    pub fn new(accept_encoding: &str) -> Accepts {
        Accepts {
            priorities: to_priorities(accept_encoding)
        }
    }

    pub fn accepts<'a>(&self, possibilities: &[&'a str]) -> Option<&'a str> {
        possibilities.iter().fold((None, -1f64), |best, possibility| {
            match self.priorities.find(&possibility.to_string()) {
                Some(&priority) if priority > best.val1() => {
                        (Some(possibility), priority)
                },
                _ => best
            }
        }).val0().map(|&b| b)
    }
}

fn to_priorities(accept_encoding: &str) -> HashMap<String, f64> {
    fail!()
}

