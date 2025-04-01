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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut graph = vec![vec![]];
        let mut st = vec![(root.unwrap(), 0)];
        let mut cur_idx = 1;

        while let Some((cur, idx)) = st.pop() {
            let mut cur = cur.as_ref().borrow_mut();

            for child in [cur.left.take(), cur.right.take()].into_iter().flatten() {
                graph[idx].push(cur_idx);
                graph.push(vec![idx]);
                st.push((child, cur_idx));
                cur_idx += 1;
            }
        }

        if graph.len() == 1 {
            return 1;
        }

        let mut q = graph
            .iter()
            .enumerate()
            .filter(|&(_, x)| x.len() == 1)
            .map(|(i, _)| i)
            .collect::<VecDeque<_>>();

        let mut camera = vec![false; graph.len()];
        let mut monitoring = vec![false; graph.len()];

        while let Some(cur) = q.pop_front() {
            if graph[cur].is_empty() {
                if !monitoring[cur] {
                    camera[cur] = true;
                }
                continue;
            }

            let c = graph[cur][0];

            if camera[c] {
                continue;
            }

            camera[c] = true;
            monitoring[c] = true;

            graph.push(vec![]);
            let edges_c = graph.swap_remove(c);

            for &x in &edges_c {
                if let Some(idx) = graph[x].iter().position(|&t| t == c) {
                    graph[x].swap_remove(idx);
                    monitoring[x] = true;
                }
            }

            for &x in &edges_c {
                if graph[x].len() == 1 {
                    let y = graph[x][0];

                    if let Some(idx) = graph[y].iter().position(|&t| t == x) {
                        graph[y].swap_remove(idx);
                        graph[x].clear();
                        q.push_back(y);
                    }
                }
            }
        }

        camera.iter().filter(|&&x| x).count() as i32
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
