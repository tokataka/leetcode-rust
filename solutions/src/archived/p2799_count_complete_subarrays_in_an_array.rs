///
/// # 2799. Count Complete Subarrays in an Array
///
/// You are given an array `nums` consisting of **positive** integers.
///
/// We call a subarray of an array **complete** if the following condition is satisfied:
///
/// * The number of **distinct** elements in the subarray is equal to the number of distinct elements in the whole array.
///
/// Return *the number of **complete** subarrays*.
///
/// A **subarray** is a contiguous non-empty part of an array.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,3,1,2,2]
/// Output: 4
/// Explanation: The complete subarrays are the following: [1,3,1,2], [1,3,1,2,2], [3,1,2] and [3,1,2,2].
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [5,5,5,5]
/// Output: 10
/// Explanation: The array consists only of the integer 5, so any subarray is complete. The number of subarrays that we can choose is 10.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 1000`
/// * `1 <= nums[i] <= 2000`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-complete-subarrays-in-an-array/
// discuss: https://leetcode.com/problems/count-complete-subarrays-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let complete_idx = nums
            .iter()
            .enumerate()
            .fold(
                (0, vec![false; 2001]),
                |(mut complete_idx, mut found), (i, &x)| {
                    if !found[x as usize] {
                        found[x as usize] = true;
                        complete_idx = i;
                    }

                    (complete_idx, found)
                },
            )
            .0;

        let mut counts = vec![0; 2001];
        let mut left = 0;
        let mut result = 0;

        for right in 0..complete_idx {
            counts[nums[right] as usize] += 1;
        }

        for right in complete_idx..nums.len() {
            counts[nums[right] as usize] += 1;

            while counts[nums[left] as usize] > 1 {
                counts[nums[left] as usize] -= 1;
                left += 1;
            }

            result += left + 1;
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2799() {
        let nums = vec![1, 3, 1, 2, 2];
        let expected = 4;
        assert_eq!(Solution::count_complete_subarrays(nums), expected);
        let nums = vec![5, 5, 5, 5];
        let expected = 10;
        assert_eq!(Solution::count_complete_subarrays(nums), expected);
    }
}
