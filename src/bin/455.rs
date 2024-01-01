pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
    g.sort();
    s.sort();
    let mut sol = 0;
    while !g.is_empty() && !s.is_empty() {
        if g.pop().unwrap() <= *s.last().unwrap() {
            s.pop();
            sol += 1;
        }
    }
    sol
}

fn main() {}
