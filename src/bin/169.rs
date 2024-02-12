pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut candidate = 0;
    let mut count = 0;
    for n in nums {
        if count == 0 {
            candidate = n;
            count = 1;
        } else if n == candidate {
            count += 1;
        } else {
            count -= 1;
        }
    }
    candidate
}

fn main() {}
