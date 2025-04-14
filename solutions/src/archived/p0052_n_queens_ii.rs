///
/// # 52. N-Queens II
///
/// The **n-queens** puzzle is the problem of placing `n` queens on an `n x n` chessboard such that no two queens attack each other.
///
/// Given an integer `n`, return *the number of distinct solutions to the **n-queens puzzle***.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2020/11/13/queens.jpg)
///
/// ```
/// Input: n = 4
/// Output: 2
/// Explanation: There are two distinct solutions to the 4-queens puzzle as shown.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 1
/// Output: 1
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 9`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/n-queens-ii/
// discuss: https://leetcode.com/problems/n-queens-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        fn _n_queens(i: usize, queens: &mut Vec<usize>, n: usize) -> i32 {
            if i == n {
                return 1;
            }

            let mut result = 0;

            for j in 0..n {
                if queens
                    .iter()
                    .enumerate()
                    .any(|(qi, &qj)| j == qj || i - qi == j.abs_diff(qj))
                {
                    continue;
                }

                queens.push(j);
                result += _n_queens(i + 1, queens, n);
                queens.pop();
            }

            result
        }

        _n_queens(0, &mut vec![], n as usize)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_52() {
        let n = 4;
        let expected = 2;
        assert_eq!(Solution::total_n_queens(n), expected);
        let n = 1;
        let expected = 1;
        assert_eq!(Solution::total_n_queens(n), expected);
    }
}
