use std::cmp::min;

pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
    let dx = (sx - fx).abs();
    let dy = (sy - fy).abs();
    let d = dx + dy - min(dx, dy);
    (d != 0 && d <= t) || (d == 0 && t != 1)
}

fn main() {}
