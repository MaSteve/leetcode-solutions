pub fn count_homogenous(s: String) -> i32 {
    let mut last_char = '-';
    let mut length = 0;
    let mut sol = 0;
    for c in s.chars() {
        if c == last_char {
            length += 1;
        } else {
            last_char = c;
            length = 1;
        }
        sol += length;
        sol %= 1000000007
    }
    sol
}

fn main() {}
