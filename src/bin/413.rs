pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
        return 0;
    }
    let mut sol = 0;
    let mut diff = nums[1] - nums[0];
    let mut consecutive = 1;
    for i in 2..nums.len() {
        let current_diff = nums[i] - nums[i - 1];
        if current_diff == diff {
            sol += consecutive;
            consecutive += 1;
        } else {
            diff = current_diff;
            consecutive = 1;
        }
    }
    sol
}

fn main() {}
