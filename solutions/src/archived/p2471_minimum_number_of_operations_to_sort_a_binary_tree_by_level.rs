///
/// # 2471. Minimum Number of Operations to Sort a Binary Tree by Level
///
/// You are given the `root` of a binary tree with **unique values**.
///
/// In one operation, you can choose any two nodes **at the same level** and swap their values.
///
/// Return *the minimum number of operations needed to make the values at each level sorted in a **strictly increasing order***.
///
/// The **level** of a node is the number of edges along the path between it and the root node*.*
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2022/09/18/image-20220918174006-2.png)
///
/// ```
/// Input: root = [1,4,3,7,6,8,5,null,null,null,null,9,null,10]
/// Output: 3
/// Explanation:
/// - Swap 4 and 3. The 2nd level becomes [3,4].
/// - Swap 7 and 5. The 3rd level becomes [5,6,8,7].
/// - Swap 8 and 7. The 3rd level becomes [5,6,7,8].
/// We used 3 operations so return 3.
/// It can be proven that 3 is the minimum number of operations needed.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2022/09/18/image-20220918174026-3.png)
///
/// ```
/// Input: root = [1,3,2,7,6,5,4]
/// Output: 3
/// Explanation:
/// - Swap 3 and 2. The 2nd level becomes [2,3].
/// - Swap 7 and 4. The 3rd level becomes [4,6,5,7].
/// - Swap 6 and 5. The 3rd level becomes [4,5,6,7].
/// We used 3 operations so return 3.
/// It can be proven that 3 is the minimum number of operations needed.
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2022/09/18/image-20220918174052-4.png)
///
/// ```
/// Input: root = [1,2,3,4,5,6]
/// Output: 0
/// Explanation: Each level is already sorted in increasing order so return 0.
///
/// ```
///
/// **Constraints:**
///
/// * The number of nodes in the tree is in the range `[1, 10<sup>5</sup>]`.
/// * `1 <= Node.val <= 10<sup>5</sup>`
/// * All the values of the tree are **unique**.
///
pub struct Solution {}
use crate::util::tree::TreeNode;

// problem: https://leetcode.com/problems/minimum-number-of-operations-to-sort-a-binary-tree-by-level/
// discuss: https://leetcode.com/problems/minimum-number-of-operations-to-sort-a-binary-tree-by-level/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut cur_level = vec![root.unwrap()];
        let mut next_level = vec![];

        let mut result = 0;

        while !cur_level.is_empty() {
            let mut values = vec![];

            for node in cur_level.drain(..) {
                let mut node = node.borrow_mut();

                values.push((node.val, values.len()));

                next_level.extend([node.left.take(), node.right.take()].into_iter().flatten());
            }

            values.sort_unstable();

            for to in 0..values.len() - 1 {
                loop {
                    let from = values[to].1;

                    if from == to {
                        break;
                    }

                    values.swap(from, to);
                    result += 1;
                }
            }

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
    fn test_2471() {
        let root = tree![1, 4, 3, 7, 6, 8, 5, null, null, null, null, 9, null, 10];
        let expected = 3;
        assert_eq!(Solution::minimum_operations(root), expected);
        let root = tree![1, 3, 2, 7, 6, 5, 4];
        let expected = 3;
        assert_eq!(Solution::minimum_operations(root), expected);
        let root = tree![1, 2, 3, 4, 5, 6];
        let expected = 0;
        assert_eq!(Solution::minimum_operations(root), expected);
    }
}
