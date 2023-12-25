use std::collections::HashMap;

fn decoding(s: &String, idx: usize, dp: &mut HashMap<usize, i32>) -> i32 {
    if idx >= s.len() {
        1
    } else if let Some(sol) = dp.get(&idx) {
        *sol
    } else {
        let c = s.chars().nth(idx).unwrap();
        let mut sol = 0;
        if c != '0' {
            sol = decoding(s, idx + 1, dp);
            if idx + 1 < s.len()
                && (c == '1' || (c == '2' && s.chars().nth(idx + 1).unwrap() <= '6'))
            {
                sol += decoding(s, idx + 2, dp);
            }
        }
        dp.insert(idx, sol);
        sol
    }
}

pub fn num_decodings(s: String) -> i32 {
    decoding(&s, 0, &mut HashMap::new())
}

fn main() {}
