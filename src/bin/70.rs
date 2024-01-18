fn prod(a: &[i32; 4], b: &[i32; 4]) -> [i32; 4] {
    [
        a[0] * b[0] + a[1] * b[2],
        a[0] * b[1] + a[1] * b[3],
        a[2] * b[0] + a[3] * b[2],
        a[2] * b[1] + a[3] * b[3],
    ]
}

pub fn climb_stairs(mut n: i32) -> i32 {
    let mut sol = [1, 0, 0, 1];
    let fib = [1, 1, 1, 0];
    let mut bits = vec![];
    while n > 0 {
        bits.push(n % 2);
        n /= 2;
    }
    bits.reverse();
    for bit in bits {
        sol = prod(&sol, &sol);
        if bit == 1 {
            sol = prod(&sol, &fib);
        }
    }
    sol[0]
}

fn main() {}
