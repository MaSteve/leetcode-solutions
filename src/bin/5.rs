use std::cmp::min;

pub fn longest_palindrome(s: String) -> String {
    let s2: Vec<char> = s.chars().flat_map(|c| ['#', c]).chain(['#']).collect();
    let mut radii = vec![0; s2.len()];
    let mut current_center: i32 = 0;
    let mut current_radius: i32 = 0;
    while current_center < s2.len() as i32 {
        while current_center - current_radius - 1 >= 0
            && current_center + current_radius + 1 < s2.len() as i32
            && s2[(current_center - current_radius - 1) as usize]
                == s2[(current_center + current_radius + 1) as usize]
        {
            current_radius += 1;
        }
        radii[current_center as usize] = current_radius;

        let old_center = current_center;
        let old_radius = current_radius;
        current_center += 1;
        current_radius = 0;
        while current_center <= old_center + old_radius {
            let mirrored_center = 2 * old_center - current_center;
            let max_mirrored_radius = old_center + old_radius - current_center;
            if radii[mirrored_center as usize] == max_mirrored_radius {
                current_radius = max_mirrored_radius;
                break;
            } else {
                radii[current_center as usize] =
                    min(max_mirrored_radius, radii[mirrored_center as usize]);
                current_center += 1;
            }
        }
    }

    let (center, radius) = radii.iter().enumerate().max_by_key(|&(_, v)| v).unwrap();
    s[((center as i32 - radius) / 2) as usize..((center as i32 + radius) / 2) as usize].to_string()
}

fn main() {}
