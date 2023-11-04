use std::cmp::max;

pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut sol = 0;
    for v in right {
        sol = max(sol, n - v);
    }
    for v in left {
        sol = max(sol, v);
    }
    sol
}

fn main() {}
