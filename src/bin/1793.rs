use std::cmp::max;

pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
    let mut i = k as usize;
    let mut j = k as usize;
    let mut current_min = nums[k as usize];
    let mut sol = nums[k as usize];
    while !(i == 0 && j == nums.len() - 1) {
        while i > 0 && nums[i - 1] >= current_min {
            i -= 1;
        }
        while j < nums.len() - 1 && nums[j + 1] >= current_min {
            j += 1;
        }
        sol = max(sol, (j as i32 - i as i32 + 1) * current_min);
        if i > 0 && j < nums.len() - 1 {
            current_min = max(nums[i - 1], nums[j + 1]);
        } else if i > 0 {
            current_min = nums[i - 1];
        } else if j < nums.len() - 1 {
            current_min = nums[j + 1];
        }
    }
    sol
}

fn main() {}
