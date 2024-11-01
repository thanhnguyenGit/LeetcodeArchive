use std::cell::RefCell;
use std::rc::Rc;
use crate::helper::treenode::TreeNode;

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        Some(root) => {
            checking(&root.borrow().left, &root.borrow().right)
        },
        None => true
    }
}
fn checking(left : &Option<Rc<RefCell<TreeNode>>>, right : &Option<Rc<RefCell<TreeNode>>>) -> bool{
    match (left,right) {
        (None, None) => true,
        (Some(_), None) => false,
        (None, Some(_)) => false,
        (Some(left), Some(right)) => {
            let borrow_left = left.borrow();
            let borrow_right = right.borrow();
            checking(&borrow_left.left, &borrow_right.right) && checking(&borrow_left.right, &borrow_right.left) && borrow_left.val == borrow_right.val
        }
    }
}

