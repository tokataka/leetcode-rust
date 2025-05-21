use std::{cmp::Reverse, collections::BinaryHeap};

///
/// # 3362. Zero Array Transformation III
///
/// You are given an integer array `nums` of length `n` and a 2D array `queries` where `queries[i] = [l<sub>i</sub>, r<sub>i</sub>]`.
///
/// Each `queries[i]` represents the following action on `nums`:
///
/// * Decrement the value at each index in the range `[l<sub>i</sub>, r<sub>i</sub>]` in `nums` by **at most** 1.
/// * The amount by which the value is decremented can be chosen **independently** for each index.
///
/// A **Zero Array** is an array with all its elements equal to 0.
///
/// Return the **maximum** number of elements that can be removed from `queries`, such that `nums` can still be converted to a **zero array** using the *remaining* queries. If it is not possible to convert `nums` to a **zero array**, return -1.
///
/// **Example 1:**
///
/// **Input:** nums = [2,0,2], queries = [[0,2],[0,2],[1,1]]
///
/// **Output:** 1
///
/// **Explanation:**
///
/// After removing `queries[2]`, `nums` can still be converted to a zero array.
///
/// * Using `queries[0]`, decrement `nums[0]` and `nums[2]` by 1 and `nums[1]` by 0.
/// * Using `queries[1]`, decrement `nums[0]` and `nums[2]` by 1 and `nums[1]` by 0.
///
/// **Example 2:**
///
/// **Input:** nums = [1,1,1,1], queries = [[1,3],[0,2],[1,3],[1,2]]
///
/// **Output:** 2
///
/// **Explanation:**
///
/// We can remove `queries[2]` and `queries[3]`.
///
/// **Example 3:**
///
/// **Input:** nums = [1,2,3,4], queries = [[0,3]]
///
/// **Output:** -1
///
/// **Explanation:**
///
/// `nums` cannot be converted to a zero array even after using all the queries.
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

// problem: https://leetcode.com/problems/zero-array-transformation-iii/
// discuss: https://leetcode.com/problems/zero-array-transformation-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
        let mut result = queries.len() as i32;

        queries.sort_unstable();

        let mut query_idx = 0;
        let mut candidate_ends = BinaryHeap::new();
        let mut active_ends = BinaryHeap::new();

        let mut active_count = 0;

        for (idx, mut num) in nums.into_iter().enumerate() {
            while let Some(&Reverse(end_idx)) = active_ends.peek() {
                if end_idx > idx {
                    break;
                }
                active_count -= 1;
                active_ends.pop();
            }

            num -= active_count;

            while query_idx < queries.len() && queries[query_idx][0] as usize == idx {
                candidate_ends.push(queries[query_idx][1] as usize + 1);
                query_idx += 1;
            }

            while num > 0 {
                active_ends.push(Reverse(match candidate_ends.pop() {
                    Some(end) if end > idx => end,
                    _ => return -1,
                }));

                num -= 1;
                active_count += 1;
                result -= 1;
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
    fn test_3362() {
        let nums = vec![2, 0, 2];
        let queries = nd_vec![[0, 2], [0, 2], [1, 1]];
        let expected = 1;
        assert_eq!(Solution::max_removal(nums, queries), expected);
        let nums = vec![1, 1, 1, 1];
        let queries = nd_vec![[1, 3], [0, 2], [1, 3], [1, 2]];
        let expected = 2;
        assert_eq!(Solution::max_removal(nums, queries), expected);
        let nums = vec![1, 2, 3, 4];
        let queries = nd_vec![[0, 3]];
        let expected = -1;
        assert_eq!(Solution::max_removal(nums, queries), expected);
    }
}
