///
/// # 1171. Remove Zero Sum Consecutive Nodes from Linked List
///
/// Given the `head` of a linked list, we repeatedly delete consecutive sequences of nodes that sum to `0` until there are no such sequences.
///
/// After doing so, return the head of the final linked list. You may return any such answer.
///
/// (Note that in the examples below, all sequences are serializations of `ListNode` objects.)
///
/// **Example 1:**
///
/// ```
/// Input: head = [1,2,-3,3,1]
/// Output: [3,1]
/// Note: The answer [1,2,1] would also be accepted.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: head = [1,2,3,-3,4]
/// Output: [1,2,4]
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: head = [1,2,3,-3,-2]
/// Output: [1]
///
/// ```
///
/// **Constraints:**
///
/// * The given linked list will contain between `1` and `1000` nodes.
/// * Each node in the linked list has `-1000 <= node.val <= 1000`.
///
pub struct Solution {}
use std::collections::HashSet;

use crate::util::linked_list::ListNode;

// problem: https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/
// discuss: https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut rev_head: Option<Box<ListNode>> = None;

        let mut sum_occurence = HashSet::new();
        sum_occurence.insert(0);

        let mut sum = 0;

        while let Some(mut x) = head {
            head = x.next.take();
            sum += x.val;

            if !sum_occurence.insert(sum) {
                let target_sum = sum;
                sum -= x.val;

                while sum != target_sum {
                    sum_occurence.remove(&sum);

                    let mut r = rev_head.unwrap();
                    rev_head = r.next.take();
                    sum -= r.val;
                }
            } else {
                x.next = rev_head.take();
                rev_head = Some(x);
            }
        }

        let mut result = None;

        while let Some(mut x) = rev_head {
            rev_head = x.next.take();

            x.next = result.take();
            result = Some(x);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1171() {
        let head = linked![1, 2, -3, 3, 1];
        let expected = linked![3, 1];
        assert_eq!(Solution::remove_zero_sum_sublists(head), expected);
        let head = linked![1, 2, 3, -3, 4];
        let expected = linked![1, 2, 4];
        assert_eq!(Solution::remove_zero_sum_sublists(head), expected);
        let head = linked![1, 2, 3, -3, -2];
        let expected = linked![1];
        assert_eq!(Solution::remove_zero_sum_sublists(head), expected);
    }
}
