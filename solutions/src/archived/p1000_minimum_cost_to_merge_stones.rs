///
/// # 1000. Minimum Cost to Merge Stones
///
/// There are `n` piles of `stones` arranged in a row. The `i<sup>th</sup>` pile has `stones[i]` stones.
///
/// A move consists of merging exactly `k` **consecutive** piles into one pile, and the cost of this move is equal to the total number of stones in these `k` piles.
///
/// Return *the minimum cost to merge all piles of stones into one pile*. If it is impossible, return `-1`.
///
/// **Example 1:**
///
/// ```
/// Input: stones = [3,2,4,1], k = 2
/// Output: 20
/// Explanation: We start with [3, 2, 4, 1].
/// We merge [3, 2] for a cost of 5, and we are left with [5, 4, 1].
/// We merge [4, 1] for a cost of 5, and we are left with [5, 5].
/// We merge [5, 5] for a cost of 10, and we are left with [10].
/// The total cost was 20, and this is the minimum possible.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: stones = [3,2,4,1], k = 3
/// Output: -1
/// Explanation: After any merge operation, there are 2 piles left, and we can't merge anymore.  So the task is impossible.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: stones = [3,5,1,2,6], k = 3
/// Output: 25
/// Explanation: We start with [3, 5, 1, 2, 6].
/// We merge [5, 1, 2] for a cost of 8, and we are left with [3, 8, 6].
/// We merge [3, 8, 6] for a cost of 17, and we are left with [17].
/// The total cost was 25, and this is the minimum possible.
///
/// ```
///
/// **Constraints:**
///
/// * `n == stones.length`
/// * `1 <= n <= 30`
/// * `1 <= stones[i] <= 100`
/// * `2 <= k <= 30`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-to-merge-stones/
// discuss: https://leetcode.com/problems/minimum-cost-to-merge-stones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        fn _merge_stones(
            start: usize,
            end: usize,
            remain: usize,
            dp: &mut Vec<Vec<Vec<i32>>>,
            prefix_sum: &[i32],
            k: usize,
        ) -> i32 {
            if dp[start][end][remain] != i32::MAX {
                return dp[start][end][remain];
            }

            if end - start == 1 {
                return 0;
            }

            if remain == 0 {
                return _merge_stones(start, end, k - 1, dp, prefix_sum, k);
            }

            let mut result = i32::MAX;

            for cur_end in (0..)
                .map(|i| start + 1 + (k - 1) * i)
                .take_while(|&x| x <= end - remain)
            {
                result = result.min(
                    _merge_stones(start, cur_end, k - 1, dp, prefix_sum, k)
                        + _merge_stones(cur_end, end, remain - 1, dp, prefix_sum, k),
                );
            }

            if remain == k - 1 {
                result += prefix_sum[end] - prefix_sum[start];
            }

            dp[start][end][remain] = result;

            result
        }

        let n = stones.len();
        let k = k as usize;

        if k > 2 && n % (k - 1) != 1 {
            return -1;
        }

        let mut prefix_sum = vec![0];

        for &stone in &stones {
            prefix_sum.push(stone + prefix_sum.last().unwrap());
        }

        let mut dp = vec![vec![vec![i32::MAX; k]; n + 1]; n + 1];

        _merge_stones(0, n, k - 1, &mut dp, &prefix_sum, k)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1000() {
        let stones = vec![3, 2, 4, 1];
        let k = 2;
        let expected = 20;
        assert_eq!(Solution::merge_stones(stones, k), expected);
        let stones = vec![3, 2, 4, 1];
        let k = 3;
        let expected = -1;
        assert_eq!(Solution::merge_stones(stones, k), expected);
        let stones = vec![3, 5, 1, 2, 6];
        let k = 3;
        let expected = 25;
        assert_eq!(Solution::merge_stones(stones, k), expected);
    }
}
