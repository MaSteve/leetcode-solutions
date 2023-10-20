use std::cmp::max;

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    let mut cr = nums[0];
    for i in 1..nums.len() {
        cr = max(nums[i], cr + nums[i]);
        res = max(res, cr);
    }
    res
}

fn main() {}
