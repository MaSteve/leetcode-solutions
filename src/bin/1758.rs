use std::cmp::min;

pub fn min_operations(s: String) -> i32 {
    let mut c0 = 0;
    let mut c1 = 0;
    for (i, c) in s.chars().enumerate() {
        if c == '0' {
            c0 += 1 - (i % 2);
            c1 += i % 2;
        } else {
            c1 += 1 - (i % 2);
            c0 += i % 2;
        }
    }
    min(c0, c1) as i32
}

fn main() {}
