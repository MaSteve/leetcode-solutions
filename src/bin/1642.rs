fn check(heights: &Vec<i32>, idx: usize, bricks: i32, ladders: i32) -> bool {
    let mut diffs = vec![];
    let mut prev = heights[0];
    for i in 1..=idx {
        let diff = heights[i] - prev;
        if diff > 0 {
            diffs.push(diff);
        }
        prev = heights[i];
    }
    diffs.sort_unstable();
    diffs.iter().rev().skip(ladders as usize).sum::<i32>() <= bricks
}

fn solve(heights: &Vec<i32>, bricks: i32, ladders: i32, first: usize, last: usize) -> i32 {
    if first == last - 1 {
        return first as i32;
    }
    let mid = (first + last) / 2;
    if check(heights, mid, bricks, ladders) {
        solve(heights, bricks, ladders, mid, last)
    } else {
        solve(heights, bricks, ladders, first, mid)
    }
}

pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
    solve(&heights, bricks, ladders, 0, heights.len())
}

fn main() {}
