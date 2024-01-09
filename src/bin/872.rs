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

struct TreeNodeLeafIterator {
    backtrack: Vec<Rc<RefCell<TreeNode>>>,
    current: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNodeLeafIterator {
    fn from(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self {
            backtrack: vec![],
            current: root,
        }
    }
}

impl Iterator for TreeNodeLeafIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            while let Some(ref node_rc) = self.current {
                let mut next_value = None;
                let mut next_node = None;
                {
                    let node = node_rc.borrow();
                    if let Some(ref rc) = node.left {
                        next_node = Some(Rc::clone(rc));
                        self.backtrack.push(Rc::clone(node_rc));
                    } else if let Some(ref rc) = node.right {
                        next_node = Some(Rc::clone(rc));
                    } else {
                        next_value = Some(node.val);
                    }
                }
                self.current = next_node;
                if next_value.is_some() {
                    return next_value;
                }
            }
            while let Some(node) = self.backtrack.pop() {
                if let Some(ref rc) = node.borrow().right {
                    self.current = Some(Rc::clone(rc));
                    break;
                }
            }
            if self.current.is_none() {
                return None;
            }
        }
    }
}

pub fn leaf_similar(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    TreeNodeLeafIterator::from(root1).eq(TreeNodeLeafIterator::from(root2))
}

fn main() {}
