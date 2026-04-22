use serde::{Deserialize, Serialize};

/// Represents an operational transformation operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operation {
    /// Inserts a character at the specified position.
    Insert { pos: usize, ch: char },
    /// Deletes a character at the specified position.
    Delete { pos: usize },
    /// Retains a specified number of characters.
    Retain { count: usize }
}

/// Transforms `op1` against `op2` such that `op1` can be applied after `op2`.
pub fn transform(op1: Operation, op2: Operation) -> Operation {
    match (op1, op2) {
        (Operation::Insert { pos: p1, ch: c1 }, Operation::Insert { pos: p2, ch: _c2 }) => {
            if p1 >= p2 {
                Operation::Insert { pos: p1 + 1, ch: c1 }
            } else {
                Operation::Insert { pos: p1, ch: c1 }
            }
        },
        (op1, _) => op1 // Simplified for basic case
    }
}