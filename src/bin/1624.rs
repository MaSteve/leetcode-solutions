use std::cmp::max;
use std::collections::HashMap;

pub fn max_length_between_equal_characters(s: String) -> i32 {
    let mut sol = -1;
    let mut first_char: HashMap<char, usize> = HashMap::new();
    for (i, c) in s.chars().enumerate() {
        if let Some(idx) = first_char.get(&c) {
            sol = max(sol, i as i32 - *idx as i32 - 1);
        } else {
            first_char.insert(c, i);
        }
    }
    sol
}

fn main() {}
