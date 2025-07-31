///
/// # 898. Bitwise ORs of Subarrays
///
/// Given an integer array `arr`, return *the number of distinct bitwise ORs of all the non-empty subarrays of* `arr`.
///
/// The bitwise OR of a subarray is the bitwise OR of each integer in the subarray. The bitwise OR of a subarray of one integer is that integer.
///
/// A **subarray** is a contiguous non-empty sequence of elements within an array.
///
/// **Example 1:**
///
/// ```
/// Input: arr = [0]
/// Output: 1
/// Explanation: There is only one possible result: 0.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: arr = [1,1,2]
/// Output: 3
/// Explanation: The possible subarrays are [1], [1], [2], [1, 1], [1, 2], [1, 1, 2].
/// These yield the results 1, 1, 2, 1, 3, 3.
/// There are 3 unique values, so the answer is 3.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: arr = [1,2,4]
/// Output: 6
/// Explanation: The possible results are 1, 2, 3, 4, 6, and 7.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= arr.length <= 5 * 10<sup>4</sup>`
/// * `0 <= arr[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/bitwise-ors-of-subarrays/
// discuss: https://leetcode.com/problems/bitwise-ors-of-subarrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashSet;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let mut result = HashSet::new();
        let mut cur = Vec::with_capacity(32);

        for x in arr {
            for v in cur.iter_mut() {
                *v |= x;
                result.insert(*v);
            }

            result.insert(x);

            cur.push(x);
            cur.dedup();
        }

        result.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_898() {
        let arr = vec![0];
        let expected = 1;
        assert_eq!(Solution::subarray_bitwise_o_rs(arr), expected);
        let arr = vec![1, 1, 2];
        let expected = 3;
        assert_eq!(Solution::subarray_bitwise_o_rs(arr), expected);
        let arr = vec![1, 2, 4];
        let expected = 6;
        assert_eq!(Solution::subarray_bitwise_o_rs(arr), expected);
    }
}
