use std::collections::HashMap;

pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut sol = 0;
    let mut count = HashMap::<i32, i32>::new();
    let mut start = 0;
    for (i, v) in nums.iter().enumerate() {
        *count.entry(*v).or_default() += 1;
        while *count.get(&v).unwrap() > k {
            count.entry(nums[start]).and_modify(|c| *c -= 1);
            start += 1;
        }
        sol = sol.max(i - start + 1);
    }
    sol as i32
}

fn main() {}
