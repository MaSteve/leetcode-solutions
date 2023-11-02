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

fn count_and_sum(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
    if let Some(node) = root {
        let node = node.borrow();
        let (sol1, sum1, count1) = count_and_sum(&node.left);
        let (sol2, sum2, count2) = count_and_sum(&node.right);

        let sum = sum1 + sum2 + node.val;
        let count = count1 + count2 + 1;
        let mut sol = sol1 + sol2;
        if node.val == sum / count {
            sol += 1
        }
        (sol, sum, count)
    } else {
        (0, 0, 0)
    }
}

pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    count_and_sum(&root).0
}

fn main() {}
