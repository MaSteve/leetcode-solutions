use std::collections::HashMap;

const MOD: i64 = 1000000007;

pub fn solve(v: i32, arr: &Vec<i32>, dp: &mut HashMap<i32, i32>) -> i32 {
    let mut sol: i64 = 1;
    for v2 in arr {
        if *v2 as f64 <= (v as f64).sqrt() {
            if v % v2 == 0 {
                let v3 = v / v2;
                if dp.contains_key(&v3) {
                    if *v2 == v3 {
                        sol += (dp[v2] as i64 * dp[&v3] as i64) % MOD;
                    } else {
                        sol += (2 * dp[v2] as i64 * dp[&v3] as i64) % MOD;
                    }
                    sol %= MOD;
                }
            }
        } else {
            break;
        }
    }
    dp.insert(v, sol as i32);
    sol as i32
}

pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
    arr.sort();
    let mut dp: HashMap<i32, i32> = HashMap::new();
    for v in &arr {
        dp.insert(*v, -1);
    }
    let mut sol = 0;
    for v in &arr {
        sol += solve(*v, &arr, &mut dp);
        sol %= MOD as i32;
    }
    sol
}

fn main() {}
