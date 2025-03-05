///
/// # 889. Construct Binary Tree from Preorder and Postorder Traversal
///
/// Given two integer arrays, `preorder` and `postorder` where `preorder` is the preorder traversal of a binary tree of **distinct** values and `postorder` is the postorder traversal of the same tree, reconstruct and return *the binary tree*.
///
/// If there exist multiple answers, you can **return any** of them.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/07/24/lc-prepost.jpg)
///
/// ```
/// Input: preorder = [1,2,4,5,3,6,7], postorder = [4,5,2,6,7,3,1]
/// Output: [1,2,3,4,5,6,7]
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: preorder = [1], postorder = [1]
/// Output: [1]
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= preorder.length <= 30`
/// * `1 <= preorder[i] <= preorder.length`
/// * All the values of `preorder` are **unique**.
/// * `postorder.length == preorder.length`
/// * `1 <= postorder[i] <= postorder.length`
/// * All the values of `postorder` are **unique**.
/// * It is guaranteed that `preorder` and `postorder` are the preorder traversal and postorder traversal of the same binary tree.
///
pub struct Solution {}
use crate::util::tree::TreeNode;

// problem: https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/
// discuss: https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn _construct_from_pre_post(
        pre_idx: &mut usize,
        post_idx: &mut usize,
        path: &mut Vec<i32>,
        preorder: &[i32],
        postorder: &[i32],
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if path.contains(&postorder[*post_idx]) {
            if let Some(&x) = path.last() {
                if x == postorder[*post_idx] {
                    *post_idx += 1;
                }
                path.pop();
                return None;
            }
        }

        if *pre_idx >= preorder.len() {
            return None;
        }

        let val = preorder[*pre_idx];

        *pre_idx += 1;

        let mut cur = TreeNode::new(val);
        path.push(val);

        if val == postorder[*post_idx] {
            *post_idx += 1;
            return Some(Rc::new(RefCell::new(cur)));
        }

        cur.left = Self::_construct_from_pre_post(pre_idx, post_idx, path, preorder, postorder);
        cur.right = Self::_construct_from_pre_post(pre_idx, post_idx, path, preorder, postorder);

        assert_eq!(cur.val, postorder[*post_idx]);
        *post_idx += 1;

        Some(Rc::new(RefCell::new(cur)))
    }

    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::_construct_from_pre_post(&mut 0, &mut 0, &mut vec![], &preorder, &postorder)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_889() {
        // let preorder = vec![1, 2, 4, 5, 3, 6, 7];
        // let postorder = vec![4, 5, 2, 6, 7, 3, 1];
        // let expected = tree![1, 2, 3, 4, 5, 6, 7];
        // assert_eq!(
        //     Solution::construct_from_pre_post(preorder, postorder),
        //     expected
        // );
        // let preorder = vec![1];
        // let postorder = vec![1];
        // let expected = tree![1];
        // assert_eq!(
        //     Solution::construct_from_pre_post(preorder, postorder),
        //     expected
        // );
        let preorder = vec![3, 4, 1, 2];
        let postorder = vec![1, 4, 2, 3];
        let expected = tree![3, 4, 2, 1];
        assert_eq!(
            Solution::construct_from_pre_post(preorder, postorder),
            expected
        );
    }
}
