pub fn max_score(s: String) -> i32 {
    let mut zeros = vec![];
    let mut count = 0;
    for c in s.chars() {
        if c == '0' {
            count += 1;
        }
        zeros.push(count);
    }
    zeros.pop();
    zeros
        .iter()
        .enumerate()
        .map(|(i, v)| v + (s.len() - (i + 1) - (count - v)))
        .max()
        .unwrap() as i32
}

fn main() {}
