use std::cmp::max;
use std::collections::HashMap;

fn solve(data: &Vec<(i32, i32, i32)>, idx: usize, dp: &mut HashMap<usize, i32>) -> i32 {
    if idx >= data.len() {
        0
    } else if let Some(sol) = dp.get(&idx) {
        *sol
    } else {
        let next_idx = data.binary_search(&(data[idx].1, -1, -1)).unwrap_err();
        let mut sol = data[idx].2 + solve(data, next_idx, dp);
        sol = max(sol, solve(data, idx + 1, dp));
        dp.insert(idx, sol);
        sol
    }
}

pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let mut data: Vec<(i32, i32, i32)> = (0..start_time.len())
        .into_iter()
        .map(|i| (start_time[i], end_time[i], profit[i]))
        .collect();
    data.sort_unstable();
    solve(&data, 0, &mut HashMap::new())
}

fn main() {}
