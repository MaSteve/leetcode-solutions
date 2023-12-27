use std::cmp::max;

pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
    let mut sol = 0;
    let mut maxi = 0;
    let mut accum = 0;
    let mut last = '-';
    for (t, c) in needed_time.iter().zip(colors.chars()) {
        if c != last {
            sol += accum - maxi;
            last = c;
            accum = *t;
            maxi = *t;
        } else {
            accum += *t;
            maxi = max(maxi, *t);
        }
    }
    sol += accum - maxi;
    sol
}

fn main() {}
