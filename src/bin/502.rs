use std::collections::BinaryHeap;

pub fn find_maximized_capital(mut k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let mut projects: Vec<(i32, i32)> = profits.iter().zip(capital).map(|(p, c)| (*p, c)).collect();
    projects.sort_unstable_by_key(|t| t.1);
    let mut pq = BinaryHeap::new();
    let mut last = 0;
    while k > 0 {
        while last < projects.len() && projects[last].1 <= w {
            pq.push(projects[last]);
            last += 1;
        }
        if let Some((p, _)) = pq.pop() {
            w += p;
            k -= 1;
        } else {
            break;
        }
    }
    w
}

fn main() {}
