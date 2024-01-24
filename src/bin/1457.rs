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

fn solve(root: &Option<Rc<RefCell<TreeNode>>>, buckets: &mut [i32; 10]) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        buckets[node.val as usize] += 1;
        let sol;
        if node.right.is_none() && node.left.is_none() {
            let mut odd = 0;
            for i in 0..=9 {
                if buckets[i] % 2 == 1 {
                    odd += 1;
                }
            }
            if odd <= 1 {
                sol = 1;
            } else {
                sol = 0;
            }
        } else {
            sol = solve(&node.left, buckets) + solve(&node.right, buckets);
        }
        buckets[node.val as usize] -= 1;
        sol
    } else {
        0
    }
}

pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    solve(&root, &mut [0; 10])
}

fn main() {}
