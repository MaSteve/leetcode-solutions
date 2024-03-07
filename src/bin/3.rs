pub fn length_of_longest_substring(s: String) -> i32 {
    let mut sol = 0;
    let mut last = 0;
    let mut count = [0; 128];
    let s = s.as_bytes();
    for i in 0..s.len() {
        count[s[i] as usize] += 1;
        while count[s[i] as usize] > 1 {
            count[s[last] as usize] -= 1;
            last += 1;
        }
        sol = sol.max(i - last + 1);
    }
    sol as i32
}

fn main() {}
