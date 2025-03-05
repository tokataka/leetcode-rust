///
/// # 1524. Number of Sub-arrays With Odd Sum
///
/// Given an array of integers `arr`, return *the number of subarrays with an **odd** sum*.
///
/// Since the answer can be very large, return it modulo `10<sup>9</sup> + 7`.
///
/// **Example 1:**
///
/// ```
/// Input: arr = [1,3,5]
/// Output: 4
/// Explanation: All subarrays are [[1],[1,3],[1,3,5],[3],[3,5],[5]]
/// All sub-arrays sum are [1,4,9,3,8,5].
/// Odd sums are [1,9,3,5] so the answer is 4.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: arr = [2,4,6]
/// Output: 0
/// Explanation: All subarrays are [[2],[2,4],[2,4,6],[4],[4,6],[6]]
/// All sub-arrays sum are [2,6,12,4,10,6].
/// All sub-arrays have even sum and the answer is 0.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: arr = [1,2,3,4,5,6,7]
/// Output: 16
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= arr.length <= 10<sup>5</sup>`
/// * `1 <= arr[i] <= 100`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum/
// discuss: https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;

        let mut odd_count = 0;
        let mut even_count = 0;

        let mut result = 0;

        for cur in arr {
            (odd_count, even_count) = match cur & 1 {
                0 => (odd_count, even_count + 1),
                _ => (even_count + 1, odd_count),
            };

            result = (result + odd_count) % MOD;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1524() {
        let arr = vec![1, 3, 5];
        let expected = 4;
        assert_eq!(Solution::num_of_subarrays(arr), expected);
        let arr = vec![2, 4, 6];
        let expected = 0;
        assert_eq!(Solution::num_of_subarrays(arr), expected);
        let arr = vec![1, 2, 3, 4, 5, 6, 7];
        let expected = 16;
        assert_eq!(Solution::num_of_subarrays(arr), expected);
    }
}
