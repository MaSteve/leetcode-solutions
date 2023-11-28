const MOD: i64 = 1000000007;

pub fn number_of_ways(corridor: String) -> i32 {
    let mut placements = vec![];
    let mut placement_counter = 1;
    let mut waiting_for_second_seat = false;
    for c in corridor.chars() {
        if c == 'S' {
            if waiting_for_second_seat {
                waiting_for_second_seat = false;
                placements.push(placement_counter);
                placement_counter = 1;
            } else {
                waiting_for_second_seat = true;
            }
        } else {
            if !waiting_for_second_seat {
                placement_counter += 1;
            }
        }
    }
    if placements.len() == 0 || waiting_for_second_seat {
        return 0;
    }
    placements[1..].iter().fold(1, |mul, v| (mul * v) % MOD) as i32
}

fn main() {}
