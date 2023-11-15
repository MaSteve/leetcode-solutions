pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
    arr.sort_unstable();
    let mut last = 0;
    for v in arr {
        if v >= last + 1 {
            last += 1;
        }
    }
    last
}

fn main() {}
