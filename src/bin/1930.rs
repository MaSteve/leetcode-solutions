use std::collections::{HashMap, HashSet};

struct PalindromicInfo {
    first: usize,
    last: usize,
    chars: HashSet<char>,
}

pub fn count_palindromic_subsequence(s: String) -> i32 {
    let mut palindromes: HashMap<char, PalindromicInfo> = HashMap::new();
    for (i, c) in s.chars().enumerate() {
        if let Some(pal_info) = palindromes.get_mut(&c) {
            pal_info.last = i;
        } else {
            palindromes.insert(
                c,
                PalindromicInfo {
                    first: i,
                    last: i,
                    chars: HashSet::new(),
                },
            );
        }
    }
    for (i, c) in s.chars().enumerate() {
        for (_, pal_info) in palindromes.iter_mut() {
            if i > pal_info.first && i < pal_info.last {
                pal_info.chars.insert(c);
            }
        }
    }
    palindromes
        .iter()
        .fold(0, |acc, (_, pal_info)| acc + pal_info.chars.len() as i32)
}

fn main() {}
