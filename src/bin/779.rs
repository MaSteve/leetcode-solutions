struct Solution {}

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n == 1 {
            if k == 1 {
                0
            } else {
                -1
            }
        } else {
            match Self::kth_grammar(n - 1, (k - 1) / 2 + 1) {
                0 => {
                    if (k - 1) % 2 == 1 {
                        1
                    } else {
                        0
                    }
                }
                1 => {
                    if (k - 1) % 2 == 1 {
                        0
                    } else {
                        1
                    }
                }
                _ => -1,
            }
        }
    }
}

fn main() {}
