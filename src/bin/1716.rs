pub fn total_money(n: i32) -> i32 {
    let gauss = |n| if n <= 0 { 0 } else { n * (n + 1) / 2 };
    let d = (n - 1) / 7;
    let r = (n - 1) % 7;
    gauss(n) - (6 * d * (r + 1)) - 42 * gauss(d - 1)
}

fn main() {}
