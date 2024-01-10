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

fn solve(
    root: &Option<Rc<RefCell<TreeNode>>>,
    distance: Option<i32>,
    start: i32,
) -> (Option<i32>, i32) {
    if let Some(rc) = root {
        let node = rc.borrow();
        if let Some(d) = distance {
            (
                Some(d + 1),
                max(
                    d + 1,
                    max(
                        solve(&node.left, Some(d + 1), start).1,
                        solve(&node.right, Some(d + 1), start).1,
                    ),
                ),
            )
        } else if node.val == start {
            (
                Some(0),
                max(
                    0,
                    max(
                        solve(&node.left, Some(0), start).1,
                        solve(&node.right, Some(0), start).1,
                    ),
                ),
            )
        } else if let (Some(d), max_dist_left) = solve(&node.left, None, start) {
            (
                Some(d + 1),
                max(
                    d + 1,
                    max(max_dist_left, solve(&node.right, Some(d + 1), start).1),
                ),
            )
        } else if let (Some(d), max_dist_right) = solve(&node.right, None, start) {
            (
                Some(d + 1),
                max(
                    d + 1,
                    max(max_dist_right, solve(&node.left, Some(d + 1), start).1),
                ),
            )
        } else {
            (None, -1)
        }
    } else {
        (None, -1)
    }
}

pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
    solve(&root, None, start).1
}

fn main() {}
