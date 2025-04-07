///
/// # 368. Largest Divisible Subset
///
/// Given a set of **distinct** positive integers `nums`, return the largest subset `answer` such that every pair `(answer[i], answer[j])` of elements in this subset satisfies:
///
/// * `answer[i] % answer[j] == 0`, or
/// * `answer[j] % answer[i] == 0`
///
/// If there are multiple solutions, return any of them.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,2,3]
/// Output: [1,2]
/// Explanation: [1,3] is also accepted.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,2,4,8]
/// Output: [1,2,4,8]
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 1000`
/// * `1 <= nums[i] <= 2 * 10<sup>9</sup>`
/// * All the integers in `nums` are **unique**.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-divisible-subset/
// discuss: https://leetcode.com/problems/largest-divisible-subset/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();

        let mut path = (0..nums.len()).map(|i| (i, 0)).collect::<Vec<_>>();

        for (i, &a) in nums.iter().enumerate() {
            for (j, &b) in nums.iter().enumerate().skip(i + 1) {
                if b % a == 0 && path[i].1 + 1 > path[j].1 {
                    path[j] = (i, path[i].1 + 1);
                }
            }
        }

        let max_end = path
            .iter()
            .enumerate()
            .max_by(|a, b| a.1 .1.cmp(&b.1 .1))
            .unwrap();

        let mut result = Vec::with_capacity(max_end.1 .1);
        let mut cur = max_end.0;

        result.push(nums[cur]);

        while cur != path[cur].0 {
            cur = path[cur].0;
            result.push(nums[cur]);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_368() {
        let nums = vec![1, 2, 3];
        let expected = vec![1, 2];
        assert_eq!(Solution::largest_divisible_subset(nums), expected);
        let nums = vec![1, 2, 4, 8];
        let expected = vec![1, 2, 4, 8];
        assert_eq!(Solution::largest_divisible_subset(nums), expected);
    }
}
