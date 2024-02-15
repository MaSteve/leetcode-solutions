pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
    nums.sort_unstable();
    let mut sol = -1;
    let mut perimeter = nums[0] as i64 + nums[1] as i64;
    for i in 2..nums.len() {
        let current_perimeter = perimeter;
        perimeter += nums[i] as i64;
        if current_perimeter > nums[i] as i64 {
            sol = perimeter;
        }
    }
    sol
}

fn main() {}
