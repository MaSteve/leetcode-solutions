use std::collections::VecDeque;

pub fn minimum_length(s: String) -> i32 {
    let mut deque = VecDeque::new();
    let mut current_char = ' ';
    let mut current_count = 0;
    for c in s.chars() {
        if c == current_char {
            current_count += 1;
        } else {
            deque.push_back((current_char, current_count));
            current_char = c;
            current_count = 1;
        }
    }
    deque.pop_front();
    deque.push_back((current_char, current_count));
    while deque.len() > 1 {
        if let (Some((c1, _)), Some((c2, _))) = (deque.front(), deque.back()) {
            if c1 == c2 {
                deque.pop_front();
                deque.pop_back();
            } else {
                break;
            }
        }
    }
    if deque.len() == 1 {
        let length = deque.front().unwrap().1;
        if length > 1 {
            deque.pop_front();
        }
    }
    deque.iter().map(|(_, n)| n).sum()
}

fn main() {}
