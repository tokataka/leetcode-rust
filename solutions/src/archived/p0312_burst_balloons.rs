///
/// # 312. Burst Balloons
///
/// You are given `n` balloons, indexed from `0` to `n - 1`. Each balloon is painted with a number on it represented by an array `nums`. You are asked to burst all the balloons.
///
/// If you burst the `i<sup>th</sup>` balloon, you will get `nums[i - 1] * nums[i] * nums[i + 1]` coins. If `i - 1` or `i + 1` goes out of bounds of the array, then treat it as if there is a balloon with a `1` painted on it.
///
/// Return *the maximum coins you can collect by bursting the balloons wisely*.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [3,1,5,8]
/// Output: 167
/// Explanation:
/// nums = [3,1,5,8] --> [3,5,8] --> [3,8] --> [8] --> []
/// coins =  3*1*5    +   3*5*8   +  1*3*8  + 1*8*1 = 167
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,5]
/// Output: 10
///
/// ```
///
/// **Constraints:**
///
/// * `n == nums.length`
/// * `1 <= n <= 300`
/// * `0 <= nums[i] <= 100`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/burst-balloons/
// discuss: https://leetcode.com/problems/burst-balloons/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let nums = [1].into_iter().chain(nums).chain([1]).collect::<Vec<_>>();
        let mut cache = vec![vec![None; nums.len()]; nums.len()];

        fn _max_coins(
            cache: &mut Vec<Vec<Option<i32>>>,
            nums: &Vec<i32>,
            left: usize,
            right: usize,
        ) -> i32 {
            if let Some(x) = cache[left][right] {
                return x;
            }

            if left > right {
                return 0;
            }

            let mut result = 0;

            for mid in left..=right {
                let coins = nums[left - 1] * nums[right + 1] * nums[mid]
                    + _max_coins(cache, nums, left, mid - 1)
                    + _max_coins(cache, nums, mid + 1, right);

                result = result.max(coins);
            }

            cache[left][right] = Some(result);

            result
        }

        _max_coins(&mut cache, &nums, 1, nums.len() - 2)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_312() {
        let nums = vec![3, 1, 5, 8];
        let expected = 167;
        assert_eq!(Solution::max_coins(nums), expected);
        let nums = vec![1, 5];
        let expected = 10;
        assert_eq!(Solution::max_coins(nums), expected);
    }
}
