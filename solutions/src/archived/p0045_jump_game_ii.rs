///
/// # 45. Jump Game II
///
/// You are given a **0-indexed** array of integers `nums` of length `n`. You are initially positioned at `nums[0]`.
///
/// Each element `nums[i]` represents the maximum length of a forward jump from index `i`. In other words, if you are at `nums[i]`, you can jump to any `nums[i + j]` where:
///
/// * `0 <= j <= nums[i]` and
/// * `i + j < n`
///
/// Return *the minimum number of jumps to reach* `nums[n - 1]`. The test cases are generated such that you can reach `nums[n - 1]`.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [2,3,1,1,4]
/// Output: 2
/// Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [2,3,0,1,4]
/// Output: 2
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>4</sup>`
/// * `0 <= nums[i] <= 1000`
/// * It's guaranteed that you can reach `nums[n - 1]`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/jump-game-ii/
// discuss: https://leetcode.com/problems/jump-game-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut cache = vec![i32::MAX; nums.len()];
        cache[nums.len() - 1] = 0;

        for i in (0..nums.len() - 1).rev() {
            let j = nums[i] as usize;

            cache[i] = cache
                .iter()
                .skip(i + 1)
                .take(j)
                .min()
                .unwrap_or(&i32::MAX)
                .saturating_add(1);
        }

        cache[0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_45() {
        let nums = vec![2, 3, 1, 1, 4];
        let expected = 2;
        assert_eq!(Solution::jump(nums), expected);
        let nums = vec![2, 3, 0, 1, 4];
        let expected = 2;
        assert_eq!(Solution::jump(nums), expected);
    }
}
