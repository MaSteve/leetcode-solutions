pub fn max_product_difference(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    (nums[nums.len() - 1] * nums[nums.len() - 2]) - (nums[1] * nums[0])
}

fn main() {}
