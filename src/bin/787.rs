use std::{cmp::Reverse, collections::BinaryHeap};

pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let mut graph = vec![vec![]; n as usize];
    for f in flights {
        graph[f[0] as usize].push((f[1] as usize, f[2]));
    }
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, src as usize, k)));
    let mut costs = vec![vec![i32::MAX; k as usize + 1]; n as usize];
    for i in 0..=k as usize {
        costs[src as usize][i] = 0;
    }
    while let Some(Reverse((cost, node, remaining_stops))) = heap.pop() {
        if node == dst as usize {
            return cost;
        }
        if remaining_stops >= 0 {
            for (dst, c) in &graph[node] {
                if cost + c >= costs[*dst][remaining_stops as usize] {
                    continue;
                }
                costs[*dst][remaining_stops as usize] = cost + c;
                heap.push(Reverse((cost + c, *dst, remaining_stops - 1)));
            }
        }
    }
    -1
}

fn main() {}
