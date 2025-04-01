///
/// # 2560. House Robber IV
///
/// There are several consecutive houses along a street, each of which has some money inside. There is also a robber, who wants to steal money from the homes, but he **refuses to steal from adjacent homes**.
///
/// The **capability** of the robber is the maximum amount of money he steals from one house of all the houses he robbed.
///
/// You are given an integer array `nums` representing how much money is stashed in each house. More formally, the `i<sup>th</sup>` house from the left has `nums[i]` dollars.
///
/// You are also given an integer `k`, representing the **minimum** number of houses the robber will steal from. It is always possible to steal at least `k` houses.
///
/// Return *the **minimum** capability of the robber out of all the possible ways to steal at least* `k` *houses*.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [2,3,5,9], k = 2
/// Output: 5
/// Explanation:
/// There are three ways to rob at least 2 houses:
/// - Rob the houses at indices 0 and 2. Capability is max(nums[0], nums[2]) = 5.
/// - Rob the houses at indices 0 and 3. Capability is max(nums[0], nums[3]) = 9.
/// - Rob the houses at indices 1 and 3. Capability is max(nums[1], nums[3]) = 9.
/// Therefore, we return min(5, 9, 9) = 5.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [2,7,9,3,1], k = 2
/// Output: 2
/// Explanation: There are 7 ways to rob the houses. The way which leads to minimum capability is to rob the house at index 0 and 4. Return max(nums[0], nums[4]) = 2.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>9</sup>`
/// * `1 <= k <= (nums.length + 1)/2`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/house-robber-iv/
// discuss: https://leetcode.com/problems/house-robber-iv/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut right = 1_000_000_001;
        let mut mid = (left + right) / 2;

        fn check(c: i32, nums: &[i32], k: i32) -> bool {
            let mut idx = 0;
            let mut count = 0;

            while idx < nums.len() {
                if nums[idx] <= c {
                    count += 1;
                    idx += 2;
                } else {
                    idx += 1
                }
            }

            count >= k
        }

        while left < right - 1 {
            if check(mid, &nums, k) {
                right = mid;
            } else {
                left = mid;
            }
            mid = (left + right) / 2;
        }

        mid + 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2560() {
        let nums = vec![2, 3, 5, 9];
        let k = 2;
        let expected = 5;
        assert_eq!(Solution::min_capability(nums, k), expected);
        let nums = vec![2, 7, 9, 3, 1];
        let k = 2;
        let expected = 2;
        assert_eq!(Solution::min_capability(nums, k), expected);
    }
}
