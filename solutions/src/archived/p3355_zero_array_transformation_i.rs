///
/// # 3355. Zero Array Transformation I
///
/// You are given an integer array `nums` of length `n` and a 2D array `queries`, where `queries[i] = [l<sub>i</sub>, r<sub>i</sub>]`.
///
/// For each `queries[i]`:
///
/// * Select a subset of indices within the range `[l<sub>i</sub>, r<sub>i</sub>]` in `nums`.
/// * Decrement the values at the selected indices by 1.
///
/// A **Zero Array** is an array where all elements are equal to 0.
///
/// Return `true` if it is *possible* to transform `nums` into a **Zero Array** after processing all the queries sequentially, otherwise return `false`.
///
/// **Example 1:**
///
/// **Input:** nums = [1,0,1], queries = [[0,2]]
///
/// **Output:** true
///
/// **Explanation:**
///
/// * **For i = 0:**
///   * Select the subset of indices as `[0, 2]` and decrement the values at these indices by 1.
///   * The array will become `[0, 0, 0]`, which is a Zero Array.
///
/// **Example 2:**
///
/// **Input:** nums = [4,3,2,1], queries = [[1,3],[0,2]]
///
/// **Output:** false
///
/// **Explanation:**
///
/// * **For i = 0:**
///   * Select the subset of indices as `[1, 2, 3]` and decrement the values at these indices by 1.
///   * The array will become `[4, 2, 1, 0]`.
///
/// * **For i = 1:**
///   * Select the subset of indices as `[0, 1, 2]` and decrement the values at these indices by 1.
///   * The array will become `[3, 1, 0, 0]`, which is not a Zero Array.
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `0 <= nums[i] <= 10<sup>5</sup>`
/// * `1 <= queries.length <= 10<sup>5</sup>`
/// * `queries[i].length == 2`
/// * `0 <= l<sub>i</sub> <= r<sub>i</sub> < nums.length`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/zero-array-transformation-i/
// discuss: https://leetcode.com/problems/zero-array-transformation-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut diffs = vec![0; nums.len() + 1];

        for query in queries {
            diffs[query[0] as usize] -= 1;
            diffs[query[1] as usize + 1] += 1;
        }

        let mut decrement = 0;

        for (num, diff) in nums.into_iter().zip(diffs) {
            decrement += diff;

            if num + decrement > 0 {
                return false;
            }
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3355() {
        let nums = vec![1, 0, 1];
        let queries = nd_vec![[0, 2]];
        let expected = true;
        assert_eq!(Solution::is_zero_array(nums, queries), expected);
        let nums = vec![4, 3, 2, 1];
        let queries = nd_vec![[1, 3], [0, 2]];
        let expected = false;
        assert_eq!(Solution::is_zero_array(nums, queries), expected);
    }
}
