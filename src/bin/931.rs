use std::cmp::min;

pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
    for i in 1..matrix.len() {
        for j in 0..matrix[i].len() {
            let mut min_prev = matrix[i - 1][j];
            if j > 0 {
                min_prev = min(min_prev, matrix[i - 1][j - 1]);
            }
            if j < matrix[i].len() - 1 {
                min_prev = min(min_prev, matrix[i - 1][j + 1]);
            }
            matrix[i][j] += min_prev;
        }
    }
    *matrix[matrix.len() - 1].iter().min().unwrap()
}

fn main() {}
