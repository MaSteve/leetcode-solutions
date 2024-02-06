use std::collections::HashMap;

fn to_freq(s: &String) -> [usize; 26] {
    let mut freq = [0; 26];
    s.bytes().for_each(|c| freq[(c - b'a') as usize] += 1);
    freq
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::<[usize; 26], Vec<String>>::new();
    for s in strs {
        let freq = to_freq(&s);
        map.entry(freq).or_default().push(s);
    }
    map.into_values().collect()
}

fn main() {}
