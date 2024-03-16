use std::collections::HashMap;

pub fn find_max_length(nums: Vec<i32>) -> i32 {
    let mut ratio = 0;
    let mut ratio_map: HashMap<i32, Vec<i32>> = HashMap::new();
    ratio_map.entry(0).or_default().push(-1);
    for (idx, v) in nums.iter().enumerate() {
        if *v == 1 {
            ratio += 1;
        } else {
            ratio -= 1;
        }
        ratio_map.entry(ratio).or_default().push(idx as i32);
    }
    let mut sol = 0;
    for (_, idxs) in &ratio_map {
        let first = idxs.first().unwrap();
        let last = idxs.last().unwrap();
        sol = sol.max(last - first);
    }
    sol as i32
}

fn main() {}
