pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = vec![0; prices.len() + 1];
    let mut last_max = -1;
    for i in (0..prices.len()).rev() {
        if prices[i] > last_max {
            last_max = prices[i];
        }
        let current_profit = last_max - prices[i];
        max_profit[i] = max_profit[i + 1].max(current_profit);
    }
    let mut sol = 0;
    let mut last_min = 100000;
    for i in 0..prices.len() {
        if prices[i] < last_min {
            last_min = prices[i];
        }
        let current_profit = prices[i] - last_min;
        sol = sol.max(max_profit[i + 1] + current_profit);
    }
    sol
}

fn main() {}
