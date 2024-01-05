pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut length = 0;
    let mut lis_map = vec![0; nums.len()];
    for i in 0..nums.len() {
        let mut low = 0;
        let mut high = length;
        while low < high {
            let mid = low + ((high - low) / 2);
            if nums[lis_map[mid]] >= nums[i] {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        lis_map[low] = i;
        if low + 1 > length {
            length = low + 1;
        }
    }
    length as i32
}

fn main() {}
