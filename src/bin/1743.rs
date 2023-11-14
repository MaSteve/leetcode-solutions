use std::collections::{HashMap, HashSet};

pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut starting_node: HashSet<i32> = HashSet::new();
    for edge in adjacent_pairs {
        if let Some(v) = graph.get_mut(&edge[0]) {
            v.push(edge[1]);
            starting_node.remove(&edge[0]);
        } else {
            graph.insert(edge[0], vec![edge[1]]);
            starting_node.insert(edge[0]);
        }
        if let Some(v) = graph.get_mut(&edge[1]) {
            v.push(edge[0]);
            starting_node.remove(&edge[1]);
        } else {
            graph.insert(edge[1], vec![edge[0]]);
            starting_node.insert(edge[1]);
        }
    }
    let mut sol = vec![];
    if let Some(start) = starting_node.iter().next() {
        let mut last = *start;
        let mut current = *start;
        while let Some(edge) = graph.get(&current) {
            sol.push(current);
            if edge[0] == last && edge.len() == 2 {
                last = current;
                current = edge[1];
            } else if edge[0] != last {
                last = current;
                current = edge[0];
            } else if edge.len() == 1 {
                break;
            }
        }
    }

    sol
}

fn main() {}
