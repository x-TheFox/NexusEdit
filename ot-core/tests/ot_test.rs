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
