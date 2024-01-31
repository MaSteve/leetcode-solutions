pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut sol = vec![0; temperatures.len()];
    let mut max_stack = vec![(i32::MAX, -1)];
    for i in (0..temperatures.len()).rev() {
        while let Some((v, idx)) = max_stack.last() {
            if temperatures[i] >= *v {
                max_stack.pop();
            } else {
                if *idx != -1 {
                    sol[i] = idx - i as i32;
                }
                break;
            }
        }
        max_stack.push((temperatures[i], i as i32));
    }
    sol
}

fn main() {}
