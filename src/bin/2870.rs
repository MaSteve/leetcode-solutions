use std::collections::HashMap;

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut sol = 0;
    let mut accum: HashMap<i32, i32> = HashMap::new();
    for n in nums {
        if let Some(c) = accum.get_mut(&n) {
            *c += 1;
        } else {
            accum.insert(n, 1);
        }
    }
    for (_, c) in accum {
        if c == 1 {
            return -1;
        }
        sol += c / 3;
        if c % 3 != 0 {
            sol += 1;
        }
    }
    sol
}

fn main() {}
