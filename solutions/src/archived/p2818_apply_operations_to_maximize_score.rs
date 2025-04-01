///
/// # 2818. Apply Operations to Maximize Score
///
/// You are given an array `nums` of `n` positive integers and an integer `k`.
///
/// Initially, you start with a score of `1`. You have to maximize your score by applying the following operation at most `k` times:
///
/// * Choose any **non-empty** subarray `nums[l, ..., r]` that you haven't chosen previously.
/// * Choose an element `x` of `nums[l, ..., r]` with the highest **prime score**. If multiple such elements exist, choose the one with the smallest index.
/// * Multiply your score by `x`.
///
/// Here, `nums[l, ..., r]` denotes the subarray of `nums` starting at index `l` and ending at the index `r`, both ends being inclusive.
///
/// The **prime score** of an integer `x` is equal to the number of distinct prime factors of `x`. For example, the prime score of `300` is `3` since `300 = 2 * 2 * 3 * 5 * 5`.
///
/// Return *the **maximum possible score** after applying at most* `k` *operations*.
///
/// Since the answer may be large, return it modulo `10<sup>9 </sup>+ 7`.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [8,3,9,3,8], k = 2
/// Output: 81
/// Explanation: To get a score of 81, we can apply the following operations:
/// - Choose subarray nums[2, ..., 2]. nums[2] is the only element in this subarray. Hence, we multiply the score by nums[2]. The score becomes 1 * 9 = 9.
/// - Choose subarray nums[2, ..., 3]. Both nums[2] and nums[3] have a prime score of 1, but nums[2] has the smaller index. Hence, we multiply the score by nums[2]. The score becomes 9 * 9 = 81.
/// It can be proven that 81 is the highest score one can obtain.
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [19,12,14,6,10,18], k = 3
/// Output: 4788
/// Explanation: To get a score of 4788, we can apply the following operations:
/// - Choose subarray nums[0, ..., 0]. nums[0] is the only element in this subarray. Hence, we multiply the score by nums[0]. The score becomes 1 * 19 = 19.
/// - Choose subarray nums[5, ..., 5]. nums[5] is the only element in this subarray. Hence, we multiply the score by nums[5]. The score becomes 19 * 18 = 342.
/// - Choose subarray nums[2, ..., 3]. Both nums[2] and nums[3] have a prime score of 2, but nums[2] has the smaller index. Hence, we multipy the score by nums[2]. The score becomes 342 * 14 = 4788.
/// It can be proven that 4788 is the highest score one can obtain.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length == n <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>5</sup>`
/// * `1 <= k <= min(n * (n + 1) / 2, 10<sup>9</sup>)`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/apply-operations-to-maximize-score/
// discuss: https://leetcode.com/problems/apply-operations-to-maximize-score/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut primes = (0..=100000).collect::<Vec<_>>();

        let mut p = 2;
        while p * p <= 100000 {
            if primes[p] == p {
                for i in (p * p..=100000).step_by(p) {
                    if primes[i] == i {
                        primes[i] = p;
                    }
                }
            }

            p += 1;
        }

        let prime_score = nums
            .iter()
            .map(|&x| {
                if x == 1 {
                    return 0;
                }

                let mut cur = x as usize;
                let mut score = 0;

                while cur > 1 {
                    let p = primes[cur];

                    while primes[cur] == p {
                        cur /= primes[cur];
                    }

                    score += 1;
                }

                score
            })
            .collect::<Vec<_>>();

        let mut monotonic_left: Vec<(usize, usize)> = vec![];
        let mut left = vec![];

        let mut monotonic_right: Vec<(usize, usize)> = vec![];
        let mut right = vec![];

        for i in 0..nums.len() {
            let cur_left = prime_score[i];

            while let Some(&last) = monotonic_left.last() {
                if last.1 >= cur_left {
                    break;
                }

                monotonic_left.pop();
            }

            if let Some(&last) = monotonic_left.last() {
                left.push(last.0 + 1);
            } else {
                left.push(0);
            }

            monotonic_left.push((i, cur_left));

            let i = nums.len() - 1 - i;
            let cur_right = prime_score[i];

            while let Some(&last) = monotonic_right.last() {
                if last.1 > cur_right {
                    break;
                }

                monotonic_right.pop();
            }

            if let Some(&last) = monotonic_right.last() {
                right.push(last.0 - 1);
            } else {
                right.push(nums.len() - 1);
            }

            monotonic_right.push((i, cur_right));
        }

        let count = left
            .into_iter()
            .zip(right.into_iter().rev())
            .enumerate()
            .map(|(i, (l, r))| ((i + 1 - l) * (r + 1 - i)) as i64);

        let mut num_count = nums
            .into_iter()
            .map(|num| num as i64)
            .zip(count)
            .collect::<Vec<_>>();

        num_count.sort_unstable_by(|a, b| b.cmp(a));

        fn power(mut x: i64, mut y: i64, p: i64) -> i64 {
            let mut result = 1;

            x %= p;

            if x == 0 {
                return 0;
            }

            while y > 0 {
                if y & 1 == 1 {
                    result = (result * x) % p;
                }

                y >>= 1;
                x = (x * x) % p;
            }

            result
        }

        num_count
            .iter()
            .scan(k as i64, |remain, &(num, count)| {
                if *remain == 0 {
                    return None;
                }

                let result = power(num, (*remain).min(count), MOD);

                *remain = (*remain - count).max(0);

                Some(result)
            })
            .fold(1, |acc, x| (acc * x) % MOD) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2818() {
        let nums = vec![8, 3, 9, 3, 8];
        let k = 2;
        let expected = 81;
        assert_eq!(Solution::maximum_score(nums, k), expected);
        let nums = vec![19, 12, 14, 6, 10, 18];
        let k = 3;
        let expected = 4788;
        assert_eq!(Solution::maximum_score(nums, k), expected);
    }
}
