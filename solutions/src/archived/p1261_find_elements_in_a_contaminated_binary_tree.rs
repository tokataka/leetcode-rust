///
/// # 1261. Find Elements in a Contaminated Binary Tree
///
/// Given a binary tree with the following rules:
///
/// 1. `root.val == 0`
/// 2. For any `treeNode`:
///    1. If `treeNode.val` has a value `x` and `treeNode.left != null`, then `treeNode.left.val == 2 * x + 1`
///    2. If `treeNode.val` has a value `x` and `treeNode.right != null`, then `treeNode.right.val == 2 * x + 2`
///
/// Now the binary tree is contaminated, which means all `treeNode.val` have been changed to `-1`.
///
/// Implement the `FindElements` class:
///
/// * `FindElements(TreeNode* root)` Initializes the object with a contaminated binary tree and recovers it.
/// * `bool find(int target)` Returns `true` if the `target` value exists in the recovered binary tree.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2019/11/06/untitled-diagram-4-1.jpg)
///
/// ```
/// Input
/// ["FindElements","find","find"]
/// [[[-1,null,-1]],[1],[2]]
/// Output
/// [null,false,true]
/// Explanation
/// FindElements findElements = new FindElements([-1,null,-1]);
/// findElements.find(1); // return False
/// findElements.find(2); // return True
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2019/11/06/untitled-diagram-4.jpg)
///
/// ```
/// Input
/// ["FindElements","find","find","find"]
/// [[[-1,-1,-1,-1,-1]],[1],[3],[5]]
/// Output
/// [null,true,true,false]
/// Explanation
/// FindElements findElements = new FindElements([-1,-1,-1,-1,-1]);
/// findElements.find(1); // return True
/// findElements.find(3); // return True
/// findElements.find(5); // return False
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2019/11/07/untitled-diagram-4-1-1.jpg)
///
/// ```
/// Input
/// ["FindElements","find","find","find","find"]
/// [[[-1,null,-1,-1,null,-1]],[2],[3],[4],[5]]
/// Output
/// [null,true,false,false,true]
/// Explanation
/// FindElements findElements = new FindElements([-1,null,-1,-1,null,-1]);
/// findElements.find(2); // return True
/// findElements.find(3); // return False
/// findElements.find(4); // return False
/// findElements.find(5); // return True
///
/// ```
///
/// **Constraints:**
///
/// * `TreeNode.val == -1`
/// * The height of the binary tree is less than or equal to `20`
/// * The total number of nodes is between `[1, 10<sup>4</sup>]`
/// * Total calls of `find()` is between `[1, 10<sup>4</sup>]`
/// * `0 <= target <= 10<sup>6</sup>`
///
pub struct Solution {}
use crate::util::tree::TreeNode;
use std::{cell::RefCell, collections::HashSet, rc::Rc};

// problem: https://leetcode.com/problems/find-elements-in-a-contaminated-binary-tree/
// discuss: https://leetcode.com/problems/find-elements-in-a-contaminated-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct FindElements {
    exists: HashSet<i32>,
}

#[allow(dead_code)]
impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut exists = HashSet::new();

        let mut cur_depth_nodes = vec![(root, 0)];
        let mut next_depth_nodes = vec![];

        while !cur_depth_nodes.is_empty() {
            for (node, idx) in cur_depth_nodes.drain(..) {
                let node = match node {
                    Some(node) => node,
                    None => continue,
                };

                exists.insert(idx);

                let node = node.borrow();

                next_depth_nodes.push((node.left.clone(), idx * 2 + 1));
                next_depth_nodes.push((node.right.clone(), idx * 2 + 2));
            }

            std::mem::swap(&mut cur_depth_nodes, &mut next_depth_nodes);
        }

        Self { exists }
    }

    fn find(&self, target: i32) -> bool {
        self.exists.contains(&target)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1261() {
        let obj = FindElements::new(tree![-1, null, -1]);
        assert!(!obj.find(1));
        assert!(obj.find(2));
        let obj = FindElements::new(tree![-1, -1, -1, -1, -1]);
        assert!(obj.find(1));
        assert!(obj.find(3));
        assert!(!obj.find(5));
        let obj = FindElements::new(tree![-1, null, -1, -1, null, -1]);
        assert!(obj.find(2));
        assert!(!obj.find(3));
        assert!(!obj.find(4));
        assert!(obj.find(5));
    }
}
