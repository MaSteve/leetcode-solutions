use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, VecDeque},
};

#[derive(PartialEq, Eq)]
enum Actions {
    Release(i64, usize),
    Fetch(i64, i64),
}

impl Ord for Actions {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Actions::Release(t1, r1), Actions::Release(t2, r2)) => (t1, r1).cmp(&(t2, r2)),
            (Actions::Fetch(f1, t1), Actions::Fetch(f2, t2)) => (f1, t1).cmp(&(f2, t2)),
            (Actions::Fetch(f1, _), Actions::Release(t2, _)) => {
                let cmp_res = f1.cmp(t2);
                match cmp_res {
                    Ordering::Equal => Ordering::Greater,
                    _ => cmp_res,
                }
            }
            (Actions::Release(t1, _), Actions::Fetch(f2, _)) => {
                let cmp_res = t1.cmp(f2);
                match cmp_res {
                    Ordering::Equal => Ordering::Less,
                    _ => cmp_res,
                }
            }
        }
    }
}

impl PartialOrd for Actions {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
    let mut queue = VecDeque::new();
    let mut rooms = (0..n as usize).map(Reverse).collect::<BinaryHeap<_>>();
    let mut count = vec![0; n as usize];
    let mut actions = meetings
        .iter()
        .map(|m| Reverse(Actions::Fetch(m[0] as i64, m[1] as i64)))
        .collect::<BinaryHeap<_>>();

    while let Some(Reverse(action)) = actions.pop() {
        match action {
            Actions::Fetch(_, to) => {
                if let Some(Reverse(room)) = rooms.pop() {
                    actions.push(Reverse(Actions::Release(to, room)));
                    count[room] += 1;
                } else {
                    queue.push_back(action);
                }
            }
            Actions::Release(t, room) => {
                if let Some(Actions::Fetch(from, to)) = queue.pop_front() {
                    actions.push(Reverse(Actions::Release(to + (t - from), room)));
                    count[room] += 1;
                } else {
                    rooms.push(Reverse(room));
                }
            }
        }
    }

    count
        .iter()
        .enumerate()
        .max_by_key(|(i, c)| (*c, Reverse(*i)))
        .unwrap()
        .0 as i32
}

fn main() {}
