///
/// # 3105. Longest Strictly Increasing or Strictly Decreasing Subarray
///
/// You are given an array of integers `nums`. Return *the length of the **longest** subarray of* `nums` *which is either **strictly increasing** or **strictly decreasing***.
///
/// **Example 1:**
///
/// **Input:** nums = [1,4,3,3,2]
///
/// **Output:** 2
///
/// **Explanation:**
///
/// The strictly increasing subarrays of `nums` are `[1]`, `[2]`, `[3]`, `[3]`, `[4]`, and `[1,4]`.
///
/// The strictly decreasing subarrays of `nums` are `[1]`, `[2]`, `[3]`, `[3]`, `[4]`, `[3,2]`, and `[4,3]`.
///
/// Hence, we return `2`.
///
/// **Example 2:**
///
/// **Input:** nums = [3,3,3,3]
///
/// **Output:** 1
///
/// **Explanation:**
///
/// The strictly increasing subarrays of `nums` are `[3]`, `[3]`, `[3]`, and `[3]`.
///
/// The strictly decreasing subarrays of `nums` are `[3]`, `[3]`, `[3]`, and `[3]`.
///
/// Hence, we return `1`.
///
/// **Example 3:**
///
/// **Input:** nums = [3,2,1]
///
/// **Output:** 3
///
/// **Explanation:**
///
/// The strictly increasing subarrays of `nums` are `[3]`, `[2]`, and `[1]`.
///
/// The strictly decreasing subarrays of `nums` are `[3]`, `[2]`, `[1]`, `[3,2]`, `[2,1]`, and `[3,2,1]`.
///
/// Hence, we return `3`.
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 50`
/// * `1 <= nums[i] <= 50`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-strictly-increasing-or-strictly-decreasing-subarray/
// discuss: https://leetcode.com/problems/longest-strictly-increasing-or-strictly-decreasing-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        nums.chunk_by(|a, b| a < b)
            .map(|x| x.len())
            .max()
            .unwrap()
            .max(nums.chunk_by(|a, b| a > b).map(|x| x.len()).max().unwrap()) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3105() {
        let nums = vec![1, 4, 3, 3, 2];
        let expected = 2;
        assert_eq!(Solution::longest_monotonic_subarray(nums), expected);
    }
}
