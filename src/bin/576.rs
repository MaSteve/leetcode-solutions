pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
    if max_move == 0 {
        return 0;
    }
    const MOD: i64 = 1000000007;
    let mut dp = vec![vec![vec![0; n as usize]; m as usize]; max_move as usize + 1];
    for mov in 1..=max_move as usize {
        for i in 0..m as usize {
            for j in 0..n as usize {
                if i == 0 {
                    dp[mov][i][j] += 1;
                } else {
                    dp[mov][i][j] += dp[mov - 1][i - 1][j];
                }
                if i == m as usize - 1 {
                    dp[mov][i][j] += 1;
                    dp[mov][i][j] %= MOD;
                } else {
                    dp[mov][i][j] += dp[mov - 1][i + 1][j];
                    dp[mov][i][j] %= MOD;
                }

                if j == 0 {
                    dp[mov][i][j] += 1;
                    dp[mov][i][j] %= MOD;
                } else {
                    dp[mov][i][j] += dp[mov - 1][i][j - 1];
                    dp[mov][i][j] %= MOD;
                }
                if j == n as usize - 1 {
                    dp[mov][i][j] += 1;
                    dp[mov][i][j] %= MOD;
                } else {
                    dp[mov][i][j] += dp[mov - 1][i][j + 1];
                    dp[mov][i][j] %= MOD;
                }
            }
        }
    }

    dp[max_move as usize][start_row as usize][start_column as usize] as i32
}

fn main() {}
