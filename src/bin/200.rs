fn flood(i: usize, j: usize, grid: &mut Vec<Vec<char>>) {
    if grid[i][j] == '1' {
        grid[i][j] = '2';
        if i > 0 {
            flood(i - 1, j, grid);
        }
        if i < grid.len() - 1 {
            flood(i + 1, j, grid);
        }
        if j > 0 {
            flood(i, j - 1, grid);
        }
        if j < grid[i].len() - 1 {
            flood(i, j + 1, grid);
        }
    }
}

pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '1' {
                count += 1;
                flood(i, j, &mut grid);
            }
        }
    }
    count
}

fn main() {}
