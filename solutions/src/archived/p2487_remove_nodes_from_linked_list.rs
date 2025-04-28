///
/// # 2487. Remove Nodes From Linked List
///
/// You are given the `head` of a linked list.
///
/// Remove every node which has a node with a greater value anywhere to the right side of it.
///
/// Return *the* `head` *of the modified linked list.*
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2022/10/02/drawio.png)
///
/// ```
/// Input: head = [5,2,13,3,8]
/// Output: [13,8]
/// Explanation: The nodes that should be removed are 5, 2 and 3.
/// - Node 13 is to the right of node 5.
/// - Node 13 is to the right of node 2.
/// - Node 8 is to the right of node 3.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: head = [1,1,1,1]
/// Output: [1,1,1,1]
/// Explanation: Every node has value 1, so no nodes are removed.
///
/// ```
///
/// **Constraints:**
///
/// * The number of the nodes in the given list is in the range `[1, 10<sup>5</sup>]`.
/// * `1 <= Node.val <= 10<sup>5</sup>`
///
pub struct Solution {}
use crate::util::linked_list::ListNode;

// problem: https://leetcode.com/problems/remove-nodes-from-linked-list/
// discuss: https://leetcode.com/problems/remove-nodes-from-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn _remove_nodes(node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut node = node?;

            if let Some(next) = _remove_nodes(node.next.take()) {
                if next.val > node.val {
                    return Some(next);
                }

                node.next = Some(next);
            }

            Some(node)
        }

        _remove_nodes(head)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2487() {
        let head = linked![5, 2, 13, 3, 8];
        let expected = linked![13, 8];
        assert_eq!(Solution::remove_nodes(head), expected);
        let head = linked![1, 1, 1, 1];
        let expected = linked![1, 1, 1, 1];
        assert_eq!(Solution::remove_nodes(head), expected);
    }
}
