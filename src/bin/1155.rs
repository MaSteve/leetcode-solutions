use std::collections::HashMap;

fn solve(n: i32, k: i32, target: i32, dp: &mut HashMap<(i32, i32), i64>) -> i64 {
    if target < n || target > n * k {
        0
    } else if n == 1 {
        1
    } else if let Some(sol) = dp.get(&(n, target)) {
        *sol
    } else {
        let mut sol = 0;
        for i in 1..=k {
            sol += solve(n - 1, k, target - i, dp);
            sol %= 1000000007
        }
        dp.insert((n, target), sol);
        sol
    }
}

pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    solve(n, k, target, &mut HashMap::new()) as i32
}

fn main() {}
