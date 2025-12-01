///
/// # 2536. Increment Submatrices by One
///
/// You are given a positive integer `n`, indicating that we initially have an `n x n` **0-indexed** integer matrix `mat` filled with zeroes.
///
/// You are also given a 2D integer array `query`. For each `query[i] = [row1<sub>i</sub>, col1<sub>i</sub>, row2<sub>i</sub>, col2<sub>i</sub>]`, you should do the following operation:
///
/// * Add `1` to **every element** in the submatrix with the **top left** corner `(row1<sub>i</sub>, col1<sub>i</sub>)` and the **bottom right** corner `(row2<sub>i</sub>, col2<sub>i</sub>)`. That is, add `1` to `mat[x][y]` for all `row1<sub>i</sub> <= x <= row2<sub>i</sub>` and `col1<sub>i</sub> <= y <= col2<sub>i</sub>`.
///
/// Return *the matrix* `mat` *after performing every query.*
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2022/11/24/p2example11.png)
///
/// ```
/// Input: n = 3, queries = [[1,1,2,2],[0,0,1,1]]
/// Output: [[1,1,0],[1,2,1],[0,1,1]]
/// Explanation: The diagram above shows the initial matrix, the matrix after the first query, and the matrix after the second query.
/// - In the first query, we add 1 to every element in the submatrix with the top left corner (1, 1) and bottom right corner (2, 2).
/// - In the second query, we add 1 to every element in the submatrix with the top left corner (0, 0) and bottom right corner (1, 1).
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2022/11/24/p2example22.png)
///
/// ```
/// Input: n = 2, queries = [[0,0,1,1]]
/// Output: [[1,1],[1,1]]
/// Explanation: The diagram above shows the initial matrix and the matrix after the first query.
/// - In the first query we add 1 to every element in the matrix.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 500`
/// * `1 <= queries.length <= 10<sup>4</sup>`
/// * `0 <= row1<sub>i</sub> <= row2<sub>i</sub> < n`
/// * `0 <= col1<sub>i</sub> <= col2<sub>i</sub> < n`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/increment-submatrices-by-one/
// discuss: https://leetcode.com/problems/increment-submatrices-by-one/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;

        let mut result = vec![vec![0; n]; n];

        for query in queries {
            let (r1, c1, r2, c2) = (
                query[0] as usize,
                query[1] as usize,
                query[2] as usize,
                query[3] as usize,
            );

            for r in r1..=r2 {
                result[r][c1] += 1;
                if c2 + 1 < n {
                    result[r][c2 + 1] -= 1;
                }
            }
        }

        for r in 0..n {
            for c in 1..n {
                result[r][c] += result[r][c - 1];
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2536() {
        let n = 3;
        let queries = nd_vec![[1, 1, 2, 2], [0, 0, 1, 1]];
        let expected = nd_vec![[1, 1, 0], [1, 2, 1], [0, 1, 1]];
        assert_eq!(Solution::range_add_queries(n, queries), expected);
        let n = 2;
        let queries = nd_vec![[0, 0, 1, 1]];
        let expected = nd_vec![[1, 1], [1, 1]];
        assert_eq!(Solution::range_add_queries(n, queries), expected);
    }
}
