pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    let (pos, neg): (Vec<i32>, Vec<i32>) = nums.iter().partition(|n| **n > 0);
    pos.iter().zip(neg).flat_map(|(p, n)| [*p, n]).collect()
}

fn main() {}
