fn gauss(n: i32) -> i32 {
    n * (n + 1) / 2
}

pub fn pivot_integer(n: i32) -> i32 {
    let gauss_n = gauss(n);
    let mut start = 1;
    let mut end = n + 1;
    while start < end {
        let mid = (start + end) / 2;
        let gauss_mid = gauss(mid);
        let second_sum = gauss_n - gauss_mid + mid;
        if gauss_mid == second_sum {
            return mid;
        }
        if gauss_mid < second_sum {
            start = mid + 1;
        } else {
            end = mid;
        }
    }
    -1
}

fn main() {}
