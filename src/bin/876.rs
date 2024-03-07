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

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut len = 0;
    let mut next = &head;
    while let Some(node) = next {
        len += 1;
        next = &node.next;
    }
    len = len / 2;
    let mut next = head;
    for _ in 0..len {
        next = next.unwrap().next;
    }
    next
}

fn main() {}
