pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let mut trusted_by = vec![vec![]; n as usize];
    let mut trusts = vec![vec![]; n as usize];
    for t in trust {
        let a: i32 = t[0];
        let b: i32 = t[1];
        trusted_by[b as usize - 1].push(a);
        trusts[a as usize - 1].push(b);
    }
    for i in 0..n as usize {
        if trusts[i].is_empty() && trusted_by[i].len() == n as usize - 1 {
            return i as i32 + 1;
        }
    }
    -1
}

fn main() {}
