///
/// # 1130. Minimum Cost Tree From Leaf Values
///
/// Given an array `arr` of positive integers, consider all binary trees such that:
///
/// * Each node has either `0` or `2` children;
/// * The values of `arr` correspond to the values of each **leaf** in an in-order traversal of the tree.
/// * The value of each non-leaf node is equal to the product of the largest leaf value in its left and right subtree, respectively.
///
/// Among all possible binary trees considered, return *the smallest possible sum of the values of each non-leaf node*. It is guaranteed this sum fits into a **32-bit** integer.
///
/// A node is a **leaf** if and only if it has zero children.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/08/10/tree1.jpg)
///
/// ```
/// Input: arr = [6,2,4]
/// Output: 32
/// Explanation: There are two possible trees shown.
/// The first has a non-leaf node sum 36, and the second has non-leaf node sum 32.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/08/10/tree2.jpg)
///
/// ```
/// Input: arr = [4,11]
/// Output: 44
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= arr.length <= 40`
/// * `1 <= arr[i] <= 15`
/// * It is guaranteed that the answer fits into a **32-bit** signed integer (i.e., it is less than 2<sup>31</sup>).
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-tree-from-leaf-values/
// discuss: https://leetcode.com/problems/minimum-cost-tree-from-leaf-values/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut st = vec![i32::MAX];

        for x in arr {
            while x >= *st.last().unwrap() {
                result += st.pop().unwrap() * x.min(*st.last().unwrap());
            }

            st.push(x);
        }

        while st.len() > 2 {
            result += st.pop().unwrap() * st.last().unwrap();
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1130() {
        // let arr = vec![6, 2, 4];
        // let expected = 32;
        // assert_eq!(Solution::mct_from_leaf_values(arr), expected);
        // let arr = vec![4, 11];
        // let expected = 44;
        // assert_eq!(Solution::mct_from_leaf_values(arr), expected);
        let arr = vec![7, 12, 8, 10];
        let expected = 284;
        assert_eq!(Solution::mct_from_leaf_values(arr), expected);
    }
}
