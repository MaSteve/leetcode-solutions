fn check(v: &[i32]) -> bool {
    let mut v = v.to_vec();
    v.sort_unstable();
    let diff = v[1] - v[0];
    let mut last = v[1];
    for i in 2..v.len() {
        if v[i] - last != diff {
            return false;
        }
        last = v[i];
    }
    true
}

pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
    l.iter()
        .zip(r.iter())
        .map(|(l, r)| check(&nums[*l as usize..=*r as usize]))
        .collect()
}

fn main() {}
