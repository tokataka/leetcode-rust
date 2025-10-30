///
/// # 996. Number of Squareful Arrays
///
/// An array is **squareful** if the sum of every pair of adjacent elements is a **perfect square**.
///
/// Given an integer array nums, return *the number of permutations of* `nums` *that are **squareful***.
///
/// Two permutations `perm1` and `perm2` are different if there is some index `i` such that `perm1[i] != perm2[i]`.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,17,8]
/// Output: 2
/// Explanation: [1,8,17] and [17,8,1] are the valid permutations.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [2,2,2]
/// Output: 1
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 12`
/// * `0 <= nums[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-squareful-arrays/
// discuss: https://leetcode.com/problems/number-of-squareful-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn num_squareful_perms(nums: Vec<i32>) -> i32 {
        fn _num_squareful_perms(
            i: usize,
            state: i32,
            dp: &mut HashMap<(usize, i32), i32>,
            square_map: &Vec<Vec<usize>>,
        ) -> i32 {
            if state == (1 << square_map.len()) - 1 {
                return 1;
            }

            if let Some(&x) = dp.get(&(i, state)) {
                return x;
            }

            let mut result = 0;

            for &j in &square_map[i] {
                if state & (1 << j) == 0 {
                    result += _num_squareful_perms(j, state | (1 << j), dp, square_map);
                }
            }

            dp.insert((i, state), result);

            result
        }

        let n = nums.len();

        let mut square_map = vec![vec![]; n];

        for (i, &num1) in nums.iter().enumerate() {
            for (j, &num2) in nums.iter().enumerate().skip(i + 1) {
                let sqrt = ((num1 + num2) as f64).sqrt().floor() as i32;

                if num1 + num2 == sqrt * sqrt {
                    square_map[i].push(j);
                    square_map[j].push(i);
                }
            }
        }

        let mut num_freq = HashMap::new();

        for &num in &nums {
            num_freq.entry(num).and_modify(|x| *x += 1).or_insert(1);
        }

        let dup: i32 = num_freq
            .values()
            .map(|&x| (1..=x).product::<i32>())
            .product();

        let mut result = 0;
        let mut dp = HashMap::new();

        for i in 0..n {
            result += _num_squareful_perms(i, 1 << i, &mut dp, &square_map);
        }

        result / dup
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_996() {
        let nums = vec![1, 17, 8];
        let expected = 2;
        assert_eq!(Solution::num_squareful_perms(nums), expected);
        let nums = vec![2, 2, 2];
        let expected = 1;
        assert_eq!(Solution::num_squareful_perms(nums), expected);
    }
}
