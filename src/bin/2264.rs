pub fn largest_good_integer(num: String) -> String {
    let (sol, _) = num
        .chars()
        .enumerate()
        .fold((-1, 0), |(sol, current), (i, c)| {
            let current = (current * 10 + (c as i16 - '0' as i16)) % 1000;
            let sol = if i >= 2 && current % 111 == 0 && current > sol {
                current
            } else {
                sol
            };
            (sol, current)
        });
    if sol >= 0 {
        format!("{:03}", sol)
    } else {
        "".to_string()
    }
}

fn main() {}
