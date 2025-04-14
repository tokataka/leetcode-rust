///
/// # 968. Binary Tree Cameras
///
/// You are given the `root` of a binary tree. We install cameras on the tree nodes where each camera at a node can monitor its parent, itself, and its immediate children.
///
/// Return *the minimum number of cameras needed to monitor all nodes of the tree*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2018/12/29/bst_cameras_01.png)
///
/// ```
/// Input: root = [0,0,null,0,0]
/// Output: 1
/// Explanation: One camera is enough to monitor all nodes if placed as shown.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2018/12/29/bst_cameras_02.png)
///
/// ```
/// Input: root = [0,0,null,0,null,0,null,null,0]
/// Output: 2
/// Explanation: At least two cameras are needed to monitor all nodes of the tree. The above image shows one of the valid configurations of camera placement.
///
/// ```
///
/// **Constraints:**
///
/// * The number of nodes in the tree is in the range `[1, 1000]`.
/// * `Node.val == 0`
///
pub struct Solution {}
use crate::util::tree::TreeNode;

// problem: https://leetcode.com/problems/binary-tree-cameras/
// discuss: https://leetcode.com/problems/binary-tree-cameras/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // return: need_cam, has_cam, cam_count
        fn _min_camera_cover(cur: Option<Rc<RefCell<TreeNode>>>) -> (bool, bool, i32) {
            let cur = match cur {
                Some(cur) => cur,
                _ => return (false, false, 0),
            };

            let cur = cur.borrow();

            let children_result = [cur.left.as_ref().cloned(), cur.right.as_ref().cloned()]
                .into_iter()
                .map(_min_camera_cover)
                .fold((false, false, 0), |acc, x| {
                    (acc.0 | x.0, acc.1 | x.1, acc.2 + x.2)
                });

            match children_result {
                (true, _, count) => (false, true, count + 1),
                (false, true, count) => (false, false, count),
                (false, false, count) => (true, false, count),
            }
        }

        match _min_camera_cover(root) {
            (true, _, count) => count + 1,
            (false, _, count) => count,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_968() {
        // let root = tree![0, 0, null, 0, 0];
        // let expected = 1;
        // assert_eq!(Solution::min_camera_cover(root), expected);
        // let root = tree![0, 0, null, 0, null, 0, null, null, 0];
        // let expected = 2;
        // assert_eq!(Solution::min_camera_cover(root), expected);
        let root = tree![0, 0, 0, null, 0, null, null, 0, null, null, 0, 0];
        let expected = 3;
        assert_eq!(Solution::min_camera_cover(root), expected);
        // let root = tree![0, 0, 0, 0, null, null, null, 0, 0, null, 0, null, 0];
        // let expected = 3;
        // assert_eq!(Solution::min_camera_cover(root), expected);
    }
}
