use std::{collections::HashMap, vec};

const MOD: i32 = 1000000007;

fn solve(n: i32, restriction: i32, dp: &mut HashMap<(i32, i32), i32>) -> i32 {
    let options = match restriction {
        0 => vec!['a', 'e', 'i', 'o', 'u'],
        1 => vec!['e'],
        2 => vec!['a', 'i'],
        3 => vec!['a', 'e', 'o', 'u'],
        4 => vec!['i', 'u'],
        5 => vec!['a'],
        _ => vec![],
    };
    if n == 1 {
        return options.len() as i32;
    } else if let Some(sol) = dp.get(&(n, restriction)) {
        return *sol;
    }
    let mut sol = 0;
    for option in options {
        let restriction = match option {
            'a' => 1,
            'e' => 2,
            'i' => 3,
            'o' => 4,
            'u' => 5,
            _ => -1,
        };
        sol += solve(n - 1, restriction, dp);
        sol %= MOD;
    }
    dp.insert((n, restriction), sol);
    sol
}

pub fn count_vowel_permutation(n: i32) -> i32 {
    solve(n, 0, &mut HashMap::new())
}

fn main() {}
