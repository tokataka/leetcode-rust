///
/// # 3217. Delete Nodes From Linked List Present in Array
///
/// You are given an array of integers `nums` and the `head` of a linked list. Return the `head` of the modified linked list after **removing** all nodes from the linked list that have a value that exists in `nums`.
///
/// **Example 1:**
///
/// **Input:** nums = [1,2,3], head = [1,2,3,4,5]
///
/// **Output:** [4,5]
///
/// **Explanation:**
///
/// **![](https://assets.leetcode.com/uploads/2024/06/11/linkedlistexample0.png)**
///
/// Remove the nodes with values 1, 2, and 3.
///
/// **Example 2:**
///
/// **Input:** nums = [1], head = [1,2,1,2,1,2]
///
/// **Output:** [2,2,2]
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/06/11/linkedlistexample1.png)
///
/// Remove the nodes with value 1.
///
/// **Example 3:**
///
/// **Input:** nums = [5], head = [1,2,3,4]
///
/// **Output:** [1,2,3,4]
///
/// **Explanation:**
///
/// **![](https://assets.leetcode.com/uploads/2024/06/11/linkedlistexample2.png)**
///
/// No node has value 5.
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>5</sup>`
/// * All elements in `nums` are unique.
/// * The number of nodes in the given list is in the range `[1, 10<sup>5</sup>]`.
/// * `1 <= Node.val <= 10<sup>5</sup>`
/// * The input is generated such that there is at least one node in the linked list that has a value not present in `nums`.
///
pub struct Solution {}
use std::collections::HashSet;

use crate::util::linked_list::ListNode;

// problem: https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array/
// discuss: https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let nums = nums.into_iter().collect::<HashSet<_>>();
        let mut head = head;
        let mut cur = &mut head;

        while let Some(mut x) = cur.take() {
            if nums.contains(&x.val) {
                *cur = x.next.take();
            } else {
                cur = &mut cur.insert(x).next;
            }
        }

        head
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3217() {
        let nums = vec![1, 2, 3];
        let head = linked![1, 2, 3, 4, 5];
        let expected = linked![4, 5];
        assert_eq!(Solution::modified_list(nums, head), expected);
        let nums = vec![1];
        let head = linked![1, 2, 1, 2, 1, 2];
        let expected = linked![2, 2, 2];
        assert_eq!(Solution::modified_list(nums, head), expected);
        let nums = vec![5];
        let head = linked![1, 2, 3, 4];
        let expected = linked![1, 2, 3, 4];
        assert_eq!(Solution::modified_list(nums, head), expected);
    }
}
