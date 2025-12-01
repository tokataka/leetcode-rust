///
/// # 60. Permutation Sequence
///
/// The set `[1, 2, 3, ..., n]` contains a total of `n!` unique permutations.
///
/// By listing and labeling all of the permutations in order, we get the following sequence for `n = 3`:
///
/// 1. `"123"`
/// 2. `"132"`
/// 3. `"213"`
/// 4. `"231"`
/// 5. `"312"`
/// 6. `"321"`
///
/// Given `n` and `k`, return the `k<sup>th</sup>` permutation sequence.
///
/// **Example 1:**
///
/// ```
/// Input: n = 3, k = 3
/// Output: "213"
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 4, k = 9
/// Output: "2314"
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: n = 3, k = 1
/// Output: "123"
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 9`
/// * `1 <= k <= n!`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/permutation-sequence/
// discuss: https://leetcode.com/problems/permutation-sequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut k = k - 1;
        let mut cur = (1..n).product::<i32>();

        let mut result = String::new();
        let mut nums = (1..=n).collect::<Vec<_>>();

        for x in (1..n).rev() {
            result.push(char::from_digit(nums.remove((k / cur) as usize) as u32, 10).unwrap());
            k %= cur;
            cur /= x;
        }

        result.push(char::from_digit(nums[0] as u32, 10).unwrap());

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_60() {
        let n = 3;
        let k = 3;
        let expected = "213".to_owned();
        assert_eq!(Solution::get_permutation(n, k), expected);
        let n = 4;
        let k = 9;
        let expected = "2314".to_owned();
        assert_eq!(Solution::get_permutation(n, k), expected);
        let n = 3;
        let k = 1;
        let expected = "123".to_owned();
        assert_eq!(Solution::get_permutation(n, k), expected);
    }
}
