///
/// # 3669. Balanced K-Factor Decomposition
///
/// Given two integers `n` and `k`, split the number `n` into exactly `k` positive integers such that the **product** of these integers is equal to `n`.
///
/// Return *any* *one* split in which the **maximum** difference between any two numbers is **minimized**. You may return the result in *any order*.
///
/// **Example 1:**
///
/// **Input:** n = 100, k = 2
///
/// **Output:** [10,10]
///
/// **Explanation:**
///
/// The split `[10, 10]` yields `10 * 10 = 100` and a max-min difference of 0, which is minimal.
///
/// **Example 2:**
///
/// **Input:** n = 44, k = 3
///
/// **Output:** [2,2,11]
///
/// **Explanation:**
///
/// * Split `[1, 1, 44]` yields a difference of 43
/// * Split `[1, 2, 22]` yields a difference of 21
/// * Split `[1, 4, 11]` yields a difference of 10
/// * Split `[2, 2, 11]` yields a difference of 9
///
/// Therefore, `[2, 2, 11]` is the optimal split with the smallest difference 9.
///
/// **Constraints:**
///
/// * `4 <= n <= 10<sup><span style="font-size: 10.8333px;">5</span></sup>`
/// * `2 <= k <= 5`
/// * `k` is strictly less than the total number of positive divisors of `n`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/balanced-k-factor-decomposition/
// discuss: https://leetcode.com/problems/balanced-k-factor-decomposition/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn min_difference(n: i32, k: i32) -> Vec<i32> {
        fn _min_difference(n: i32, k: usize, path: &mut Vec<i32>, min_path: &mut Vec<i32>) {
            if path.len() == k - 1 {
                fn diff(x: &[i32]) -> i32 {
                    x.iter().max().unwrap() - x.iter().min().unwrap()
                }

                path.push(n);

                if diff(path) < diff(min_path) {
                    *min_path = path.clone();
                }

                path.pop();

                return;
            }

            path.push(n);
            _min_difference(1, k, path, min_path);
            path.pop();

            for x in 2..=n / 2 {
                if n % x == 0 {
                    path.push(x);
                    _min_difference(n / x, k, path, min_path);
                    path.pop();
                }
            }
        }

        let k = k as usize;

        let mut result = vec![1; k];
        result[0] = n;

        _min_difference(n, k, &mut vec![], &mut result);

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3669() {
        let n = 100;
        let k = 2;
        let expected = vec![10, 10];
        assert_eq!(Solution::min_difference(n, k), expected);
        let n = 44;
        let k = 3;
        let expected = vec![2, 2, 11];
        assert_eq!(Solution::min_difference(n, k), expected);
    }
}
