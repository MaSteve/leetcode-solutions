pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut duplicate = -1;
    let mut set = vec![false; nums.len() + 1];
    let mut sum = 0;
    for n in &nums {
        if set[*n as usize] {
            duplicate = *n;
        } else {
            set[*n as usize] = true;
            sum += n;
        }
    }
    vec![
        duplicate,
        ((nums.len() as i32 + 1) * nums.len() as i32) / 2 - sum,
    ]
}

fn main() {}
