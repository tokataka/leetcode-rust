///
/// # 25. Reverse Nodes in k-Group
///
/// Given the `head` of a linked list, reverse the nodes of the list `k` at a time, and return *the modified list*.
///
/// `k` is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of `k` then left-out nodes, in the end, should remain as it is.
///
/// You may not alter the values in the list's nodes, only nodes themselves may be changed.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2020/10/03/reverse_ex1.jpg)
///
/// ```
/// Input: head = [1,2,3,4,5], k = 2
/// Output: [2,1,4,3,5]
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2020/10/03/reverse_ex2.jpg)
///
/// ```
/// Input: head = [1,2,3,4,5], k = 3
/// Output: [3,2,1,4,5]
///
/// ```
///
/// **Constraints:**
///
/// * The number of nodes in the list is `n`.
/// * `1 <= k <= n <= 5000`
/// * `0 <= Node.val <= 1000`
///
/// **Follow-up:** Can you solve the problem in `O(1)` extra memory space?
///
pub struct Solution {}
use crate::util::linked_list::ListNode;

// problem: https://leetcode.com/problems/reverse-nodes-in-k-group/
// discuss: https://leetcode.com/problems/reverse-nodes-in-k-group/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let k = k as usize;

        let mut cur = &mut head;
        let mut need_swap = Vec::with_capacity(k);

        while let Some(x) = cur {
            need_swap.push(&mut x.val);

            if need_swap.len() == k {
                for i in 0..k / 2 {
                    (*need_swap[i], *need_swap[k - 1 - i]) = (*need_swap[k - 1 - i], *need_swap[i]);
                }

                need_swap.clear();
            }

            cur = &mut x.next;
        }

        head
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_25() {
        let head = linked![1, 2, 3, 4, 5];
        let k = 2;
        let expected = linked![2, 1, 4, 3, 5];
        assert_eq!(Solution::reverse_k_group(head, k), expected);
        let head = linked![1, 2, 3, 4, 5];
        let k = 3;
        let expected = linked![3, 2, 1, 4, 5];
        assert_eq!(Solution::reverse_k_group(head, k), expected);
    }
}
