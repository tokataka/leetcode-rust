///
/// # 119. Pascal's Triangle II
///
/// Given an integer `rowIndex`, return the `rowIndex<sup>th</sup>` (**0-indexed**) row of the **Pascal's triangle**.
///
/// In **Pascal's triangle**, each number is the sum of the two numbers directly above it as shown:
///
/// ![](https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif)
///
/// **Example 1:**
///
/// ```
/// Input: rowIndex = 3
/// Output: [1,3,3,1]
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: rowIndex = 0
/// Output: [1]
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: rowIndex = 1
/// Output: [1,1]
///
/// ```
///
/// **Constraints:**
///
/// * `0 <= rowIndex <= 33`
///
/// **Follow up:** Could you optimize your algorithm to use only `O(rowIndex)` extra space?
///
pub struct Solution {}

// problem: https://leetcode.com/problems/pascals-triangle-ii/
// discuss: https://leetcode.com/problems/pascals-triangle-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut result = vec![1];

        for r in 1..=row_index as usize {
            for i in (1..r).rev() {
                result[i] += result[i - 1];
            }

            result.push(1);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_119() {
        let row_index = 3;
        let expected = vec![1, 3, 3, 1];
        assert_eq!(Solution::get_row(row_index), expected);
        let row_index = 0;
        let expected = vec![1];
        assert_eq!(Solution::get_row(row_index), expected);
        let row_index = 1;
        let expected = vec![1, 1];
        assert_eq!(Solution::get_row(row_index), expected);
    }
}
