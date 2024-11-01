use std::cell::RefCell;
use std::rc::Rc;
use crate::helper::treenode::TreeNode;

pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if n == 0 {
        return vec![None]
    }
    vec![None]
}