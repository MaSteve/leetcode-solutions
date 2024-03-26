pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    nums.iter_mut().for_each(|v| {
        if *v <= 0 {
            *v = 10001;
        }
    });
    for i in 0..nums.len() {
        let v = (nums[i].abs() - 1) as usize;
        if v < nums.len() && nums[v] > 0 {
            nums[v] = -nums[v];
        }
    }
    for i in 0..nums.len() {
        if nums[i] > 0 {
            return (i + 1) as i32;
        }
    }
    (nums.len() + 1) as i32
}

fn main() {}
