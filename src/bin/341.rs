#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    next: Option<i32>,
    stack: Vec<Vec<NestedInteger>>,
}

impl NestedIterator {
    fn new(mut nested_list: Vec<NestedInteger>) -> Self {
        nested_list.reverse();
        let mut iterator = NestedIterator {
            next: None,
            stack: vec![nested_list],
        };
        iterator.find_next();
        iterator
    }

    fn find_next(&mut self) {
        if let Some(v) = self.stack.last_mut() {
            if let Some(elem) = v.pop() {
                match elem {
                    NestedInteger::Int(value) => {
                        self.next = Some(value);
                    }
                    NestedInteger::List(mut v2) => {
                        v2.reverse();
                        self.stack.push(v2);
                        self.find_next();
                    }
                }
            } else {
                self.stack.pop();
                self.find_next();
            }
        } else {
            self.next = None;
        };
    }

    fn next(&mut self) -> i32 {
        let res = self.next.unwrap();
        self.find_next();
        res
    }

    fn has_next(&self) -> bool {
        self.next.is_some()
    }
}

fn main() {}
