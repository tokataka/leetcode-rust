///
/// # 3480. Maximize Subarrays After Removing One Conflicting Pair
///
/// You are given an integer `n` which represents an array `nums` containing the numbers from 1 to `n` in order. Additionally, you are given a 2D array `conflictingPairs`, where `conflictingPairs[i] = [a, b]` indicates that `a` and `b` form a conflicting pair.
///
/// Remove **exactly** one element from `conflictingPairs`. Afterward, count the number of non-empty subarrays of `nums` which do not contain both `a` and `b` for any remaining conflicting pair `[a, b]`.
///
/// Return the **maximum** number of subarrays possible after removing **exactly** one conflicting pair.
///
/// **Example 1:**
///
/// **Input:** n = 4, conflictingPairs = [[2,3],[1,4]]
///
/// **Output:** 9
///
/// **Explanation:**
///
/// * Remove `[2, 3]` from `conflictingPairs`. Now, `conflictingPairs = [[1, 4]]`.
/// * There are 9 subarrays in `nums` where `[1, 4]` do not appear together. They are `[1]`, `[2]`, `[3]`, `[4]`, `[1, 2]`, `[2, 3]`, `[3, 4]`, `[1, 2, 3]` and `[2, 3, 4]`.
/// * The maximum number of subarrays we can achieve after removing one element from `conflictingPairs` is 9.
///
/// **Example 2:**
///
/// **Input:** n = 5, conflictingPairs = [[1,2],[2,5],[3,5]]
///
/// **Output:** 12
///
/// **Explanation:**
///
/// * Remove `[1, 2]` from `conflictingPairs`. Now, `conflictingPairs = [[2, 5], [3, 5]]`.
/// * There are 12 subarrays in `nums` where `[2, 5]` and `[3, 5]` do not appear together.
/// * The maximum number of subarrays we can achieve after removing one element from `conflictingPairs` is 12.
///
/// **Constraints:**
///
/// * `2 <= n <= 10<sup>5</sup>`
/// * `1 <= conflictingPairs.length <= 2 * n`
/// * `conflictingPairs[i].length == 2`
/// * `1 <= conflictingPairs[i][j] <= n`
/// * `conflictingPairs[i][0] != conflictingPairs[i][1]`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximize-subarrays-after-removing-one-conflicting-pair/
// discuss: https://leetcode.com/problems/maximize-subarrays-after-removing-one-conflicting-pair/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;

        let mut b_mins = vec![(i32::MAX, i32::MAX); n + 1];

        for pair in conflicting_pairs {
            let (a, b) = (pair[0], pair[1]);
            let (a, b) = (a.min(b) as usize, a.max(b));

            if b < b_mins[a].0 {
                b_mins[a] = (b, b_mins[a].0);
            } else if b < b_mins[a].1 {
                b_mins[a].1 = b;
            }
        }

        let mut result = 0;

        let mut b1_idx = n;
        let mut b2 = i32::MAX;

        let mut del_count = vec![0; n + 1];

        for i in (1..=n).rev() {
            if b_mins[i].0 < b_mins[b1_idx].0 {
                b2 = b2.min(b_mins[b1_idx].0);
                b1_idx = i;
            } else {
                b2 = b2.min(b_mins[i].0);
            }

            result += (b_mins[b1_idx].0.min((n + 1) as i32) - i as i32) as i64;
            del_count[b1_idx] += (b2.min(b_mins[b1_idx].1).min((n + 1) as i32)
                - b_mins[b1_idx].0.min((n + 1) as i32)) as i64;
        }

        result + del_count.iter().max().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3480() {
        let n = 4;
        let conflicting_pairs = nd_vec![[2, 3], [1, 4]];
        let expected = 9;
        assert_eq!(Solution::max_subarrays(n, conflicting_pairs), expected);
        let n = 5;
        let conflicting_pairs = nd_vec![[1, 2], [2, 5], [3, 5]];
        let expected = 12;
        assert_eq!(Solution::max_subarrays(n, conflicting_pairs), expected);
    }
}
