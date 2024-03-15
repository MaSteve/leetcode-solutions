pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut sol = vec![1; nums.len()];
    for i in 1..nums.len() {
        sol[i] = sol[i - 1] * nums[i - 1];
    }
    let mut prod = 1;
    for i in (0..nums.len() - 1).rev() {
        prod *= nums[i + 1];
        sol[i] *= prod;
    }
    sol
}

fn main() {}
