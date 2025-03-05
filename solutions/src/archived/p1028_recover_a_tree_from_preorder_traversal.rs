///
/// # 1028. Recover a Tree From Preorder Traversal
///
/// We run a preorder depth-first search (DFS) on the `root` of a binary tree.
///
/// At each node in this traversal, we output `D` dashes (where `D` is the depth of this node), then we output the value of this node. If the depth of a node is `D`, the depth of its immediate child is `D + 1`. The depth of the `root` node is `0`.
///
/// If a node has only one child, that child is guaranteed to be **the left child**.
///
/// Given the output `traversal` of this traversal, recover the tree and return *its* `root`.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2024/09/10/recover_tree_ex1.png)
///
/// ```
/// Input: traversal = "1-2--3--4-5--6--7"
/// Output: [1,2,5,3,4,6,7]
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2024/09/10/recover_tree_ex2.png)
///
/// ```
/// Input: traversal = "1-2--3---4-5--6---7"
/// Output: [1,2,5,3,null,6,null,4,null,7]
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2024/09/10/recover_tree_ex3.png)
///
/// ```
/// Input: traversal = "1-401--349---90--88"
/// Output: [1,401,null,349,88,90]
///
/// ```
///
/// **Constraints:**
///
/// * The number of nodes in the original tree is in the range `[1, 1000]`.
/// * `1 <= Node.val <= 10<sup>9</sup>`
///
pub struct Solution {}
use crate::util::tree::TreeNode;

// problem: https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/
// discuss: https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn _recover_from_preorder(
        traversal_idx: &mut usize,
        depth: i32,
        traversal: &str,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if *traversal_idx >= traversal.len() {
            return None;
        }

        let numeric_pos = *traversal_idx
            + traversal
                .chars()
                .skip(*traversal_idx)
                .position(|ch| ch != '-')
                .unwrap();

        let next_depth = (numeric_pos - *traversal_idx) as i32;

        if next_depth <= depth {
            return None;
        }

        let dash_pos = match traversal.chars().skip(numeric_pos).position(|ch| ch == '-') {
            Some(x) => numeric_pos + x,
            None => traversal.len(),
        };

        let val = traversal[numeric_pos..dash_pos].parse().unwrap();
        *traversal_idx = dash_pos;

        let mut node = TreeNode::new(val);

        node.left = Self::_recover_from_preorder(traversal_idx, depth + 1, traversal);
        node.right = Self::_recover_from_preorder(traversal_idx, depth + 1, traversal);

        Some(Rc::new(RefCell::new(node)))
    }

    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        Self::_recover_from_preorder(&mut 0, -1, &traversal)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1028() {
        let traversal = "1-2--3--4-5--6--7".to_owned();
        let expected = tree![1, 2, 5, 3, 4, 6, 7];
        assert_eq!(Solution::recover_from_preorder(traversal), expected);
        let traversal = "1-2--3---4-5--6---7".to_owned();
        let expected = tree![1, 2, 5, 3, null, 6, null, 4, null, 7];
        assert_eq!(Solution::recover_from_preorder(traversal), expected);
        let traversal = "1-401--349---90--88".to_owned();
        let expected = tree![1, 401, null, 349, 88, 90];
        assert_eq!(Solution::recover_from_preorder(traversal), expected);
    }
}
