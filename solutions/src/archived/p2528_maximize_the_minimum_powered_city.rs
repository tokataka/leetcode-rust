///
/// # 2528. Maximize the Minimum Powered City
///
/// You are given a **0-indexed** integer array `stations` of length `n`, where `stations[i]` represents the number of power stations in the `i<sup>th</sup>` city.
///
/// Each power station can provide power to every city in a fixed **range**. In other words, if the range is denoted by `r`, then a power station at city `i` can provide power to all cities `j` such that `|i - j| <= r` and `0 <= i, j <= n - 1`.
///
/// * Note that `|x|` denotes **absolute** value. For example, `|7 - 5| = 2` and `|3 - 10| = 7`.
///
/// The **power** of a city is the total number of power stations it is being provided power from.
///
/// The government has sanctioned building `k` more power stations, each of which can be built in any city, and have the same range as the pre-existing ones.
///
/// Given the two integers `r` and `k`, return *the **maximum possible minimum power** of a city, if the additional power stations are built optimally.*
///
/// **Note** that you can build the `k` power stations in multiple cities.
///
/// **Example 1:**
///
/// ```
/// Input: stations = [1,2,4,5,0], r = 1, k = 2
/// Output: 5
/// Explanation:
/// One of the optimal ways is to install both the power stations at city 1.
/// So stations will become [1,4,4,5,0].
/// - City 0 is provided by 1 + 4 = 5 power stations.
/// - City 1 is provided by 1 + 4 + 4 = 9 power stations.
/// - City 2 is provided by 4 + 4 + 5 = 13 power stations.
/// - City 3 is provided by 5 + 4 = 9 power stations.
/// - City 4 is provided by 5 + 0 = 5 power stations.
/// So the minimum power of a city is 5.
/// Since it is not possible to obtain a larger power, we return 5.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: stations = [4,4,4,4], r = 0, k = 3
/// Output: 4
/// Explanation:
/// It can be proved that we cannot make the minimum power of a city greater than 4.
///
/// ```
///
/// **Constraints:**
///
/// * `n == stations.length`
/// * `1 <= n <= 10<sup>5</sup>`
/// * `0 <= stations[i] <= 10<sup>5</sup>`
/// * `0 <= r <= n - 1`
/// * `0 <= k <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximize-the-minimum-powered-city/
// discuss: https://leetcode.com/problems/maximize-the-minimum-powered-city/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let r = r as usize;
        let k = k as i64;
        let n = stations.len();

        let mut base_powers = vec![];
        let mut window_sum = stations.iter().take(r).map(|&x| x as i64).sum::<i64>();

        for i in 0..n {
            if i + r < n {
                window_sum += stations[i + r] as i64;
            }

            base_powers.push(window_sum);

            if i >= r {
                window_sum -= stations[i - r] as i64;
            }
        }

        let mut left = 0;
        let mut right = 11_000_000_000;
        let mut diff = vec![0; n + 2 * r + 1];

        'outer: while left < right {
            let target_power = (left + right + 1) / 2;

            let mut remain_install = k;
            let mut diff_sum = 0;

            for (i, &base_power) in base_powers.iter().enumerate() {
                diff_sum += diff[i];
                diff[i] = 0;

                let install = match target_power - (base_power + diff_sum) {
                    x if x > remain_install => {
                        right = target_power - 1;
                        continue 'outer;
                    }
                    x if x <= 0 => {
                        diff[i + 2 * r + 1] = 0;
                        continue;
                    }
                    x => x,
                };

                remain_install -= install;
                diff_sum += install;
                diff[i + 2 * r + 1] = -install;
            }

            left = target_power;
        }

        left
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2528() {
        let stations = vec![1, 2, 4, 5, 0];
        let r = 1;
        let k = 2;
        let expected = 5;
        assert_eq!(Solution::max_power(stations, r, k), expected);
        let stations = vec![4, 4, 4, 4];
        let r = 0;
        let k = 3;
        let expected = 4;
        assert_eq!(Solution::max_power(stations, r, k), expected);
    }
}
