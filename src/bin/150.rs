pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut values: Vec<i32> = vec![];
    for t in tokens {
        match t.as_str() {
            "+" => {
                let v = values.pop().unwrap() + values.pop().unwrap();
                values.push(v);
            }
            "-" => {
                let v2 = values.pop().unwrap();
                let v = values.pop().unwrap() - v2;
                values.push(v);
            }
            "*" => {
                let v = values.pop().unwrap() * values.pop().unwrap();
                values.push(v);
            }
            "/" => {
                let v2 = values.pop().unwrap();
                let v = values.pop().unwrap() / v2;
                values.push(v);
            }
            v => {
                values.push(v.parse().unwrap());
            }
        }
    }

    values.pop().unwrap()
}

fn main() {}
