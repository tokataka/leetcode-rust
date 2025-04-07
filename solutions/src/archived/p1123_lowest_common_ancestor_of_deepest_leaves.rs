///
/// # 1123. Lowest Common Ancestor of Deepest Leaves
///
/// Given the `root` of a binary tree, return *the lowest common ancestor of its deepest leaves*.
///
/// Recall that:
///
/// * The node of a binary tree is a leaf if and only if it has no children
/// * The depth of the root of the tree is `0`. if the depth of a node is `d`, the depth of each of its children is `d + 1`.
/// * The lowest common ancestor of a set `S` of nodes, is the node `A` with the largest depth such that every node in `S` is in the subtree with root `A`.
///
/// **Example 1:**
///
/// ![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/01/sketch1.png)
///
/// ```
/// Input: root = [3,5,1,6,2,0,8,null,null,7,4]
/// Output: [2,7,4]
/// Explanation: We return the node with value 2, colored in yellow in the diagram.
/// The nodes coloured in blue are the deepest leaf-nodes of the tree.
/// Note that nodes 6, 0, and 8 are also leaf nodes, but the depth of them is 2, but the depth of nodes 7 and 4 is 3.
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: root = [1]
/// Output: [1]
/// Explanation: The root is the deepest node in the tree, and it's the lca of itself.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: root = [0,1,3,null,2]
/// Output: [2]
/// Explanation: The deepest leaf node in the tree is 2, the lca of one node is itself.
///
/// ```
///
/// **Constraints:**
///
/// * The number of nodes in the tree will be in the range `[1, 1000]`.
/// * `0 <= Node.val <= 1000`
/// * The values of the nodes in the tree are **unique**.
///
/// **Note:** This question is the same as 865: [https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/](https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/)
///
pub struct Solution {}
use crate::util::tree::TreeNode;

// problem: https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/
// discuss: https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn lca(
            node: Option<Rc<RefCell<TreeNode>>>,
            depth: i32,
        ) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
            if node.is_none() {
                return (None, -1);
            }

            let node_borrow = node.clone().unwrap();
            let node_borrow = node_borrow.borrow();

            let left = lca(node_borrow.left.clone(), depth + 1);
            let right = lca(node_borrow.right.clone(), depth + 1);

            match left.1.cmp(&right.1) {
                std::cmp::Ordering::Less => right,
                std::cmp::Ordering::Equal => (node, depth.max(left.1)),
                std::cmp::Ordering::Greater => left,
            }
        }

        lca(root, 0).0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1123() {
        let root = tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4];
        let expected = tree![2, 7, 4];
        assert_eq!(Solution::lca_deepest_leaves(root), expected);
        let root = tree![1];
        let expected = tree![1];
        assert_eq!(Solution::lca_deepest_leaves(root), expected);
        let root = tree![0, 1, 3, null, 2];
        let expected = tree![2];
        assert_eq!(Solution::lca_deepest_leaves(root), expected);
        let root = tree![1, 2, null, 3, 4, null, 6, null, 5];
        let expected = tree![2, 3, 4, null, 6, null, 5];
        assert_eq!(Solution::lca_deepest_leaves(root), expected);
    }
}
