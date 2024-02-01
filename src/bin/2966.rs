pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    if nums.chunks(3).all(|chunk| chunk[2] - chunk[0] <= k) {
        nums.chunks(3).map(|chunk| chunk.to_vec()).collect()
    } else {
        vec![]
    }
}

fn main() {}
