///
/// # 2661. First Completely Painted Row or Column
///
/// You are given a **0-indexed** integer array `arr`, and an `m x n` integer **matrix** `mat`. `arr` and `mat` both contain **all** the integers in the range `[1, m * n]`.
///
/// Go through each index `i` in `arr` starting from index `0` and paint the cell in `mat` containing the integer `arr[i]`.
///
/// Return *the smallest index* `i` *at which either a row or a column will be completely painted in* `mat`.
///
/// **Example 1:**
///
/// ![](image%20explanation%20for%20example%201)![image explanation for example 1](https://assets.leetcode.com/uploads/2023/01/18/grid1.jpg)
///
/// ```
/// Input: arr = [1,3,4,2], mat = [[1,4],[2,3]]
/// Output: 2
/// Explanation: The moves are shown in order, and both the first row and second column of the matrix become fully painted at arr[2].
///
/// ```
///
/// **Example 2:**
///
/// ![image explanation for example 2](https://assets.leetcode.com/uploads/2023/01/18/grid2.jpg)
///
/// ```
/// Input: arr = [2,8,7,4,1,3,5,6,9], mat = [[3,2,5],[1,4,6],[8,7,9]]
/// Output: 3
/// Explanation: The second column becomes fully painted at arr[3].
///
/// ```
///
/// **Constraints:**
///
/// * `m == mat.length`
/// * `n = mat[i].length`
/// * `arr.length == m * n`
/// * `1 <= m, n <= 10<sup>5</sup>`
/// * `1 <= m * n <= 10<sup>5</sup>`
/// * `1 <= arr[i], mat[r][c] <= m * n`
/// * All the integers of `arr` are **unique**.
/// * All the integers of `mat` are **unique**.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/first-completely-painted-row-or-column/
// discuss: https://leetcode.com/problems/first-completely-painted-row-or-column/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let mut arr_map = vec![0; arr.len() + 1];
        for (i, x) in arr.into_iter().enumerate() {
            arr_map[x as usize] = i;
        }

        let height = mat.len();
        let width = mat[0].len();

        let mut row_maxs = vec![0; height];
        let mut col_maxs = vec![0; width];

        for (i, row) in mat.into_iter().enumerate() {
            for (j, idx) in row.into_iter().enumerate() {
                let x = arr_map[idx as usize];

                row_maxs[i] = row_maxs[i].max(x);
                col_maxs[j] = col_maxs[j].max(x);
            }
        }

        row_maxs.into_iter().chain(col_maxs).min().unwrap() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2661() {
        let arr = vec![1,3,4,2];
        let mat = nd_vec![[1,4],[2,3]];
        let expected = 2;
        assert_eq!(Solution::first_complete_index(arr, mat), expected);
        let arr = vec![2,8,7,4,1,3,5,6,9];
        let mat = nd_vec![[3,2,5],[1,4,6],[8,7,9]];
        let expected = 3;
        assert_eq!(Solution::first_complete_index(arr, mat), expected);
    }
}
