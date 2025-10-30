///
/// # 3005. Count Elements With Maximum Frequency
///
/// You are given an array `nums` consisting of **positive** integers.
///
/// Return *the **total frequencies** of elements in* `nums` *such that those elements all have the **maximum** frequency*.
///
/// The **frequency** of an element is the number of occurrences of that element in the array.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,2,2,3,1,4]
/// Output: 4
/// Explanation: The elements 1 and 2 have a frequency of 2 which is the maximum frequency in the array.
/// So the number of elements in the array with maximum frequency is 4.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,2,3,4,5]
/// Output: 5
/// Explanation: All elements of the array have a frequency of 1 which is the maximum.
/// So the number of elements in the array with maximum frequency is 5.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 100`
/// * `1 <= nums[i] <= 100`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-elements-with-maximum-frequency/
// discuss: https://leetcode.com/problems/count-elements-with-maximum-frequency/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut freq = [0; 101];

        let mut max = 0;
        let mut count = 0;

        for num in nums {
            freq[num as usize] += 1;

            if freq[num as usize] > max {
                max = freq[num as usize];
                count = 0;
            }

            if freq[num as usize] == max {
                count += 1;
            }
        }

        max * count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3005() {
        let nums = vec![1, 2, 2, 3, 1, 4];
        let expected = 4;
        assert_eq!(Solution::max_frequency_elements(nums), expected);
        let nums = vec![1, 2, 3, 4, 5];
        let expected = 5;
        assert_eq!(Solution::max_frequency_elements(nums), expected);
    }
}
