use std::{collections::HashMap, mem};

fn gcd(mut a: i32, mut b: i32) -> i32 {
    if a < b {
        mem::swap(&mut a, &mut b);
    }
    while b != 0 {
        let d = a % b;
        a = b;
        b = d;
    }
    a
}

fn canonize((x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
    let d = gcd(x.abs(), gcd(y.abs(), z.abs()));
    if x < 0 || (x == 0 && y < 0) {
        (-x / d, -y / d, -z / d)
    } else {
        (x / d, y / d, z / d)
    }
}

pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
    let mut sol = 0;
    let mut lines = HashMap::<(i32, i32, i32), i32>::new();
    for i in 0..points.len() {
        lines.clear();
        for j in i + 1..points.len() {
            let (x1, y1) = (points[i][0], points[i][1]);
            let (x2, y2) = (points[j][0], points[j][1]);
            let line = canonize((y1 - y2, x2 - x1, x1 * y2 - x2 * y1));
            let entry = lines.entry(line).or_insert(0);
            *entry += 1;
            if *entry > sol {
                sol = *entry;
            }
        }
    }
    sol + 1
}

fn main() {}
