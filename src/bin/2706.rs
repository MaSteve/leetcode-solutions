pub fn buy_choco(mut prices: Vec<i32>, money: i32) -> i32 {
    prices.sort_unstable();
    let sol = money - prices[0] - prices[1];
    if sol >= 0 {
        sol
    } else {
        money
    }
}

fn main() {}
