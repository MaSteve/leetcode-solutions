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

fn solve(node: &Option<Rc<RefCell<TreeNode>>>, sol: &mut Vec<i32>, depth: usize) {
    if let Some(node) = node {
        let node = node.borrow();
        if sol.len() <= depth {
            sol.push(node.val);
        } else {
            sol[depth] = max(sol[depth], node.val);
        }
        solve(&node.right, sol, depth + 1);
        solve(&node.left, sol, depth + 1);
    }
}

pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut sol = vec![];
    solve(&root, &mut sol, 0);
    sol
}

fn main() {}
