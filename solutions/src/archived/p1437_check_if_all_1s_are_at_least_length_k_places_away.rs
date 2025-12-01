///
/// # 1437. Check If All 1's Are at Least Length K Places Away
///
/// Given an binary array `nums` and an integer `k`, return `true` *if all* `1`*'s are at least* `k` *places away from each other, otherwise return* `false`.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2020/04/15/sample_1_1791.png)
///
/// ```
/// Input: nums = [1,0,0,0,1,0,0,1], k = 2
/// Output: true
/// Explanation: Each of the 1s are at least 2 places away from each other.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2020/04/15/sample_2_1791.png)
///
/// ```
/// Input: nums = [1,0,0,1,0,1], k = 2
/// Output: false
/// Explanation: The second 1 and third 1 are only one apart from each other.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `0 <= k <= nums.length`
/// * `nums[i]` is `0` or `1`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-all-1s-are-at-least-length-k-places-away/
// discuss: https://leetcode.com/problems/check-if-all-1s-are-at-least-length-k-places-away/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut distance = nums.len() as i32;

        for num in nums {
            match num {
                1 => {
                    if distance < k {
                        return false;
                    } else {
                        distance = 0;
                    }
                }
                _ => distance += 1,
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
    fn test_1437() {
        let nums = vec![1, 0, 0, 0, 1, 0, 0, 1];
        let k = 2;
        let expected = true;
        assert_eq!(Solution::k_length_apart(nums, k), expected);
        let nums = vec![1, 0, 0, 1, 0, 1];
        let k = 2;
        let expected = false;
        assert_eq!(Solution::k_length_apart(nums, k), expected);
    }
}
