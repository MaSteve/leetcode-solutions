use std::collections::VecDeque;

pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
    tokens.sort_unstable();
    let mut deque = VecDeque::from(tokens);
    let mut score = 0;
    let mut max_score = 0;
    loop {
        let mut skip = false;
        if let Some(v) = deque.front() {
            if *v <= power {
                power -= *v;
                score += 1;
                max_score = max_score.max(score);
                skip = true;
                deque.pop_front();
            }
        }
        if skip {
            continue;
        }
        if let Some(v) = deque.back() {
            if score > 0 {
                power += *v;
                score -= 1;
                skip = true;
                deque.pop_back();
            }
        }
        if skip {
            continue;
        }
        break;
    }
    max_score
}

fn main() {}
