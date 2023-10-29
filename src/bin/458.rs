const EPSILON: f64 = 1e-9;

pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
    let tries = minutes_to_test / minutes_to_die;
    ((buckets as f64).log((tries + 1) as f64) - EPSILON).ceil() as i32
}

fn main() {}
