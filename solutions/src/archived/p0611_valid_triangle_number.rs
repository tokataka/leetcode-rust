///
/// # 611. Valid Triangle Number
///
/// Given an integer array `nums`, return *the number of triplets chosen from the array that can make triangles if we take them as side lengths of a triangle*.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [2,2,3,4]
/// Output: 3
/// Explanation: Valid combinations are:
/// 2,3,4 (using the first 2)
/// 2,3,4 (using the second 2)
/// 2,2,3
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [4,2,3,4]
/// Output: 4
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 1000`
/// * `0 <= nums[i] <= 1000`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-triangle-number/
// discuss: https://leetcode.com/problems/valid-triangle-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut result = 0;

        let mut prev_nums = vec![0; 1001];
        let mut prev_sums = vec![0; 2001];

        for num in nums.into_iter().skip_while(|&x| x == 0) {
            let num = num as usize;

            for sum in num + 1..=2 * num {
                result += prev_sums[sum];
            }

            for prev in 1..=num {
                prev_sums[num + prev] += prev_nums[prev]
            }

            prev_nums[num] += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_611() {
        let nums = vec![2, 2, 3, 4];
        let expected = 3;
        assert_eq!(Solution::triangle_number(nums), expected);
        let nums = vec![4, 2, 3, 4];
        let expected = 4;
        assert_eq!(Solution::triangle_number(nums), expected);
    }
}
