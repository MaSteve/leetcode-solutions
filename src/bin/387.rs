pub fn first_uniq_char(s: String) -> i32 {
    let mut freq = [(0, 0); 26];
    s.bytes()
        .enumerate()
        .for_each(|(idx, c)| freq[(c - b'a') as usize] = (freq[(c - b'a') as usize].0 + 1, idx));
    freq.iter()
        .filter(|(f, _)| *f == 1)
        .map(|(_, idx)| *idx as i32)
        .min()
        .unwrap_or(-1)
}

fn main() {}
