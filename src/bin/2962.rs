pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let mut sol = 0;
    let maxi = *nums.iter().max().unwrap();
    let mut start = 0;
    let mut accum = 0;
    let mut k_reached = false;
    for v in nums.iter() {
        if *v == maxi {
            accum += 1;
            if accum == k {
                k_reached = true;
            }
        }
        while accum >= k {
            if nums[start] == maxi {
                accum -= 1;
            }
            start += 1;
        }
        if k_reached {
            sol += start as i64;
        }
    }
    sol
}

fn main() {}
