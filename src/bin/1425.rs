use std::collections::VecDeque;

pub fn constrained_subset_sum(mut nums: Vec<i32>, k: i32) -> i32 {
    let mut max_index = VecDeque::new();
    for i in 0..nums.len() {
        if let Some(j) = max_index.front() {
            nums[i] += nums[*j];
        }

        while let Some(j) = max_index.front() {
            if i as i32 - *j as i32 >= k {
                max_index.pop_front();
            } else {
                break;
            }
        }
        while let Some(j) = max_index.back() {
            if nums[i] >= nums[*j] {
                max_index.pop_back();
            } else {
                break;
            }
        }

        if nums[i] > 0 {
            max_index.push_back(i);
        }
    }
    *nums.iter().max().unwrap()
}

fn main() {}
