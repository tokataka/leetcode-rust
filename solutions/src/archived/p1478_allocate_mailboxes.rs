///
/// # 1478. Allocate Mailboxes
///
/// Given the array `houses` where `houses[i]` is the location of the `i<sup>th</sup>` house along a street and an integer `k`, allocate `k` mailboxes in the street.
///
/// Return *the **minimum** total distance between each house and its nearest mailbox*.
///
/// The test cases are generated so that the answer fits in a 32-bit integer.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2020/05/07/sample_11_1816.png)
///
/// ```
/// Input: houses = [1,4,8,10,20], k = 3
/// Output: 5
/// Explanation: Allocate mailboxes in position 3, 9 and 20.
/// Minimum total distance from each houses to nearest mailboxes is |3-1| + |4-3| + |9-8| + |10-9| + |20-20| = 5
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2020/05/07/sample_2_1816.png)
///
/// ```
/// Input: houses = [2,3,5,12,18], k = 2
/// Output: 9
/// Explanation: Allocate mailboxes in position 3 and 14.
/// Minimum total distance from each houses to nearest mailboxes is |2-3| + |3-3| + |5-3| + |12-14| + |18-14| = 9.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= k <= houses.length <= 100`
/// * `1 <= houses[i] <= 10<sup>4</sup>`
/// * All the integers of `houses` are **unique**.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/allocate-mailboxes/
// discuss: https://leetcode.com/problems/allocate-mailboxes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_distance(mut houses: Vec<i32>, k: i32) -> i32 {
        let n = houses.len();
        let k = k as usize;

        fn _min_distance(
            cur: usize,
            k: usize,
            n: usize,
            houses_prefix_sum: &[i32],
            cache: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if cur == n {
                return 0;
            }

            if cache[cur][k] != i32::MAX {
                return cache[cur][k];
            }

            let next_range = match k {
                0 => unreachable!(),
                1 => n..=n,
                k => cur + 1..=n - k + 1,
            };

            let mut result = i32::MAX / 2;

            for next in next_range {
                let half = (next - cur) / 2;

                let distance = houses_prefix_sum[next]
                    - houses_prefix_sum[next - half]
                    - houses_prefix_sum[cur + half]
                    - houses_prefix_sum[cur];

                result =
                    result.min(distance + _min_distance(next, k - 1, n, houses_prefix_sum, cache));
            }

            cache[cur][k] = result;

            result
        }

        houses.sort_unstable();

        let mut houses_prefix_sum = vec![0];
        let mut sum = 0;

        for house in &houses {
            sum += house;
            houses_prefix_sum.push(sum);
        }

        _min_distance(
            0,
            k,
            n,
            &houses_prefix_sum,
            &mut vec![vec![i32::MAX; k + 1]; n],
        )
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1478() {
        let houses = vec![1, 4, 8, 10, 20];
        let k = 3;
        let expected = 5;
        assert_eq!(Solution::min_distance(houses, k), expected);
        let houses = vec![2, 3, 5, 12, 18];
        let k = 2;
        let expected = 9;
        assert_eq!(Solution::min_distance(houses, k), expected);
    }
}
