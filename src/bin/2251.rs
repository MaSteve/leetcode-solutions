pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, mut people: Vec<i32>) -> Vec<i32> {
    let mut events: Vec<(i32, u8, usize)> = flowers
        .iter()
        .flat_map(|f| [(f[0], 0, 0), (f[1], 2, 0)])
        .chain(people.iter().enumerate().map(|(p, t)| (*t, 1, p)))
        .collect();
    events.sort_unstable();
    let mut bloom = 0;
    for (_, op, p) in events {
        if op == 0 {
            bloom += 1;
        } else if op == 1 {
            people[p] = bloom;
        } else if op == 2 {
            bloom -= 1;
        }
    }
    people
}

fn main() {}
