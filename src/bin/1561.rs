pub fn max_coins(mut piles: Vec<i32>) -> i32 {
    piles.sort_unstable();
    piles.iter().skip(piles.len() / 3).step_by(2).sum()
}

fn main() {}
