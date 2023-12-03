use std::cmp::max;

pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    let mut sol = 0;
    for i in 1..points.len() {
        let dx = (points[i][0] - points[i - 1][0]).abs();
        let dy = (points[i][1] - points[i - 1][1]).abs();
        sol += max(dx, dy);
    }
    sol
}

fn main() {}
