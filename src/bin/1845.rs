use std::collections::BinaryHeap;

struct SeatManager {
    pristine: i32,
    unreserved: BinaryHeap<i32>,
}

impl SeatManager {
    fn new(n: i32) -> Self {
        SeatManager {
            pristine: 0,
            unreserved: BinaryHeap::new(),
        }
    }

    fn reserve(&mut self) -> i32 {
        if self.unreserved.is_empty() {
            self.pristine += 1;
            self.pristine
        } else {
            -self.unreserved.pop().unwrap()
        }
    }

    fn unreserve(&mut self, seat_number: i32) {
        if seat_number == self.pristine {
            self.pristine -= 1;
        } else {
            self.unreserved.push(-seat_number);
        }
    }
}

fn main() {}
