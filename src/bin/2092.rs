use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
    let mut secret = HashSet::new();
    secret.insert(0);
    secret.insert(first_person);
    let mut grouped_meetings = BTreeMap::new();
    for m in meetings {
        grouped_meetings
            .entry(m[2])
            .or_insert(vec![])
            .push((m[0], m[1]));
    }
    for (_, group) in grouped_meetings {
        let mut graph = HashMap::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        for (p1, p2) in group {
            graph.entry(p1).or_insert(vec![]).push(p2);
            graph.entry(p2).or_insert(vec![]).push(p1);
            if secret.contains(&p1) {
                queue.push_back(p1);
            }
            if secret.contains(&p2) {
                queue.push_back(p2);
            }
        }
        while let Some(p) = queue.pop_front() {
            if visited.contains(&p) {
                continue;
            }
            visited.insert(p);
            secret.insert(p);
            for p in graph.get(&p).unwrap() {
                if !secret.contains(p) {
                    queue.push_back(*p);
                }
            }
        }
    }
    secret.iter().map(|&p| p).collect()
}

fn main() {}
