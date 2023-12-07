pub fn largest_odd_number(mut num: String) -> String {
    while let Some(c) = num.pop() {
        if ['1', '3', '5', '7', '9'].contains(&c) {
            num.push(c);
            return num;
        }
    }
    num
}

fn main() {}
