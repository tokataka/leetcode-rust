///
/// # 1290. Convert Binary Number in a Linked List to Integer
///
/// Given `head` which is a reference node to a singly-linked list. The value of each node in the linked list is either `0` or `1`. The linked list holds the binary representation of a number.
///
/// Return the *decimal value* of the number in the linked list.
///
/// The **most significant bit** is at the head of the linked list.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2019/12/05/graph-1.png)
///
/// ```
/// Input: head = [1,0,1]
/// Output: 5
/// Explanation: (101) in base 2 = (5) in base 10
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: head = [0]
/// Output: 0
///
/// ```
///
/// **Constraints:**
///
/// * The Linked List is not empty.
/// * Number of nodes will not exceed `30`.
/// * Each node's value is either `0` or `1`.
///
pub struct Solution {}
use crate::util::linked_list::ListNode;

// problem: https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/
// discuss: https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut result = 0;
        let mut cur = &head;

        while let Some(x) = cur.as_ref() {
            result = (result << 1) + x.val;
            cur = &x.next;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1290() {
        let head = linked![1, 0, 1];
        let expected = 5;
        assert_eq!(Solution::get_decimal_value(head), expected);
        let head = linked![0];
        let expected = 0;
        assert_eq!(Solution::get_decimal_value(head), expected);
    }
}
