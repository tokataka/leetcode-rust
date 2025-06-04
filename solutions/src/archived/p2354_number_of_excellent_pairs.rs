///
/// # 2354. Number of Excellent Pairs
///
/// You are given a **0-indexed** positive integer array `nums` and a positive integer `k`.
///
/// A pair of numbers `(num1, num2)` is called **excellent** if the following conditions are satisfied:
///
/// * **Both** the numbers `num1` and `num2` exist in the array `nums`.
/// * The sum of the number of set bits in `num1 OR num2` and `num1 AND num2` is greater than or equal to `k`, where `OR` is the bitwise **OR** operation and `AND` is the bitwise **AND** operation.
///
/// Return *the number of **distinct** excellent pairs*.
///
/// Two pairs `(a, b)` and `(c, d)` are considered distinct if either `a != c` or `b != d`. For example, `(1, 2)` and `(2, 1)` are distinct.
///
/// **Note** that a pair `(num1, num2)` such that `num1 == num2` can also be excellent if you have at least **one** occurrence of `num1` in the array.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,2,3,1], k = 3
/// Output: 5
/// Explanation: The excellent pairs are the following:
/// - (3, 3). (3 AND 3) and (3 OR 3) are both equal to (11) in binary. The total number of set bits is 2 + 2 = 4, which is greater than or equal to k = 3.
/// - (2, 3) and (3, 2). (2 AND 3) is equal to (10) in binary, and (2 OR 3) is equal to (11) in binary. The total number of set bits is 1 + 2 = 3.
/// - (1, 3) and (3, 1). (1 AND 3) is equal to (01) in binary, and (1 OR 3) is equal to (11) in binary. The total number of set bits is 1 + 2 = 3.
/// So the number of excellent pairs is 5.
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [5,1,1], k = 10
/// Output: 0
/// Explanation: There are no excellent pairs for this array.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>9</sup>`
/// * `1 <= k <= 60`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-excellent-pairs/
// discuss: https://leetcode.com/problems/number-of-excellent-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_excellent_pairs(mut nums: Vec<i32>, k: i32) -> i64 {
        nums.sort_unstable();
        nums.dedup();

        let mut count_bits = vec![0; 31];

        for num in nums {
            count_bits[num.count_ones() as usize] += 1;
        }

        let mut prefix_sum = vec![0; 61];
        for i in (0..31).rev() {
            prefix_sum[i] = prefix_sum[i + 1] + count_bits[i];
        }

        let k = k as usize;
        let mut result = 0;

        for (i, &count) in count_bits.iter().enumerate().filter(|(_, &x)| x > 0) {
            if i * 2 >= k {
                result += count * count;
            }

            result += 2 * count * prefix_sum[(i + 1).max(k.saturating_sub(i))];
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2354() {
        let nums = vec![1, 2, 3, 1];
        let k = 3;
        let expected = 5;
        assert_eq!(Solution::count_excellent_pairs(nums, k), expected);
        let nums = vec![5, 1, 1];
        let k = 10;
        let expected = 0;
        assert_eq!(Solution::count_excellent_pairs(nums, k), expected);
    }
}
