pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];
    for (i, c1) in text1.bytes().enumerate().rev() {
        for (j, c2) in text2.bytes().enumerate().rev() {
            if c1 == c2 {
                dp[i][j] = 1 + dp[i + 1][j + 1];
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }

    dp[0][0]
}

fn main() {}
