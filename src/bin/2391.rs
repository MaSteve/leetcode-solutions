pub fn garbage_collection(garbage: Vec<String>, mut travel: Vec<i32>) -> i32 {
    let (gb, last_metal, last_paper, last_grass) = garbage.iter().enumerate().fold(
        (0, 0, 0, 0),
        |(gb, last_metal, last_paper, last_grass), (i, house_garbage)| {
            (
                gb + house_garbage.len(),
                if house_garbage.contains('M') {
                    i
                } else {
                    last_metal
                },
                if house_garbage.contains('P') {
                    i
                } else {
                    last_paper
                },
                if house_garbage.contains('G') {
                    i
                } else {
                    last_grass
                },
            )
        },
    );
    let mut acum = 0;
    for t in travel.iter_mut() {
        let v = *t;
        *t = acum;
        acum += v;
    }
    travel.push(acum);
    gb as i32 + travel[last_metal] + travel[last_paper] + travel[last_grass]
}

fn main() {}
