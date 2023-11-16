pub fn find_different_binary_string(nums: Vec<String>) -> String {
    let mut sol = String::new();
    for (i, n) in nums.iter().enumerate() {
        if n.chars().nth(i).unwrap() == '1' {
            sol.push('0');
        } else {
            sol.push('1');
        }
    }
    sol
}

fn main() {}
