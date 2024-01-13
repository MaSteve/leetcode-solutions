pub fn min_steps(s: String, t: String) -> i32 {
    let mut counters = [0_i32; 26];
    s.bytes().for_each(|c| counters[(c - b'a') as usize] += 1);
    t.bytes().for_each(|c| counters[(c - b'a') as usize] -= 1);
    counters.iter().map(|v| v.abs()).sum::<i32>() / 2
}

fn main() {}
