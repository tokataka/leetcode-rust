use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

///
/// # 3489. Zero Array Transformation IV
///
/// You are given an integer array `nums` of length `n` and a 2D array `queries`, where `queries[i] = [l<sub>i</sub>, r<sub>i</sub>, val<sub>i</sub>]`.
///
/// Each `queries[i]` represents the following action on `nums`:
///
/// * Select a subset of indices in the range `[l<sub>i</sub>, r<sub>i</sub>]` from `nums`.
/// * Decrement the value at each selected index by **exactly** `val<sub>i</sub>`.
///
/// A **Zero Array** is an array with all its elements equal to 0.
///
/// Return the **minimum** possible **non-negative** value of `k`, such that after processing the first `k` queries in **sequence**, `nums` becomes a **Zero Array**. If no such `k` exists, return -1.
///
/// **Example 1:**
///
/// **Input:** nums = [2,0,2], queries = [[0,2,1],[0,2,1],[1,1,3]]
///
/// **Output:** 2
///
/// **Explanation:**
///
/// * **For query 0 (l = 0, r = 2, val = 1):**
///   * Decrement the values at indices `[0, 2]` by 1.
///   * The array will become `[1, 0, 1]`.
///
/// * **For query 1 (l = 0, r = 2, val = 1):**
///   * Decrement the values at indices `[0, 2]` by 1.
///   * The array will become `[0, 0, 0]`, which is a Zero Array. Therefore, the minimum value of `k` is 2.
///
/// **Example 2:**
///
/// **Input:** nums = [4,3,2,1], queries = [[1,3,2],[0,2,1]]
///
/// **Output:** -1
///
/// **Explanation:**
///
/// It is impossible to make nums a Zero Array even after all the queries.
///
/// **Example 3:**
///
/// **Input:** nums = [1,2,3,2,1], queries = [[0,1,1],[1,2,1],[2,3,2],[3,4,1],[4,4,1]]
///
/// **Output:** 4
///
/// **Explanation:**
///
/// * **For query 0 (l = 0, r = 1, val = 1):**
///   * Decrement the values at indices `[0, 1]` by `1`.
///   * The array will become `[0, 1, 3, 2, 1]`.
///
/// * **For query 1 (l = 1, r = 2, val = 1):**
///   * Decrement the values at indices `[1, 2]` by 1.
///   * The array will become `[0, 0, 2, 2, 1]`.
///
/// * **For query 2 (l = 2, r = 3, val = 2):**
///   * Decrement the values at indices `[2, 3]` by 2.
///   * The array will become `[0, 0, 0, 0, 1]`.
///
/// * **For query 3 (l = 3, r = 4, val = 1):**
///   * Decrement the value at index 4 by 1.
///   * The array will become `[0, 0, 0, 0, 0]`. Therefore, the minimum value of `k` is 4.
///
/// **Example 4:**
///
/// **Input:** nums = [1,2,3,2,6], queries = [[0,1,1],[0,2,1],[1,4,2],[4,4,4],[3,4,1],[4,4,5]]
///
/// **Output:** 4
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10`
/// * `0 <= nums[i] <= 1000`
/// * `1 <= queries.length <= 1000`
/// * `queries[i] = [l<sub>i</sub>, r<sub>i</sub>, val<sub>i</sub>]`
/// * `0 <= l<sub>i</sub> <= r<sub>i</sub> < nums.length`
/// * `1 <= val<sub>i</sub> <= 10`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/zero-array-transformation-iv/
// discuss: https://leetcode.com/problems/zero-array-transformation-iv/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut diffs: BinaryHeap<(Reverse<usize>, i32)> = BinaryHeap::new();
        let mut val_count: HashMap<i32, i32> = HashMap::new();
        let mut query_idx = 0;

        for (idx, num) in nums.into_iter().enumerate() {
            while let Some(&(Reverse(diff_idx), val)) = diffs.peek() {
                if diff_idx > idx {
                    break;
                }

                diffs.pop();

                if val > 0 {
                    *val_count.entry(val).or_default() += 1;
                } else {
                    let v = val_count.get_mut(&(-val)).unwrap();
                    if *v > 1 {
                        *v -= 1;
                    } else {
                        val_count.remove(&(-val));
                    }
                }
            }

            let mut valid_nums = vec![false; 1001];
            valid_nums[0] = true;

            for (&val, &count) in &val_count {
                for _ in 0..count {
                    for i in (0..1000).rev() {
                        if valid_nums[i] && i + val as usize <= 1000 {
                            valid_nums[i + val as usize] = true;
                        }
                    }
                }
            }

            while !valid_nums[num as usize] {
                if query_idx == queries.len() {
                    return -1;
                }

                let (l, r, val) = (
                    queries[query_idx][0] as usize,
                    queries[query_idx][1] as usize + 1,
                    queries[query_idx][2],
                );

                if r > idx {
                    if l > idx {
                        diffs.push((Reverse(l), val));
                    } else {
                        *val_count.entry(val).or_default() += 1;

                        for i in (0..1000).rev() {
                            if valid_nums[i] && i + val as usize <= 1000 {
                                valid_nums[i + val as usize] = true;
                            }
                        }
                    }

                    diffs.push((Reverse(r), -val));
                }

                query_idx += 1;
            }
        }

        query_idx as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3489() {
        let nums = vec![2, 0, 2];
        let queries = nd_vec![[0, 2, 1], [0, 2, 1], [1, 1, 3]];
        let expected = 2;
        assert_eq!(Solution::min_zero_array(nums, queries), expected);
        let nums = vec![4, 3, 2, 1];
        let queries = nd_vec![[1, 3, 2], [0, 2, 1]];
        let expected = -1;
        assert_eq!(Solution::min_zero_array(nums, queries), expected);
        let nums = vec![1, 2, 3, 2, 1];
        let queries = nd_vec![[0, 1, 1], [1, 2, 1], [2, 3, 2], [3, 4, 1], [4, 4, 1]];
        let expected = 4;
        assert_eq!(Solution::min_zero_array(nums, queries), expected);
        let nums = vec![1, 2, 3, 2, 6];
        let queries = nd_vec![
            [0, 1, 1],
            [0, 2, 1],
            [1, 4, 2],
            [4, 4, 4],
            [3, 4, 1],
            [4, 4, 5]
        ];
        let expected = 4;
        assert_eq!(Solution::min_zero_array(nums, queries), expected);
    }
}
