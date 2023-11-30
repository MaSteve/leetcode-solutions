fn to_zero(n: i32, p: usize) -> i32 {
    let bit = n & (1 << p);
    if p == 0 {
        if bit == 0 {
            0
        } else {
            1
        }
    } else {
        if bit == 0 {
            to_zero(n, p - 1)
        } else {
            one_up(n, p - 1) + (1 << p)
        }
    }
}

fn one_up(n: i32, p: usize) -> i32 {
    let bit = n & (1 << p);
    if p == 0 {
        if bit == 0 {
            1
        } else {
            0
        }
    } else {
        if bit == 0 {
            one_up(n, p - 1) + (1 << p)
        } else {
            to_zero(n, p - 1)
        }
    }
}

pub fn minimum_one_bit_operations_2(n: i32) -> i32 {
    to_zero(n, 31)
}

// Gray code to binary
pub fn minimum_one_bit_operations(mut n: i32) -> i32 {
    n ^= n >> 16;
    n ^= n >> 8;
    n ^= n >> 4;
    n ^= n >> 2;
    n ^= n >> 1;
    n
}

fn main() {}
