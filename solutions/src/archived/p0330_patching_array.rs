///
/// # 330. Patching Array
///
/// Given a sorted integer array `nums` and an integer `n`, add/patch elements to the array such that any number in the range `[1, n]` inclusive can be formed by the sum of some elements in the array.
///
/// Return *the minimum number of patches required*.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,3], n = 6
/// Output: 1
/// Explanation:
/// Combinations of nums are [1], [3], [1,3], which form possible sums of: 1, 3, 4.
/// Now if we add/patch 2 to nums, the combinations are: [1], [2], [3], [1,3], [2,3], [1,2,3].
/// Possible sums are 1, 2, 3, 4, 5, 6, which now covers the range [1, 6].
/// So we only need 1 patch.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,5,10], n = 20
/// Output: 2
/// Explanation: The two patches can be [2, 4].
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = [1,2,2], n = 5
/// Output: 0
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 1000`
/// * `1 <= nums[i] <= 10<sup>4</sup>`
/// * `nums` is sorted in **ascending order**.
/// * `1 <= n <= 2<sup>31</sup> - 1`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/patching-array/
// discuss: https://leetcode.com/problems/patching-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let n = n as i64;

        let mut result = 0;

        let mut sum: i64 = 0;

        for num in nums.into_iter().map(|x| x as i64) {
            if n <= sum {
                return result;
            }

            while num > sum + 1 {
                result += 1;
                sum = (sum << 1) + 1;

                if n <= sum {
                    return result;
                }
            }

            sum += num;
        }

        while n > sum {
            result += 1;
            sum = (sum << 1) + 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_330() {
        // let nums = vec![1, 3];
        // let n = 6;
        // let expected = 1;
        // assert_eq!(Solution::min_patches(nums, n), expected);
        // let nums = vec![1, 5, 10];
        // let n = 20;
        // let expected = 2;
        // assert_eq!(Solution::min_patches(nums, n), expected);
        // let nums = vec![1, 2, 2];
        // let n = 5;
        // let expected = 0;
        // assert_eq!(Solution::min_patches(nums, n), expected);
        let nums = vec![2, 9, 22, 28, 31, 38, 44, 44, 47, 52, 56, 61, 71, 77];
        let n = 42;
        let expected = 3;
        assert_eq!(Solution::min_patches(nums, n), expected);
    }
}
