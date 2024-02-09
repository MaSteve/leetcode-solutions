pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    let mut chains = vec![(-1, 0); nums.len()];
    for i in (0..nums.len() - 1).rev() {
        for j in i + 1..nums.len() {
            if chains[j].1 + 1 > chains[i].1 && nums[j] % nums[i] == 0 {
                chains[i] = (j as i32, chains[j].1 + 1);
            }
        }
    }
    let mut link = chains.iter().enumerate().max_by_key(|(_, l)| l.1).unwrap();
    let mut sol = vec![];
    loop {
        sol.push(nums[link.0]);
        if link.1 .0 == -1 {
            break;
        }
        link = (link.1 .0 as usize, &chains[link.1 .0 as usize]);
    }
    sol
}

fn main() {}
