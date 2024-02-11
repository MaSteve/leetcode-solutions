fn solve(
    grid: &Vec<Vec<i32>>,
    c1: usize,
    c2: usize,
    row: usize,
    dp: &mut Vec<Vec<Vec<i32>>>,
) -> i32 {
    if row >= grid.len() {
        return 0;
    }
    if dp[c1][c2][row] != -1 {
        return dp[c1][c2][row];
    }
    let collected = if c1 == c2 {
        grid[row][c1]
    } else {
        grid[row][c1] + grid[row][c2]
    };
    let mut sol = 0;
    for i in -1..=1 {
        let next_c1 = c1 as i32 + i;
        if next_c1 < 0 || next_c1 >= grid[row].len() as i32 {
            continue;
        }
        for j in -1..=1 {
            let next_c2 = c2 as i32 + j;
            if next_c2 < 0 || next_c2 >= grid[row].len() as i32 || next_c2 < next_c1 {
                continue;
            }
            sol = sol.max(collected + solve(grid, next_c1 as usize, next_c2 as usize, row + 1, dp));
        }
    }
    dp[c1][c2][row] = sol;
    sol
}

pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    solve(
        &grid,
        0,
        grid[0].len() - 1,
        0,
        &mut vec![vec![vec![-1; grid.len()]; grid[0].len()]; grid[0].len()],
    )
}

fn main() {}
