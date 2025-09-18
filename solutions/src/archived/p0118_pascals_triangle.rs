///
/// # 118. Pascal's Triangle
///
/// Given an integer `numRows`, return the first numRows of **Pascal's triangle**.
///
/// In **Pascal's triangle**, each number is the sum of the two numbers directly above it as shown:
///
/// ![](https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif)
///
/// **Example 1:**
///
/// ```
/// Input: numRows = 5
/// Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: numRows = 1
/// Output: [[1]]
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= numRows <= 30`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/pascals-triangle/
// discuss: https://leetcode.com/problems/pascals-triangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![1]];

        for _ in 1..num_rows {
            let mut row = vec![1];

            for x in result.last().unwrap().windows(2) {
                row.push(x[0] + x[1]);
            }

            row.push(1);

            result.push(row);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_118() {
        let num_rows = 5;
        let expected = nd_vec![[1], [1, 1], [1, 2, 1], [1, 3, 3, 1], [1, 4, 6, 4, 1]];
        assert_eq!(Solution::generate(num_rows), expected);
        let num_rows = 1;
        let expected = nd_vec![[1]];
        assert_eq!(Solution::generate(num_rows), expected);
    }
}
