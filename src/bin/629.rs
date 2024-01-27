pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
    const MOD: i32 = 1000000007;
    let mut dp = vec![vec![0; k as usize + 1]; n as usize + 1];
    dp[1][0] = 1;
    for i in 2..=n as usize {
        let mut sum = 0;
        for j in 0..=k as usize {
            sum += dp[i - 1][j];
            if sum >= MOD {
                sum -= MOD;
            }
            if j >= i {
                sum -= dp[i - 1][j - i];
                if sum < 0 {
                    sum += MOD;
                }
            }
            dp[i][j] = sum;
        }
    }
    dp[n as usize][k as usize]
}

fn main() {}
