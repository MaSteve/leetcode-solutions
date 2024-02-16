use std::collections::HashMap;

pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
    let mut count = HashMap::<i32, i32>::new();
    for n in arr {
        *count.entry(n).or_default() += 1;
    }

    let mut count: Vec<i32> = count.iter().map(|(_, c)| *c).collect();
    count.sort_unstable();
    let mut removed = 0;
    let mut sol = count.len() as i32;
    for c in count {
        removed += c;
        if removed <= k && sol >= 1 {
            sol -= 1;
        } else {
            break;
        }
    }
    sol
}

fn main() {}
