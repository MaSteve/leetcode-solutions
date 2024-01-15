use std::collections::HashMap;

pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut loss: HashMap<i32, i32> = HashMap::new();
    for m in matches {
        loss.entry(m[0]).or_insert(0);
        *loss.entry(m[1]).or_insert(0) += 1;
    }
    let mut v1 = vec![];
    let mut v2 = vec![];
    for (p, l) in loss {
        if l == 0 {
            v1.push(p);
        } else if l == 1 {
            v2.push(p);
        }
    }
    v1.sort_unstable();
    v2.sort_unstable();
    vec![v1, v2]
}

fn main() {}
