pub fn first_palindrome(words: Vec<String>) -> String {
    for w in words {
        let bytes = w.as_bytes();
        let mut pal = true;
        for i in 0..w.len() / 2 {
            if bytes[i] != bytes[w.len() - i - 1] {
                pal = false;
                break;
            }
        }
        if pal {
            return w;
        }
    }
    "".to_string()
}

fn main() {}
