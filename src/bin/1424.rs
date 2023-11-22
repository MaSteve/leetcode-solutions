use std::cmp::min;

pub fn find_diagonal_order2(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut sol = vec![];
    let mut updated = true;
    let mut i = 1;
    while updated {
        updated = false;
        for j in (0..min(i, nums.len())).rev() {
            let p = i - j - 1;
            if p < nums[j].len() {
                sol.push(nums[j][p]);
                updated = true;
            }
        }
        i += 1;
    }
    sol
}

pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut nums: Vec<(i64, i32)> = nums
        .iter()
        .enumerate()
        .flat_map(|(j, v)| {
            v.iter().enumerate().map(move |(i, n)| {
                let i = i as i64;
                let j = j as i64;
                ((i + 1) * (i + 2) / 2 + j * (j + 1) / 2 + j * i, *n)
            })
        })
        .collect();
    nums.sort_unstable();
    nums.iter().map(|(_, n)| *n).collect()
}

fn main() {}
