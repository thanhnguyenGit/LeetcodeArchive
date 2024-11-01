// Given the root of a binary tree, return the inorder traversal of its nodes' values.

use std::cell::RefCell;
use std::rc::Rc;
use crate::helper::treenode::TreeNode;

// root null = None
// root have val = Some(val)
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    traversal(root,&mut result);
    println!("final state {:?}", result);
    result
}
fn traversal(node : Option<Rc<RefCell<TreeNode>>>, result : &mut Vec<i32>) {
    if let Some(node) = node {
        traversal(node.borrow().left.clone(),result);
        result.push(node.borrow().val);
        println!("result state : {:?}", result);
        println!("val push in {}", node.borrow().val);
        traversal(node.borrow().right.clone(),result);
    }
}