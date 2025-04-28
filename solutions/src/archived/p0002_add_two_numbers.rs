///
/// # 2. Add Two Numbers
///
/// You are given two **non-empty** linked lists representing two non-negative integers. The digits are stored in **reverse order**, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
///
/// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2020/10/02/addtwonumber1.jpg)
///
/// ```
/// Input: l1 = [2,4,3], l2 = [5,6,4]
/// Output: [7,0,8]
/// Explanation: 342 + 465 = 807.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: l1 = [0], l2 = [0]
/// Output: [0]
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
/// Output: [8,9,9,9,0,0,0,1]
///
/// ```
///
/// **Constraints:**
///
/// * The number of nodes in each linked list is in the range `[1, 100]`.
/// * `0 <= Node.val <= 9`
/// * It is guaranteed that the list represents a number that does not have leading zeros.
///
pub struct Solution {}
use crate::util::linked_list::ListNode;

// problem: https://leetcode.com/problems/add-two-numbers/
// discuss: https://leetcode.com/problems/add-two-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut result_tail = &mut result;
        let mut carryover = 0;

        while l1.is_some() || l2.is_some() {
            if l1.is_none() {
                (l1, l2) = (l2, l1);
            }

            let mut x1 = l1.unwrap();
            l1 = x1.next.take();

            let mut sum = carryover + x1.val;

            if let Some(x2) = l2 {
                sum += x2.val;
                l2 = x2.next;
            }

            x1.val = sum % 10;

            result_tail = &mut result_tail.insert(x1).next;
            carryover = sum / 10;
        }

        if carryover == 1 {
            *result_tail = Some(Box::new(ListNode { val: 1, next: None }));
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        let l1 = linked![2, 4, 3];
        let l2 = linked![5, 6, 4];
        let expected = linked![7, 0, 8];
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
        let l1 = linked![0];
        let l2 = linked![0];
        let expected = linked![0];
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
        let l1 = linked![9, 9, 9, 9, 9, 9, 9];
        let l2 = linked![9, 9, 9, 9];
        let expected = linked![8, 9, 9, 9, 0, 0, 0, 1];
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }
}
