pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
    let mut sol = arr[0];
    let mut count = 0;
    for i in 1..arr.len() {
        if arr[i] > sol {
            sol = arr[i];
            count = 1;
        } else {
            count += 1;
        }
        if count >= k {
            break;
        }
    }
    sol
}

fn main() {}
