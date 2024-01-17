use std::collections::{HashMap, HashSet};

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut counters = HashMap::new();
    for v in arr {
        *counters.entry(v).or_insert(0) += 1;
    }
    let mut frecuencies = HashSet::new();
    for (_, f) in &counters {
        if frecuencies.contains(&f) {
            return false;
        }
        frecuencies.insert(f);
    }
    frecuencies.len() == counters.len()
}

fn main() {}
