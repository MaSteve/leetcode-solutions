pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut freq = [0; 101];
    nums.iter().for_each(|v| freq[*v as usize] += 1);
    let max_freq = freq.iter().max().unwrap();
    freq.iter().filter(|f| *f == max_freq).sum::<i32>()
}

fn main() {}
