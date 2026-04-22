use ot_core::*;

#[test]
fn test_insert_transform() {
    let op1 = Operation::Insert { pos: 5, char: 'A' };
    let op2 = Operation::Insert { pos: 5, char: 'B' };
    
    let transformed = transform(op1, op2);
    assert_eq!(transformed, Operation::Insert { pos: 6, char: 'A' });
}
