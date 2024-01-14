pub fn close_strings(word1: String, word2: String) -> bool {
    let mut counter1 = [0; 26];
    let mut mask1 = [false; 26];
    let mut counter2 = [0; 26];
    let mut mask2 = [false; 26];
    word1.bytes().for_each(|c| {
        let idx = (c - b'a') as usize;
        counter1[idx] += 1;
        mask1[idx] = true;
    });
    word2.bytes().for_each(|c| {
        let idx = (c - b'a') as usize;
        counter2[idx] += 1;
        mask2[idx] = true;
    });
    counter1.sort_unstable();
    counter2.sort_unstable();
    mask1 == mask2 && counter1 == counter2
}

fn main() {}
