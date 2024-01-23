use std::collections::HashSet;

fn solve(arr: &Vec<HashSet<char>>, idx: usize, bucket: &mut HashSet<char>) -> i32 {
    if idx >= arr.len() {
        return 0;
    }
    let mut sol = solve(arr, idx + 1, bucket);
    if bucket.is_disjoint(&arr[idx]) {
        for c in &arr[idx] {
            bucket.insert(*c);
        }
        sol = sol.max(solve(arr, idx + 1, bucket) + arr[idx].len() as i32);
        for c in &arr[idx] {
            bucket.remove(&c);
        }
    }
    sol
}

pub fn max_length(arr: Vec<String>) -> i32 {
    solve(
        &arr.iter()
            .map(|s| {
                let mut bucket = HashSet::new();
                for c in s.chars() {
                    if bucket.contains(&c) {
                        return HashSet::new();
                    }
                    bucket.insert(c);
                }
                bucket
            })
            .filter(|b| !b.is_empty())
            .collect(),
        0,
        &mut HashSet::new(),
    )
}

fn main() {}
