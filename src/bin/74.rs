pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut start = 0;
    let mut end = m * n;
    while start < end {
        let mid = (start + end) / 2;
        let (i, j) = (mid / n, mid % n);
        if matrix[i][j] == target {
            return true;
        } else if matrix[i][j] < target {
            start = mid + 1;
        } else {
            end = mid;
        }
    }
    false
}

fn main() {}
