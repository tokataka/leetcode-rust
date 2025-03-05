use std::collections::HashMap;

///
/// # 873. Length of Longest Fibonacci Subsequence
///
/// A sequence `x<sub>1</sub>, x<sub>2</sub>, ..., x<sub>n</sub>` is *Fibonacci-like* if:
///
/// * `n >= 3`
/// * `x<sub>i</sub> + x<sub>i+1</sub> == x<sub>i+2</sub>` for all `i + 2 <= n`
///
/// Given a **strictly increasing** array `arr` of positive integers forming a sequence, return *the **length** of the longest Fibonacci-like subsequence of* `arr`. If one does not exist, return `0`.
///
/// A **subsequence** is derived from another sequence `arr` by deleting any number of elements (including none) from `arr`, without changing the order of the remaining elements. For example, `[3, 5, 8]` is a subsequence of `[3, 4, 5, 6, 7, 8]`.
///
/// **Example 1:**
///
/// ```
/// Input: arr = [1,2,3,4,5,6,7,8]
/// Output: 5
/// Explanation: The longest subsequence that is fibonacci-like: [1,2,3,5,8].
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: arr = [1,3,7,11,12,14,18]
/// Output: 3
/// Explanation: The longest subsequence that is fibonacci-like: [1,11,12], [3,11,14] or [7,11,18].
/// ```
///
/// **Constraints:**
///
/// * `3 <= arr.length <= 1000`
/// * `1 <= arr[i] < arr[i + 1] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/length-of-longest-fibonacci-subsequence/
// discuss: https://leetcode.com/problems/length-of-longest-fibonacci-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut used = vec![vec![false; arr.len()]; arr.len()];
        let arr_map = arr
            .iter()
            .enumerate()
            .map(|(i, &x)| (x, i))
            .collect::<HashMap<_, _>>();

        for (i, &a) in arr.iter().enumerate() {
            for (j, &b) in arr.iter().enumerate().skip(i + 1) {
                if used[i][j] {
                    continue;
                }

                used[i][j] = true;

                if !arr_map.contains_key(&(a + b)) {
                    continue;
                }

                let mut cur_result = 2;

                let (mut cur, mut next) = (b, a + b);
                let mut cur_i = j;

                while let Some(&next_i) = arr_map.get(&next) {
                    cur_result += 1;
                    used[cur_i][next_i] = true;

                    (cur, next) = (next, cur + next);
                    cur_i = next_i;
                }

                result = result.max(cur_result);
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
    fn test_873() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let expected = 5;
        assert_eq!(Solution::len_longest_fib_subseq(arr), expected);
        let arr = vec![1, 3, 7, 11, 12, 14, 18];
        let expected = 3;
        assert_eq!(Solution::len_longest_fib_subseq(arr), expected);
    }
}
