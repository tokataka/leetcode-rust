///
/// # 61. Rotate List
///
/// Given the `head` of a linked list, rotate the list to the right by `k` places.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2020/11/13/rotate1.jpg)
///
/// ```
/// Input: head = [1,2,3,4,5], k = 2
/// Output: [4,5,1,2,3]
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2020/11/13/roate2.jpg)
///
/// ```
/// Input: head = [0,1,2], k = 4
/// Output: [2,0,1]
///
/// ```
///
/// **Constraints:**
///
/// * The number of nodes in the list is in the range `[0, 500]`.
/// * `-100 <= Node.val <= 100`
/// * `0 <= k <= 2 * 10<sup>9</sup>`
///
pub struct Solution {}
use crate::util::linked_list::ListNode;

// problem: https://leetcode.com/problems/rotate-list/
// discuss: https://leetcode.com/problems/rotate-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 || head.is_none() {
            return head;
        }

        let mut n = 0;
        let mut cur = &mut head;

        while let Some(x) = cur {
            n += 1;
            cur = &mut x.next;
        }

        let split = n - (k % n);

        if split == n {
            return head;
        }

        let mut root = None;
        let mut n = 0;
        let mut cur = &mut head;

        while let Some(x) = cur {
            n += 1;

            if n == split {
                root = x.next.take();
                break;
            }

            cur = &mut x.next;
        }

        let mut cur = &mut root;

        while let Some(x) = cur {
            if x.next.is_none() {
                x.next = head;
                break;
            }

            cur = &mut x.next;
        }

        root
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_61() {
        let head = linked![1, 2, 3, 4, 5];
        let k = 2;
        let expected = linked![4, 5, 1, 2, 3];
        assert_eq!(Solution::rotate_right(head, k), expected);
        let head = linked![0, 1, 2];
        let k = 4;
        let expected = linked![2, 0, 1];
        assert_eq!(Solution::rotate_right(head, k), expected);
    }
}
