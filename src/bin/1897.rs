pub fn make_equal(words: Vec<String>) -> bool {
    let mut count = vec![0; 'z' as usize - 'a' as usize + 1];
    words
        .iter()
        .flat_map(|w| w.chars())
        .for_each(|c| count[c as usize - 'a' as usize] += 1);
    count.iter().all(|v| v % words.len() == 0)
}

fn main() {}
