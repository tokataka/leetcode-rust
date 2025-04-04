use std::cell::RefCell;
use std::collections::VecDeque;
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

    pub fn from_vec(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if vec.is_empty() {
            return None;
        }

        let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
        let mut queue = VecDeque::new();
        queue.push_back(head.as_ref().unwrap().clone());

        for children in vec[1..].chunks(2) {
            let parent = queue.pop_front().unwrap();
            if let Some(v) = children[0] {
                parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
            }
            if children.len() > 1 {
                if let Some(v) = children[1] {
                    parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                    queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
                }
            }
        }
        head
    }
}

#[macro_export]
macro_rules! tree {
    ($($e:expr),* $(,)?) => {TreeNode::from_vec(option_vec![$($e),*])};
}
