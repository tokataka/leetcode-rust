///
/// # 2226. Maximum Candies Allocated to K Children
///
/// You are given a **0-indexed** integer array `candies`. Each element in the array denotes a pile of candies of size `candies[i]`. You can divide each pile into any number of **sub piles**, but you **cannot** merge two piles together.
///
/// You are also given an integer `k`. You should allocate piles of candies to `k` children such that each child gets the **same** number of candies. Each child can be allocated candies from **only one** pile of candies and some piles of candies may go unused.
///
/// Return *the **maximum number of candies** each child can get.*
///
/// **Example 1:**
///
/// ```
/// Input: candies = [5,8,6], k = 3
/// Output: 5
/// Explanation: We can divide candies[1] into 2 piles of size 5 and 3, and candies[2] into 2 piles of size 5 and 1. We now have five piles of candies of sizes 5, 5, 3, 5, and 1. We can allocate the 3 piles of size 5 to 3 children. It can be proven that each child cannot receive more than 5 candies.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: candies = [2,5], k = 11
/// Output: 0
/// Explanation: There are 11 children but only 7 candies in total, so it is impossible to ensure each child receives at least one candy. Thus, each child gets no candy and the answer is 0.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= candies.length <= 10<sup>5</sup>`
/// * `1 <= candies[i] <= 10<sup>7</sup>`
/// * `1 <= k <= 10<sup>12</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-candies-allocated-to-k-children/
// discuss: https://leetcode.com/problems/maximum-candies-allocated-to-k-children/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let mut left = 0;
        let mut right = 10_000_001;
        let mut mid = 5_000_000;

        while left < right - 1 {
            if candies.iter().map(|&x| x as i64 / mid).sum::<i64>() < k {
                right = mid;
            } else {
                left = mid;
            }
            mid = (left + right) / 2;
        }

        mid as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2226() {
        let candies = vec![5, 8, 6];
        let k = 3;
        let expected = 5;
        assert_eq!(Solution::maximum_candies(candies, k), expected);
        let candies = vec![2, 5];
        let k = 11;
        let expected = 0;
        assert_eq!(Solution::maximum_candies(candies, k), expected);
    }
}
