///
/// # 3542. Minimum Operations to Convert All Elements to Zero
///
/// You are given an array `nums` of size `n`, consisting of **non-negative** integers. Your task is to apply some (possibly zero) operations on the array so that **all** elements become 0.
///
/// In one operation, you can select a subarray `[i, j]` (where `0 <= i <= j < n`) and set all occurrences of the **minimum** **non-negative** integer in that subarray to 0.
///
/// Return the **minimum** number of operations required to make all elements in the array 0.
///
/// **Example 1:**
///
/// **Input:** nums = [0,2]
///
/// **Output:** 1
///
/// **Explanation:**
///
/// * Select the subarray `[1,1]` (which is `[2]`), where the minimum non-negative integer is 2. Setting all occurrences of 2 to 0 results in `[0,0]`.
/// * Thus, the minimum number of operations required is 1.
///
/// **Example 2:**
///
/// **Input:** nums = [3,1,2,1]
///
/// **Output:** 3
///
/// **Explanation:**
///
/// * Select subarray `[1,3]` (which is `[1,2,1]`), where the minimum non-negative integer is 1. Setting all occurrences of 1 to 0 results in `[3,0,2,0]`.
/// * Select subarray `[2,2]` (which is `[2]`), where the minimum non-negative integer is 2. Setting all occurrences of 2 to 0 results in `[3,0,0,0]`.
/// * Select subarray `[0,0]` (which is `[3]`), where the minimum non-negative integer is 3. Setting all occurrences of 3 to 0 results in `[0,0,0,0]`.
/// * Thus, the minimum number of operations required is 3.
///
/// **Example 3:**
///
/// **Input:** nums = [1,2,1,2,1,2]
///
/// **Output:** 4
///
/// **Explanation:**
///
/// * Select subarray `[0,5]` (which is `[1,2,1,2,1,2]`), where the minimum non-negative integer is 1. Setting all occurrences of 1 to 0 results in `[0,2,0,2,0,2]`.
/// * Select subarray `[1,1]` (which is `[2]`), where the minimum non-negative integer is 2. Setting all occurrences of 2 to 0 results in `[0,0,0,2,0,2]`.
/// * Select subarray `[3,3]` (which is `[2]`), where the minimum non-negative integer is 2. Setting all occurrences of 2 to 0 results in `[0,0,0,0,0,2]`.
/// * Select subarray `[5,5]` (which is `[2]`), where the minimum non-negative integer is 2. Setting all occurrences of 2 to 0 results in `[0,0,0,0,0,0]`.
/// * Thus, the minimum number of operations required is 4.
///
/// **Constraints:**
///
/// * `1 <= n == nums.length <= 10<sup>5</sup>`
/// * `0 <= nums[i] <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-operations-to-convert-all-elements-to-zero/
// discuss: https://leetcode.com/problems/minimum-operations-to-convert-all-elements-to-zero/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut monotonic = vec![];

        for num in nums {
            while let Some(&x) = monotonic.last() {
                if x < num {
                    break;
                }

                if x > num {
                    result += 1;
                }

                monotonic.pop();
            }

            if num > 0 {
                monotonic.push(num);
            }
        }

        result + monotonic.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3542() {
        let nums = vec![0, 2];
        let expected = 1;
        assert_eq!(Solution::min_operations(nums), expected);
        let nums = vec![3, 1, 2, 1];
        let expected = 3;
        assert_eq!(Solution::min_operations(nums), expected);
        let nums = vec![1, 2, 1, 2, 1, 2];
        let expected = 4;
        assert_eq!(Solution::min_operations(nums), expected);
    }
}
