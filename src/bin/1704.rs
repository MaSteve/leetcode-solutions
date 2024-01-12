pub fn halves_are_alike(s: String) -> bool {
    let (a, b) = s.split_at(s.len() / 2);
    let filter_exp = |c: &char| ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(c);
    a.chars().into_iter().filter(filter_exp).count()
        == b.chars().into_iter().filter(filter_exp).count()
}

fn main() {}
