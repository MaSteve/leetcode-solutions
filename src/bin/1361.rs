use std::collections::VecDeque;

pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
    let mut visited = vec![false; n as usize];
    for i in 0..n {
        if !visited[i as usize] {
            let mut pending_children = VecDeque::new();
            if left_child[i as usize] >= 0 {
                pending_children.push_back(left_child[i as usize]);
            }
            if right_child[i as usize] >= 0 {
                pending_children.push_back(right_child[i as usize]);
            }
            while let Some(node) = pending_children.pop_front() {
                if visited[node as usize] {
                    return false;
                }
                visited[node as usize] = true;
                if node >= i {
                    if left_child[node as usize] >= 0 {
                        pending_children.push_back(left_child[node as usize]);
                    }
                    if right_child[node as usize] >= 0 {
                        pending_children.push_back(right_child[node as usize]);
                    }
                }
            }
        }
    }
    visited.iter().filter(|visited| !*visited).count() == 1
}

fn main() {}
