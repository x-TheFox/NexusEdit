use ot_core::{Operation, transform};
use rand::Rng;

#[test]
fn fuzz_test_random_operations() {
    let mut rng = rand::thread_rng();
    
    for _ in 0..100 {
        let op1 = generate_random_operation(&mut rng);
        let op2 = generate_random_operation(&mut rng);
        
        // Just verify it doesn't panic
        let _transformed = transform(op1, op2);
    }
}

fn generate_random_operation(rng: &mut impl Rng) -> Operation {
    match rng.gen_range(0..3) {
        0 => Operation::Insert { 
            pos: rng.gen_range(0..100), 
            ch: (rng.gen_range(65..91) as u8) as char 
        },
        1 => Operation::Delete { 
            pos: rng.gen_range(0..100) 
        },
        _ => Operation::Retain { 
            count: rng.gen_range(0..10) 
        },
    }
}
