use std::cell::RefCell;
use std::cmp::max;
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

fn solve(root: &Option<Rc<RefCell<TreeNode>>>) -> (i64, i64) {
    if let Some(node) = root {
        let node = node.borrow();
        let sol_right = solve(&node.right);
        let sol_left = solve(&node.left);
        let best_path_ending_at_root = max(
            node.val as i64,
            max(node.val as i64 + sol_right.1, node.val as i64 + sol_left.1),
        );

        (
            max(
                sol_right.0,
                max(
                    sol_left.0,
                    max(
                        best_path_ending_at_root,
                        sol_right.1 + sol_left.1 + node.val as i64,
                    ),
                ),
            ),
            best_path_ending_at_root,
        )
    } else {
        (i32::MIN as i64, i32::MIN as i64)
    }
}

pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    solve(&root).0 as i32
}

fn main() {}
