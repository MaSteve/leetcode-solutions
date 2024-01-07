use std::collections::HashMap;

pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let mut sol = 0;
    let mut dp: HashMap<(usize, i64), i32> = HashMap::new();
    for i in 1..nums.len() {
        for j in 0..i {
            let diff = nums[j] as i64 - nums[i] as i64;
            let subsequences = *dp.get(&(j, diff)).unwrap_or(&0);
            if let Some(accum) = dp.get_mut(&(i, diff)) {
                *accum += subsequences + 1;
            } else {
                dp.insert((i, diff), subsequences + 1);
            }
            sol += subsequences;
        }
    }
    sol
}

fn main() {}
