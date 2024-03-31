pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
    let mut sol = 0;
    let mut count_min = 0;
    let mut count_max = 0;
    let mut start = 0;
    let mut end = 0;
    let mut min_max_reached = false;
    for (i, v) in nums.iter().enumerate() {
        if *v < min_k || *v > max_k {
            count_min = 0;
            count_max = 0;
            start = i + 1;
            end = i + 1;
            min_max_reached = false;
        } else {
            if *v == min_k || *v == max_k {
                if *v == min_k {
                    count_min += 1;
                }
                if *v == max_k {
                    count_max += 1;
                }
                if count_max > 0 && count_min > 0 {
                    min_max_reached = true;
                }
            }
            while count_min > 0 && count_max > 0 {
                if nums[end] == min_k {
                    count_min -= 1;
                }
                if nums[end] == max_k {
                    count_max -= 1;
                }
                end += 1;
            }
            if min_max_reached {
                sol += (end - start) as i64;
            }
        }
    }
    sol
}

fn main() {}
