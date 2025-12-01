///
/// # 3741. Minimum Distance Between Three Equal Elements II
///
/// You are given an integer array `nums`.
///
/// A tuple `(i, j, k)` of 3 **distinct** indices is **good** if `nums[i] == nums[j] == nums[k]`.
///
/// The **distance** of a **good** tuple is `abs(i - j) + abs(j - k) + abs(k - i)`, where `abs(x)` denotes the **absolute value** of `x`.
///
/// Return an integer denoting the **minimum** possible **distance** of a **good** tuple. If no **good** tuples exist, return `-1`.
///
/// **Example 1:**
///
/// **Input:** nums = [1,2,1,1,3]
///
/// **Output:** 6
///
/// **Explanation:**
///
/// The minimum distance is achieved by the good tuple `(0, 2, 3)`.
///
/// `(0, 2, 3)` is a good tuple because `nums[0] == nums[2] == nums[3] == 1`. Its distance is `abs(0 - 2) + abs(2 - 3) + abs(3 - 0) = 2 + 1 + 3 = 6`.
///
/// **Example 2:**
///
/// **Input:** nums = [1,1,2,3,2,1,2]
///
/// **Output:** 8
///
/// **Explanation:**
///
/// The minimum distance is achieved by the good tuple `(2, 4, 6)`.
///
/// `(2, 4, 6)` is a good tuple because `nums[2] == nums[4] == nums[6] == 2`. Its distance is `abs(2 - 4) + abs(4 - 6) + abs(6 - 2) = 2 + 2 + 4 = 8`.
///
/// **Example 3:**
///
/// **Input:** nums = [1]
///
/// **Output:** -1
///
/// **Explanation:**
///
/// There are no good tuples. Therefore, the answer is -1.
///
/// **Constraints:**
///
/// * `1 <= n == nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= n`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-distance-between-three-equal-elements-ii/
// discuss: https://leetcode.com/problems/minimum-distance-between-three-equal-elements-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut result = i32::MAX;
        let mut qs = vec![[usize::MAX; 2]; nums.len() + 1];

        for (i, num) in nums.into_iter().enumerate() {
            let q = &mut qs[num as usize];

            if q[0] != usize::MAX {
                result = result.min(2 * (i - q[0]) as i32);
            }

            (q[0], q[1]) = (q[1], i);
        }

        match result {
            i32::MAX => -1,
            x => x,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3741() {
        let nums = vec![1, 2, 1, 1, 3];
        let expected = 6;
        assert_eq!(Solution::minimum_distance(nums), expected);
        let nums = vec![1, 1, 2, 3, 2, 1, 2];
        let expected = 8;
        assert_eq!(Solution::minimum_distance(nums), expected);
        let nums = vec![1];
        let expected = -1;
        assert_eq!(Solution::minimum_distance(nums), expected);
    }
}
