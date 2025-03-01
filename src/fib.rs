use num_bigint::BigUint;
use std::collections::HashMap;

pub fn fibonacci(n: u32, memo: &mut HashMap<u32, BigUint>) -> BigUint {
    if let Some(result) = memo.get(&n) {
        return result.clone(); 
    }

    let result = match n {
        0 => BigUint::from(0u32),
        1 => BigUint::from(1u32),
        _ => fibonacci(n - 1, memo) + fibonacci(n - 2, memo),
    };

    memo.insert(n, result.clone()); 
    result
}