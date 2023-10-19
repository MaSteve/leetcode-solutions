pub fn backspace_compare(s: String, t: String) -> bool {
    let mut bss = 0;
    let mut bst = 0;
    let mut ps = s.len() as i32 - 1;
    let mut pt = t.len() as i32 - 1;
    while ps >= 0 || pt >= 0 {
        while ps >= 0 {
            if let Some('#') = s.chars().nth(ps as usize) {
                bss += 1;
            } else if bss > 0 {
                bss -= 1;
            } else {
                break;
            }
            ps -= 1;
        }
        while pt >= 0 {
            if let Some('#') = t.chars().nth(pt as usize) {
                bst += 1;
            } else if bst > 0 {
                bst -= 1;
            } else {
                break;
            }
            pt -= 1;
        }
        if ps >= 0 && pt >= 0 {
            if s.chars().nth(ps as usize) != t.chars().nth(pt as usize) {
                return false;
            }
            ps -= 1;
            pt -= 1;
        } else if ps >= 0 || pt >= 0 {
            return false;
        }
    }
    true
}

fn main() {}
