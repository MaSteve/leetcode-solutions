pub fn reduction_operations(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut last = i32::MAX;
    let mut sol = 0;
    for (i, n) in nums.iter().rev().enumerate() {
        if *n != last {
            sol += i as i32;
            last = *n;
        }
    }
    sol
}

fn main() {}
