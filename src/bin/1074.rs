fn get(matrix: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    if i < matrix.len() && j < matrix[i].len() {
        matrix[i][j]
    } else {
        0
    }
}

pub fn num_submatrix_sum_target(mut matrix: Vec<Vec<i32>>, target: i32) -> i32 {
    for i in (0..matrix.len()).rev() {
        for j in (0..matrix[i].len()).rev() {
            matrix[i][j] +=
                get(&matrix, i + 1, j) + get(&matrix, i, j + 1) - get(&matrix, i + 1, j + 1);
        }
    }
    let mut sol = 0;
    for x1 in 0..matrix.len() {
        for x2 in x1..matrix.len() {
            for y1 in 0..matrix[x1].len() {
                for y2 in y1..matrix[x2].len() {
                    let sum = matrix[x1][y1] - get(&matrix, x1, y2 + 1) - get(&matrix, x2 + 1, y1)
                        + get(&matrix, x2 + 1, y2 + 1);
                    if sum == target {
                        sol += 1;
                    }
                }
            }
        }
    }
    sol
}

fn main() {}
