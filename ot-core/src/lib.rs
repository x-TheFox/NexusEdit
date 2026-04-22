use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operation {
    Insert { pos: usize, char: char },
    Delete { pos: usize },
    Retain { count: usize }
}

pub fn transform(op1: Operation, op2: Operation) -> Operation {
    match (op1, op2) {
        (Operation::Insert { pos: p1, char: c1 }, Operation::Insert { pos: p2, char: _c2 }) => {
            if p1 >= p2 {
                Operation::Insert { pos: p1 + 1, char: c1 }
            } else {
                Operation::Insert { pos: p1, char: c1 }
            }
        },
        (op1, _) => op1 // Simplified for basic case
    }
}