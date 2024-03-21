use std::collections::VecDeque;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut queue = VecDeque::new();
    while let Some(node) = head {
        queue.push_back(node.val);
        head = node.next;
    }
    while let Some(val) = queue.pop_front() {
        head = Some(Box::new(ListNode { next: head, val }));
    }
    head
}

fn main() {}
