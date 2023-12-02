fn count(word: &String) -> [u8; 26] {
    let mut counters = [0; 26];
    for c in word.chars() {
        counters[c as usize - 'a' as usize] += 1;
    }
    return counters;
}

fn lower_eq_count(c1: &[u8; 26], c2: &[u8; 26]) -> bool {
    for i in 0..26 {
        if c1[i] > c2[i] {
            return false;
        }
    }
    true
}

pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let chars_count = count(&chars);
    words
        .iter()
        .map(|w| {
            if lower_eq_count(&count(w), &chars_count) {
                w.len() as i32
            } else {
                0
            }
        })
        .sum()
}

fn main() {}
