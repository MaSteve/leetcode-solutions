use std::cmp::max;

pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
    if matrix.len() == 0 {
        return 0;
    }
    let mut sol = 0;
    let mut heights = vec![0; matrix[0].len()];
    for row in matrix {
        for i in 0..row.len() {
            if row[i] == 1 {
                heights[i] += 1;
            } else {
                heights[i] = 0;
            }
        }
        let mut sorted_heights = heights.clone();
        sorted_heights.sort_unstable();
        for (i, v) in sorted_heights.iter().enumerate() {
            sol = max(sol, v * (row.len() - i) as i32);
        }
    }
    sol
}

fn main() {}
