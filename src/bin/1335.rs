use std::cmp::{max, min};

pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
    if (job_difficulty.len() as i32) < d {
        return -1;
    }
    let mut dp = vec![vec![20000; d as usize]; job_difficulty.len()];
    let mut maxi = -1;
    for job in (0..job_difficulty.len()).rev() {
        maxi = max(maxi, job_difficulty[job]);
        dp[job][d as usize - 1] = maxi;
    }
    for day in (0..d as usize - 1).rev() {
        for job in (0..job_difficulty.len()).rev().skip(d as usize - 1 - day) {
            let mut maxi = -1;
            let mut sol = 20000;
            for i in job..job_difficulty.len() - (d as usize - 1 - day) {
                maxi = max(maxi, job_difficulty[i]);
                sol = min(sol, dp[i + 1][day + 1] + maxi);
            }
            dp[job][day] = sol;
        }
    }
    dp[0][0]
}

fn main() {}
