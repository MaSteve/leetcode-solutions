use std::cmp::min;
use std::collections::HashMap;

fn compressed_len(consecutive: i32) -> i32 {
    if consecutive == 0 {
        0
    } else if consecutive == 1 {
        1
    } else if consecutive < 10 {
        2
    } else if consecutive < 100 {
        3
    } else {
        4
    }
}

fn solve(s: &String, idx: usize, k: i32, dp: &mut HashMap<(usize, i32), i32>) -> i32 {
    if idx >= s.len() {
        0
    } else if let Some(sol) = dp.get(&(idx, k)) {
        *sol
    } else {
        let mut sol = i32::MAX;
        if k > 0 {
            sol = solve(s, idx + 1, k - 1, dp);
        }
        let mut deleted = 0;
        let mut consecutive = 0;
        let c = s.chars().nth(idx).unwrap();
        for (i, c_i) in s.chars().skip(idx).enumerate() {
            if c == c_i {
                consecutive += 1;
            } else {
                deleted += 1;
                if deleted > k {
                    break;
                }
            }
            sol = min(
                sol,
                solve(s, idx + i + 1, k - deleted, dp) + compressed_len(consecutive),
            );
        }
        dp.insert((idx, k), sol);
        sol
    }
}

pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
    solve(&s, 0, k, &mut HashMap::new())
}

fn main() {}
