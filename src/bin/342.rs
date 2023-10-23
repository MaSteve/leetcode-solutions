pub fn is_power_of_four(mut n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    while n & 3 == 0 {
        n >>= 2;
    }
    n == 1
}

fn main() {}
