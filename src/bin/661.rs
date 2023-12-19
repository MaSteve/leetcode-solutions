pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut sol = vec![vec![0; img[0].len()]; img.len()];
    for i in 0..img.len() {
        for j in 0..img[i].len() {
            let mut cells = 0;
            for di in -1..=1 {
                let i_prime = i as isize + di;
                if i_prime < 0 || i_prime as usize >= img.len() {
                    continue;
                }
                for dj in -1..=1 {
                    let j_prime = j as isize + dj;
                    if j_prime < 0 || j_prime as usize >= img[i].len() {
                        continue;
                    }
                    cells += 1;
                    sol[i][j] += img[i_prime as usize][j_prime as usize];
                }
            }
            sol[i][j] /= cells;
        }
    }
    sol
}

fn main() {}
