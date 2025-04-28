///
/// # 86. Partition List
///
/// Given the `head` of a linked list and a value `x`, partition it such that all nodes **less than** `x` come before nodes **greater than or equal** to `x`.
///
/// You should **preserve** the original relative order of the nodes in each of the two partitions.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/01/04/partition.jpg)
///
/// ```
/// Input: head = [1,4,3,2,5,2], x = 3
/// Output: [1,2,2,4,3,5]
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: head = [2,1], x = 2
/// Output: [1,2]
///
/// ```
///
/// **Constraints:**
///
/// * The number of nodes in the list is in the range `[0, 200]`.
/// * `-100 <= Node.val <= 100`
/// * `-200 <= x <= 200`
///
pub struct Solution {}
use crate::util::linked_list::ListNode;

// problem: https://leetcode.com/problems/partition-list/
// discuss: https://leetcode.com/problems/partition-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut left = None;
        let mut right = None;

        let mut left_tail = &mut left;
        let mut right_tail = &mut right;

        let mut cur = head;

        while let Some(mut node) = cur {
            cur = node.next.take();

            if node.val < x {
                left_tail = &mut left_tail.insert(node).next;
            } else {
                right_tail = &mut right_tail.insert(node).next;
            }
        }

        *left_tail = right;

        left
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_86() {
        let head = linked![1, 4, 3, 2, 5, 2];
        let x = 3;
        let expected = linked![1, 2, 2, 4, 3, 5];
        assert_eq!(Solution::partition(head, x), expected);
        let head = linked![2, 1];
        let x = 2;
        let expected = linked![1, 2];
        assert_eq!(Solution::partition(head, x), expected);
    }
}
