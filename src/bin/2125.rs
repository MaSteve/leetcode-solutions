pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let mut prev = 0;
    let mut sol = 0;
    for row in bank {
        let current = row.as_bytes().iter().filter(|c| **c == b'1').count() as i32;
        if current == 0 {
            continue;
        }
        sol += current * prev;
        prev = current;
    }
    sol
}

fn main() {}
