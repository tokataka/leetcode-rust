///
/// # 594. Longest Harmonious Subsequence
///
/// We define a harmonious array as an array where the difference between its maximum value and its minimum value is **exactly** `1`.
///
/// Given an integer array `nums`, return the length of its longest harmonious subsequence among all its possible subsequences.
///
/// **Example 1:**
///
/// **Input:** nums = [1,3,2,2,5,2,3,7]
///
/// **Output:** 5
///
/// **Explanation:**
///
/// The longest harmonious subsequence is `[3,2,2,2,3]`.
///
/// **Example 2:**
///
/// **Input:** nums = [1,2,3,4]
///
/// **Output:** 2
///
/// **Explanation:**
///
/// The longest harmonious subsequences are `[1,2]`, `[2,3]`, and `[3,4]`, all of which have a length of 2.
///
/// **Example 3:**
///
/// **Input:** nums = [1,1,1,1]
///
/// **Output:** 0
///
/// **Explanation:**
///
/// No harmonic subsequence exists.
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 2 * 10<sup>4</sup>`
/// * `-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-harmonious-subsequence/
// discuss: https://leetcode.com/problems/longest-harmonious-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            *map.entry(num).or_default() += 1;
        }

        let mut result = 0;

        for (&num, &count) in map.iter() {
            if let Some(count2) = map.get(&(num + 1)) {
                result = result.max(count + count2);
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
    fn test_594() {
        let nums = vec![1, 3, 2, 2, 5, 2, 3, 7];
        let expected = 5;
        assert_eq!(Solution::find_lhs(nums), expected);
        let nums = vec![1, 2, 3, 4];
        let expected = 2;
        assert_eq!(Solution::find_lhs(nums), expected);
        let nums = vec![1, 1, 1, 1];
        let expected = 0;
        assert_eq!(Solution::find_lhs(nums), expected);
    }
}
