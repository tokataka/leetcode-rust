///
/// # 124. Binary Tree Maximum Path Sum
///
/// A **path** in a binary tree is a sequence of nodes where each pair of adjacent nodes in the sequence has an edge connecting them. A node can only appear in the sequence **at most once**. Note that the path does not need to pass through the root.
///
/// The **path sum** of a path is the sum of the node's values in the path.
///
/// Given the `root` of a binary tree, return *the maximum **path sum** of any **non-empty** path*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2020/10/13/exx1.jpg)
///
/// ```
/// Input: root = [1,2,3]
/// Output: 6
/// Explanation: The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 = 6.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2020/10/13/exx2.jpg)
///
/// ```
/// Input: root = [-10,9,20,null,null,15,7]
/// Output: 42
/// Explanation: The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 + 7 = 42.
///
/// ```
///
/// **Constraints:**
///
/// * The number of nodes in the tree is in the range `[1, 3 * 10<sup>4</sup>]`.
/// * `-1000 <= Node.val <= 1000`
///
pub struct Solution {}
use crate::util::tree::TreeNode;

// problem: https://leetcode.com/problems/binary-tree-maximum-path-sum/
// discuss: https://leetcode.com/problems/binary-tree-maximum-path-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn _max_path_sum(cur: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            let cur = match cur.as_ref() {
                Some(cur) => cur,
                None => return (i32::MIN, i32::MIN),
            };

            let cur = cur.borrow();

            let (left_max_sum, left_max_path) = _max_path_sum(&cur.left);
            let (right_max_sum, right_max_path) = _max_path_sum(&cur.right);

            (
                left_max_sum
                    .max(right_max_sum)
                    .max(cur.val + left_max_path.max(0) + right_max_path.max(0)),
                cur.val + left_max_path.max(right_max_path).max(0),
            )
        }

        _max_path_sum(&root).0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_124() {
        let root = tree![1, 2, 3];
        let expected = 6;
        assert_eq!(Solution::max_path_sum(root), expected);
        let root = tree![-10, 9, 20, null, null, 15, 7];
        let expected = 42;
        assert_eq!(Solution::max_path_sum(root), expected);
    }
}
