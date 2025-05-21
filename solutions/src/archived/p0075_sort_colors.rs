///
/// # 75. Sort Colors
///
/// Given an array `nums` with `n` objects colored red, white, or blue, sort them **[in-place](https://en.wikipedia.org/wiki/In-place_algorithm)** so that objects of the same color are adjacent, with the colors in the order red, white, and blue.
///
/// We will use the integers `0`, `1`, and `2` to represent the color red, white, and blue, respectively.
///
/// You must solve this problem without using the library's sort function.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [2,0,2,1,1,0]
/// Output: [0,0,1,1,2,2]
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [2,0,1]
/// Output: [0,1,2]
///
/// ```
///
/// **Constraints:**
///
/// * `n == nums.length`
/// * `1 <= n <= 300`
/// * `nums[i]` is either `0`, `1`, or `2`.
///
/// **Follow up:** Could you come up with a one-pass algorithm using only constant extra space?
///
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-colors/
// discuss: https://leetcode.com/problems/sort-colors/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sort_colors(nums: &mut [i32]) {
        let mut right = nums.len() - 1;

        for pivot in [2, 1] {
            let mut left = 0;

            while left < right {
                while nums[left] < pivot && left < right {
                    left += 1;
                }

                while nums[right] >= pivot && left < right {
                    right -= 1;
                }

                if left >= right {
                    break;
                }

                nums.swap(left, right);
            }
        }

        print!("{}", false as usize);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_75() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        let expected = vec![0, 0, 1, 1, 2, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, expected);
        let mut nums = vec![2, 0, 1];
        let expected = vec![0, 1, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, expected);
    }
}
