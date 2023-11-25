pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
    let sum: i32 = nums.iter().sum();
    nums.iter()
        .enumerate()
        .scan((sum, 0), |(s1, s2), (i, v)| {
            let res = *s1 - (nums.len() as i32 - 2 * i as i32) * v - *s2;
            *s1 -= v;
            *s2 += v;
            Some(res)
        })
        .collect()
}

fn main() {}
