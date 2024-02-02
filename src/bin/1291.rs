pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    const SEQ_NUM: [i32; 45] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 23, 34, 45, 56, 67, 78, 89, 123, 234, 345, 456, 567, 678,
        789, 1234, 2345, 3456, 4567, 5678, 6789, 12345, 23456, 34567, 45678, 56789, 123456, 234567,
        345678, 456789, 1234567, 2345678, 3456789, 12345678, 23456789, 123456789,
    ];
    SEQ_NUM[SEQ_NUM.partition_point(|&v| v < low)..SEQ_NUM.partition_point(|&v| v <= high)].to_vec()
}

fn main() {
    let v = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut sol: Vec<i32> = vec![];

    for i in 1..=9 {
        v.windows(i).for_each(|w| {
            let mut v = 0;
            for d in w {
                v *= 10;
                v += d;
            }
            sol.push(v);
        });
    }

    sol.sort_unstable();
    println!("{:?}", sol);
}
