pub fn num_squares(n: i32) -> i32 {
    let mut dp = vec![i32::MAX; n as usize + 1];
    for i in 1..=100 {
        let v = i * i;
        if v > n {
            break;
        }
        dp[(i * i) as usize] = 1;
    }
    for i in 1..n {
        let res = dp[i as usize] + 1;
        for j in 1..=100 {
            let v = i + j * j;
            if v > n {
                break;
            }
            dp[v as usize] = dp[v as usize].min(res);
        }
    }
    dp[n as usize]
}

fn main() {}
