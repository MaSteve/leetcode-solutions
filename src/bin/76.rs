fn to_freq(s: &String) -> [i32; 58] {
    let mut freq = [0; 58];
    s.bytes().for_each(|c| freq[(c - b'A') as usize] += 1);
    freq
}

fn check(freq_window: &[i32; 58], freq_t: &[i32; 58]) -> bool {
    freq_window.iter().zip(freq_t).all(|(f1, f2)| f1 >= f2)
}

pub fn min_window(s: String, t: String) -> String {
    let freq_t = to_freq(&t);
    let mut freq_window = [0; 58];
    let mut min_len = usize::MAX;
    let mut min_start = s.len();
    let mut start = 0;
    let bytes = s.as_bytes();
    for idx in 0..bytes.len() {
        freq_window[(bytes[idx] - b'A') as usize] += 1;
        while check(&freq_window, &freq_t) {
            let len = idx - start + 1;
            if min_len > len {
                min_len = len;
                min_start = start;
            }
            freq_window[(bytes[start] - b'A') as usize] -= 1;
            start += 1;
        }
    }
    if min_start == s.len() {
        "".to_string()
    } else {
        s[min_start..(min_start + min_len)].to_string()
    }
}

fn main() {}
