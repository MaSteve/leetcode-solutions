pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    word1
        .iter()
        .flat_map(|w| w.chars())
        .partial_cmp(word2.iter().flat_map(|w| w.chars()))
        == Some(std::cmp::Ordering::Equal)
}

fn main() {}
