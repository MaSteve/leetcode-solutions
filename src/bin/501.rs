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

fn find(
    root: &Option<Rc<RefCell<TreeNode>>>,
    current_value: &mut i32,
    current_count: &mut i32,
    max_count: &mut i32,
    mut modes: Vec<i32>,
) -> Vec<i32> {
    if let Some(node) = root {
        let node = node.borrow();
        modes = find(&node.left, current_value, current_count, max_count, modes);

        if node.val == *current_value {
            *current_count += 1;
        } else {
            *current_value = node.val;
            *current_count = 1;
        }

        if current_count == max_count {
            modes.push(*current_value);
        } else if current_count > max_count {
            *max_count = *current_count;
            modes.clear();
            modes.push(*current_value);
        }

        return find(&node.right, current_value, current_count, max_count, modes);
    }
    modes
}

pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    find(&root, &mut 0, &mut 0, &mut 0, vec![])
}

fn main() {}
