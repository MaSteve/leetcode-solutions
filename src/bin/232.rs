struct MyQueue {
    entry_stack: Vec<i32>,
    exit_stack: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            entry_stack: vec![],
            exit_stack: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.entry_stack.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.exit_stack.is_empty() {
            while let Some(x) = self.entry_stack.pop() {
                self.exit_stack.push(x);
            }
        }
        if let Some(x) = self.exit_stack.pop() {
            return x;
        }
        unreachable!();
    }

    fn peek(&mut self) -> i32 {
        if self.exit_stack.is_empty() {
            while let Some(x) = self.entry_stack.pop() {
                self.exit_stack.push(x);
            }
        }
        if let Some(&x) = self.exit_stack.last() {
            return x;
        }
        unreachable!()
    }

    fn empty(&self) -> bool {
        self.entry_stack.is_empty() && self.exit_stack.is_empty()
    }
}

fn main() {}
