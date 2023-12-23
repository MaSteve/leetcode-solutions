use std::collections::HashSet;

pub fn is_path_crossing(path: String) -> bool {
    let mut point = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(point);
    for d in path.chars() {
        let next_point = match d {
            'N' => (point.0, point.1 + 1),
            'S' => (point.0, point.1 - 1),
            'E' => (point.0 + 1, point.1),
            _ => (point.0 - 1, point.1),
        };
        if visited.contains(&next_point) {
            return true;
        }
        visited.insert(next_point);
        point = next_point;
    }
    false
}

fn main() {}
