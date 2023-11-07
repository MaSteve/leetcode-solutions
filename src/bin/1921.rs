pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut time: Vec<i32> = vec![0; dist.len()];
    for i in 0..dist.len() {
        time[i] = dist[i] / speed[i];
        if dist[i] % speed[i] != 0 {
            time[i] += 1;
        }
    }
    time.sort_unstable();
    for i in 0..time.len() {
        if time[i] <= i as i32 {
            return i as i32;
        }
    }
    time.len() as i32
}

fn main() {}
