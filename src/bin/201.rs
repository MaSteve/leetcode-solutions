pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
    let mut sol = 0;
    for bit_pos in (0..31).rev() {
        let mask = 1 << bit_pos;
        if left & mask != right & mask {
            return sol;
        }
        sol |= left & mask;
    }
    sol
}

fn main() {}
