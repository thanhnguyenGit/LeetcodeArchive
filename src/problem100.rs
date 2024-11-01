use std::cell::RefCell;
use std::rc::Rc;
use crate::helper::treenode::TreeNode;

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p,q) {
        (Some(ref p), Some(ref q)) => {
            let p1 = p.borrow();
            let q1 = q.borrow();
            p1.val == q1.val && is_same_tree(p1.left.clone(),q1.left.clone()) && is_same_tree(p1.right.clone(),q1.right.clone())
        }
        (None, None) => true,
        (Some(_), None) => false,
        (None, Some(_)) => false,
    }
}