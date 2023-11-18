use std::cmp::{max, min};
use std::collections::BTreeMap;

pub fn max_frequency_2(nums: Vec<i32>, k: i32) -> i32 {
    let mut frequencies = BTreeMap::new();
    for n in nums {
        frequencies.entry(-n).and_modify(|f| *f += 1).or_insert(1);
    }
    let mut sol = 0;
    for (n, f) in &frequencies {
        let mut alt_sol = *f;
        let mut k = k;
        for (n2, f2) in frequencies.range(n + 1..) {
            let d = n2 - n;
            let increment = min(*f2, k / d);
            if increment == 0 {
                break;
            }
            alt_sol += increment;
            k -= d * increment;
        }
        sol = max(alt_sol, sol);
    }
    sol
}

pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let mut j = 0;
    let mut acum = 0;
    let mut sol = 0;
    for (i, v) in nums.iter().enumerate() {
        acum += v;
        while (i - j + 1) as i32 * v - acum > k {
            acum -= nums[j];
            j += 1;
        }
        sol = max(sol, i - j + 1);
    }
    sol as i32
}

fn main() {}
