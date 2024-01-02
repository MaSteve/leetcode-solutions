use std::cmp::max;
use std::collections::HashMap;

pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut rows = 1;
    let mut accum = HashMap::new();
    for n in nums {
        if let Some(c) = accum.get_mut(&n) {
            *c += 1;
            rows = max(rows, *c);
        } else {
            accum.insert(n, 1);
        }
    }
    let mut sol = vec![vec![]; rows];
    for (v, c) in accum {
        for i in 0..c {
            sol[i].push(v);
        }
    }
    sol
}

fn main() {}
