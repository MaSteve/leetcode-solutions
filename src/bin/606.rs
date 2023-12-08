use std::{cell::RefCell, rc::Rc};

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

fn to_string(node: &TreeNode, sol: &mut String) {
    sol.push_str(&node.val.to_string());
    if node.left.is_some() || node.right.is_some() {
        sol.push('(');
        if let Some(ref node) = node.left {
            to_string(&node.borrow(), sol);
        }
        sol.push(')');
        if let Some(ref node) = node.right {
            sol.push('(');
            to_string(&node.borrow(), sol);
            sol.push(')');
        }
    }
}

pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut sol = String::new();
    if let Some(node) = root {
        to_string(&node.borrow(), &mut sol);
    }
    sol
}

fn main() {}
