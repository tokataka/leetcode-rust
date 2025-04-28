///
/// # 3392. Count Subarrays of Length Three With a Condition
///
/// Given an integer array `nums`, return the number of subarrays of length 3 such that the sum of the first and third numbers equals *exactly* half of the second number.
///
/// **Example 1:**
///
/// **Input:** nums = [1,2,1,4,1]
///
/// **Output:** 1
///
/// **Explanation:**
///
/// Only the subarray `[1,4,1]` contains exactly 3 elements where the sum of the first and third numbers equals half the middle number.
///
/// **Example 2:**
///
/// **Input:** nums = [1,1,1]
///
/// **Output:** 0
///
/// **Explanation:**
///
/// `[1,1,1]` is the only subarray of length 3. However, its first and third numbers do not add to half the middle number.
///
/// **Constraints:**
///
/// * `3 <= nums.length <= 100`
/// * `-100 <= nums[i] <= 100`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-subarrays-of-length-three-with-a-condition/
// discuss: https://leetcode.com/problems/count-subarrays-of-length-three-with-a-condition/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        nums.windows(3)
            .filter(|x| (x[0] + x[2]) * 2 == x[1])
            .count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3392() {
        let nums = vec![1, 2, 1, 4, 1];
        let expected = 1;
        assert_eq!(Solution::count_subarrays(nums), expected);
        let nums = vec![1, 1, 1];
        let expected = 0;
        assert_eq!(Solution::count_subarrays(nums), expected);
    }
}
