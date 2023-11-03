pub fn build_array(target: Vec<i32>, _: i32) -> Vec<String> {
    let mut sol = vec![];
    let mut last = 0;
    for v in target {
        for _ in 0..(v - last - 1) {
            sol.push("Push".to_string());
            sol.push("Pop".to_string());
        }
        sol.push("Push".to_string());
        last = v;
    }
    sol
}

fn main() {}
