use std::{cmp::Reverse, collections::BinaryHeap};

pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_unstable();
    let mut heap = BinaryHeap::new();
    let mut sol = 0;
    let mut last_burst = -1;
    for (idx, balloon) in points.iter().enumerate() {
        while let Some(Reverse((end, idx2))) = heap.peek() {
            if *end < balloon[0] {
                if *idx2 as i32 > last_burst {
                    last_burst = idx as i32 - 1;
                    sol += 1;
                }
                heap.pop();
            } else {
                break;
            }
        }
        heap.push(Reverse((balloon[1], idx)));
    }
    while let Some(Reverse((_, idx))) = heap.pop() {
        if idx as i32 > last_burst {
            sol += 1;
            break;
        }
    }
    sol
}

fn main() {}
