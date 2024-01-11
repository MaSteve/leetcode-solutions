use std::cell::RefCell;
use std::cmp::{max, min};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn solve(root: &Rc<RefCell<TreeNode>>) -> (i32, i32, i32) {
    let node = root.borrow();
    let mut max_diff = 0;
    let mut min_value = node.val;
    let mut max_value = node.val;
    if let Some(ref left) = node.left {
        let sol_left = solve(left);
        max_diff = max(
            max(max_diff, sol_left.0),
            max((node.val - sol_left.1).abs(), (node.val - sol_left.2).abs()),
        );
        min_value = min(min_value, sol_left.1);
        max_value = max(max_value, sol_left.2);
    }
    if let Some(ref right) = node.right {
        let sol_right = solve(right);
        max_diff = max(
            max(max_diff, sol_right.0),
            max(
                (node.val - sol_right.1).abs(),
                (node.val - sol_right.2).abs(),
            ),
        );
        min_value = min(min_value, sol_right.1);
        max_value = max(max_value, sol_right.2);
    }
    (max_diff, min_value, max_value)
}

pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    solve(root.as_ref().unwrap()).0
}

fn main() {}
