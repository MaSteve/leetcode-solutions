use std::cmp::max;

pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
    let mut points: Vec<i32> = points.iter().map(|p| p[0]).collect();
    points.sort_unstable();
    let mut previous = points[0];
    let mut sol = 0;
    for i in 1..points.len() {
        sol = max(sol, points[i] - previous);
        previous = points[i];
    }
    sol
}

fn main() {}
