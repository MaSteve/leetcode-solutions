use std::cmp::max;
use std::collections::HashMap;

fn solve(
    nums1: &Vec<i32>,
    nums2: &Vec<i32>,
    p1: usize,
    p2: usize,
    allow_zero: bool,
    dp: &mut HashMap<(usize, usize, bool), i32>,
) -> i32 {
    if p1 >= nums1.len() || p2 >= nums2.len() {
        return 0;
    }
    if let Some(sol) = dp.get(&(p1, p2, allow_zero)) {
        return *sol;
    }
    let mut sol = nums1[p1] * nums2[p2] + solve(nums1, nums2, p1 + 1, p2 + 1, true, dp);
    if p1 < nums1.len() - 1 || allow_zero {
        sol = max(sol, solve(nums1, nums2, p1 + 1, p2, allow_zero, dp));
    }
    if p2 < nums2.len() - 1 || allow_zero {
        sol = max(sol, solve(nums1, nums2, p1, p2 + 1, allow_zero, dp));
    }
    dp.insert((p1, p2, allow_zero), sol);
    sol
}

pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    solve(&nums1, &nums2, 0, 0, false, &mut HashMap::new())
}

fn main() {}
