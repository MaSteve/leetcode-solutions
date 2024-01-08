use std::cell::RefCell;
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

fn solve(node: &Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    if let Some(rc_node) = node {
        let node = rc_node.borrow();
        let sol;
        if node.val <= high && node.val >= low {
            sol = node.val + solve(&node.right, low, high) + solve(&node.left, low, high);
        } else if node.val > high {
            sol = solve(&node.left, low, high);
        } else {
            sol = solve(&node.right, low, high);
        }
        sol
    } else {
        0
    }
}

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    solve(&root, low, high)
}

fn main() {}
