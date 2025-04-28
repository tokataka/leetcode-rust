///
/// # 82. Remove Duplicates from Sorted List II
///
/// Given the `head` of a sorted linked list, *delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list*. Return *the linked list **sorted** as well*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/01/04/linkedlist1.jpg)
///
/// ```
/// Input: head = [1,2,3,3,4,4,5]
/// Output: [1,2,5]
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/01/04/linkedlist2.jpg)
///
/// ```
/// Input: head = [1,1,1,2,3]
/// Output: [2,3]
///
/// ```
///
/// **Constraints:**
///
/// * The number of nodes in the list is in the range `[0, 300]`.
/// * `-100 <= Node.val <= 100`
/// * The list is guaranteed to be **sorted** in ascending order.
///
pub struct Solution {}
use crate::util::linked_list::ListNode;

// problem: https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/
// discuss: https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut result_tail = &mut result;

        let mut temp = None;
        let mut temp_val = i32::MIN;

        let mut cur = head;

        while let Some(mut x) = cur {
            cur = x.next.take();

            if temp_val < x.val {
                temp_val = x.val;
                if let Some(t) = temp.replace(x) {
                    result_tail = &mut result_tail.insert(t).next;
                }
            } else {
                temp = None;
            }
        }

        *result_tail = temp;

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_82() {
        let head = linked![1, 2, 3, 3, 4, 4, 5];
        let expected = linked![1, 2, 5];
        assert_eq!(Solution::delete_duplicates(head), expected);
        let head = linked![1, 1, 1, 2, 3];
        let expected = linked![2, 3];
        assert_eq!(Solution::delete_duplicates(head), expected);
    }
}
