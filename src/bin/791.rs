use std::collections::HashMap;

pub fn custom_sort_string(order: String, s: String) -> String {
    let mut order: HashMap<char, usize> =
        order.chars().enumerate().map(|(idx, c)| (c, idx)).collect();
    let mut next_idx = order.len();
    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        if order.contains_key(&c) {
            continue;
        }
        order.insert(c, next_idx);
        next_idx += 1;
    }
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable_by_key(|c| order.get(c).unwrap());
    chars.iter().collect()
}

fn main() {}
