///
/// # 2411. Smallest Subarrays With Maximum Bitwise OR
///
/// You are given a **0-indexed** array `nums` of length `n`, consisting of non-negative integers. For each index `i` from `0` to `n - 1`, you must determine the size of the **minimum sized** non-empty subarray of `nums` starting at `i` (**inclusive**) that has the **maximum** possible **bitwise OR**.
///
/// * In other words, let `B<sub>ij</sub>` be the bitwise OR of the subarray `nums[i...j]`. You need to find the smallest subarray starting at `i`, such that bitwise OR of this subarray is equal to `max(B<sub>ik</sub>)` where `i <= k <= n - 1`.
///
/// The bitwise OR of an array is the bitwise OR of all the numbers in it.
///
/// Return *an integer array* `answer` *of size* `n` *where* `answer[i]` *is the length of the **minimum** sized subarray starting at* `i` *with **maximum** bitwise OR.*
///
/// A **subarray** is a contiguous non-empty sequence of elements within an array.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,0,2,1,3]
/// Output: [3,3,2,2,1]
/// Explanation:
/// The maximum possible bitwise OR starting at any index is 3.
/// - Starting at index 0, the shortest subarray that yields it is [1,0,2].
/// - Starting at index 1, the shortest subarray that yields the maximum bitwise OR is [0,2,1].
/// - Starting at index 2, the shortest subarray that yields the maximum bitwise OR is [2,1].
/// - Starting at index 3, the shortest subarray that yields the maximum bitwise OR is [1,3].
/// - Starting at index 4, the shortest subarray that yields the maximum bitwise OR is [3].
/// Therefore, we return [3,3,2,2,1].
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,2]
/// Output: [2,1]
/// Explanation:
/// Starting at index 0, the shortest subarray that yields the maximum bitwise OR is of length 2.
/// Starting at index 1, the shortest subarray that yields the maximum bitwise OR is of length 1.
/// Therefore, we return [2,1].
///
/// ```
///
/// **Constraints:**
///
/// * `n == nums.length`
/// * `1 <= n <= 10<sup>5</sup>`
/// * `0 <= nums[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-subarrays-with-maximum-bitwise-or/
// discuss: https://leetcode.com/problems/smallest-subarrays-with-maximum-bitwise-or/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        let mut max_ors = vec![];
        let mut cur = 0;

        for &num in nums.iter().rev() {
            cur |= num;
            max_ors.push(cur);
        }

        max_ors.reverse();

        let mut result = vec![];

        let mut cur = 0;
        let mut cur_count = [0; 31];
        let mut right = 0;

        for left in 0..n {
            while left == right || cur != max_ors[left] {
                let mut x = nums[right];

                for i in 0.. {
                    if x == 0 {
                        break;
                    }

                    if x & 1 == 1 {
                        if cur_count[i] == 0 {
                            cur ^= 1 << i;
                        }

                        cur_count[i] += 1;
                    }

                    x >>= 1;
                }

                right += 1;
            }

            result.push((right - left) as i32);

            let mut x = nums[left];

            for i in 0.. {
                if x == 0 {
                    break;
                }

                if x & 1 == 1 {
                    cur_count[i] -= 1;

                    if cur_count[i] == 0 {
                        cur ^= 1 << i;
                    }
                }

                x >>= 1;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2411() {
        let nums = vec![1, 0, 2, 1, 3];
        let expected = vec![3, 3, 2, 2, 1];
        assert_eq!(Solution::smallest_subarrays(nums), expected);
        let nums = vec![1, 2];
        let expected = vec![2, 1];
        assert_eq!(Solution::smallest_subarrays(nums), expected);
    }
}
