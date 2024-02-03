fn solve(arr: &Vec<i32>, idx: usize, k: i32, dp: &mut [i32; 500]) -> i32 {
    if idx >= arr.len() {
        return 0;
    }
    if dp[idx] != -1 {
        return dp[idx];
    }
    dp[idx] = 0;
    let mut maxi = -1;
    let lim = arr.len().min(idx + k as usize);
    for i in idx..lim {
        maxi = maxi.max(arr[i]);
        dp[idx] = dp[idx].max(maxi * (i - idx + 1) as i32 + solve(arr, i + 1, k, dp));
    }
    dp[idx]
}

pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
    solve(&arr, 0, k, &mut [-1; 500])
}

fn main() {}
