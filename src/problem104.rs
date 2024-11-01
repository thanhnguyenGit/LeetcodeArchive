use std::cell::RefCell;
use std::rc::Rc;
use crate::helper::treenode::TreeNode;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let counter = 0;
    match root {
        None => 0,
        Some(root) => {
            let root = root.borrow();
            checking(&root.left, &root.right)
        }
    }
}

fn checking(left : &Option<Rc<RefCell<TreeNode>>>, right : &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match (left,right) {

    }
}