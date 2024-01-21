pub fn rob(mut nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        nums[0]
    } else {
        nums[1] = nums[1].max(nums[0]);
        for i in 2..nums.len() {
            nums[i] = nums[i - 1].max(nums[i - 2] + nums[i]);
        }
        *nums.last().unwrap()
    }
}

fn main() {}
