pub fn frequency_sort(s: String) -> String {
    let mut strs = vec![String::new(); (b'z' - b'0' + 1) as usize];
    s.bytes()
        .for_each(|c| strs[(c - b'0') as usize].push(c as char));
    strs.sort_unstable_by_key(|s| s.len());
    strs.iter().rev().flat_map(|s| s.chars()).collect()
}

fn main() {}
