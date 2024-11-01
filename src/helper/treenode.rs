use std::cell::RefCell;
use std::rc::Rc;

// binary tree node, use Rc<RefCell> for tree, graph DSA (refer to the book)
#[derive(Debug,Eq,PartialEq)]
pub struct TreeNode {
    pub val : i32,
    pub left : Option<Rc<RefCell<TreeNode>>>,
    pub right : Option<Rc<RefCell<TreeNode>>>,
}
// constructing a new Node
impl TreeNode {
    #[inline]
    pub fn new(val : i32) -> TreeNode {
        TreeNode {
            val,
            left : None,
            right : None
        }
    }
}