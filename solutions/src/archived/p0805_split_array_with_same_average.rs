///
/// # 805. Split Array With Same Average
///
/// You are given an integer array `nums`.
///
/// You should move each element of `nums` into one of the two arrays `A` and `B` such that `A` and `B` are non-empty, and `average(A) == average(B)`.
///
/// Return `true` if it is possible to achieve that and `false` otherwise.
///
/// **Note** that for an array `arr`, `average(arr)` is the total_sum of all the elements of `arr` over the length of `arr`.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,2,3,4,5,6,7,8]
/// Output: true
/// Explanation: We can split the array into [1,4,5,8] and [2,3,6,7], and both of them have an average of 4.5.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [3,1]
/// Output: false
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 30`
/// * `0 <= nums[i] <= 10<sup>4</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/split-array-with-same-average/
// discuss: https://leetcode.com/problems/split-array-with-same-average/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        let nums = nums.into_iter().map(|x| x as usize).collect::<Vec<_>>();
        let sum = nums.iter().sum::<usize>();
        let n = nums.len();

        let mut dp = vec![0; sum + 1];
        dp[nums[0]] = 0b10;

        for i in 1..n {
            for s in (0..sum - nums[i]).rev() {
                if dp[s] > 0 {
                    dp[s + nums[i]] |= dp[s] << 1;
                }
            }

            dp[nums[i]] |= 0b10;
        }

        for i in 1..n {
            if sum * i % n == 0 && ((1 << i) & dp[sum * i / n]) > 0 {
                return true;
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_805() {
        // let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
        // let expected = true;
        // assert_eq!(Solution::split_array_same_average(nums), expected);
        // let nums = vec![3, 1];
        // let expected = false;
        // assert_eq!(Solution::split_array_same_average(nums), expected);
        let nums = vec![5, 3, 11, 19, 2];
        let expected = true;
        assert_eq!(Solution::split_array_same_average(nums), expected);
    }
}
