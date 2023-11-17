use std::cmp::max;

pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut sol = 0;
    for i in 0..nums.len() / 2 {
        sol = max(sol, nums[i] + nums[nums.len() - 1 - i]);
    }
    sol
}

fn main() {}
