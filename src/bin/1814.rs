use std::collections::HashMap;

#[inline(always)]
fn rev(mut n: i32) -> i32 {
    let mut res = 0;
    while n > 0 {
        res = 10 * res + n % 10;
        n /= 10;
    }
    res
}

pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
    let mut buckets = HashMap::new();
    let mut sol: i64 = 0;
    for n in nums {
        sol += *buckets
            .entry(n - rev(n))
            .and_modify(|c| *c += 1)
            .or_insert(0);
    }
    (sol % 1000000007) as i32
}

fn main() {}
