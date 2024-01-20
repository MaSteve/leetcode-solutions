pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    const MOD: i64 = 1000000007;
    let mut prev_min = Vec::with_capacity(arr.len() + 1);
    prev_min.push((0, -1, 0));
    let mut sol = 0;
    for (idx, &v) in arr.iter().enumerate() {
        while let Some(last) = prev_min.last() {
            if v <= last.0 {
                prev_min.pop();
            } else {
                let incr = (v as i64 * (idx as i64 - last.1) + last.2) % MOD;
                sol += incr;
                sol %= MOD;
                prev_min.push((v, idx as i64, incr));
                break;
            }
        }
    }
    sol as i32
}

fn main() {}
