use ot_core::*;

#[test]
fn test_insert_transform() {
    let op1 = Operation::Insert { pos: 5, ch: 'A' };
    let op2 = Operation::Insert { pos: 5, ch: 'B' };
    
    let transformed = transform(op1, op2);
    assert_eq!(transformed, Operation::Insert { pos: 6, ch: 'A' });
}

#[test]
fn test_insert_transform_unshifted() {
    let op1 = Operation::Insert { pos: 3, ch: 'A' };
    let op2 = Operation::Insert { pos: 5, ch: 'B' };
    
    let transformed = transform(op1, op2);
    assert_eq!(transformed, Operation::Insert { pos: 3, ch: 'A' });
}

#[test]
fn test_delete_transform() {
    let op1 = Operation::Delete { pos: 5 };
    let op2 = Operation::Insert { pos: 3, ch: 'B' };
    let transformed = transform(op1, op2);
    assert_eq!(transformed, Operation::Delete { pos: 6 });
}

#[test]
fn test_overlapping_delete() {
    let op1 = Operation::Delete { pos: 5 };
    let op2 = Operation::Delete { pos: 5 };
    let transformed = transform(op1, op2);
    assert_eq!(transformed, Operation::Retain { count: 0 });
}

