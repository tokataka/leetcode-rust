///
/// # 103. Binary Tree Zigzag Level Order Traversal
///
/// Given the `root` of a binary tree, return *the zigzag level order traversal of its nodes' values*. (i.e., from left to right, then right to left for the next level and alternate between).
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/02/19/tree1.jpg)
///
/// ```
/// Input: root = [3,9,20,null,null,15,7]
/// Output: [[3],[20,9],[15,7]]
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: root = [1]
/// Output: [[1]]
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: root = []
/// Output: []
///
/// ```
///
/// **Constraints:**
///
/// * The number of nodes in the tree is in the range `[0, 2000]`.
/// * `-100 <= Node.val <= 100`
///
pub struct Solution {}
use crate::util::tree::TreeNode;

// problem: https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
// discuss: https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut cur_level = vec![root];
        let mut next_level = vec![];
        let mut is_rev = false;
        let mut result = vec![];

        loop {
            let mut level_result = vec![];

            for cur in cur_level.drain(..).rev().flatten() {
                let cur = cur.borrow();

                level_result.push(cur.val);

                if is_rev {
                    next_level.push(cur.right.clone());
                    next_level.push(cur.left.clone());
                } else {
                    next_level.push(cur.left.clone());
                    next_level.push(cur.right.clone());
                }
            }

            if next_level.is_empty() {
                break;
            }

            result.push(level_result);
            is_rev = !is_rev;
            std::mem::swap(&mut cur_level, &mut next_level);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_103() {
        let root = tree![3, 9, 20, null, null, 15, 7];
        let expected = nd_vec![[3], [20, 9], [15, 7]];
        assert_eq!(Solution::zigzag_level_order(root), expected);
        let root = tree![1];
        let expected = nd_vec![[1]];
        assert_eq!(Solution::zigzag_level_order(root), expected);
        let root = tree![];
        let expected: Vec<Vec<i32>> = nd_vec![];
        assert_eq!(Solution::zigzag_level_order(root), expected);
    }
}
