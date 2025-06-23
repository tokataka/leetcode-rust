///
/// # 3443. Maximum Manhattan Distance After K Changes
///
/// You are given a string `s` consisting of the characters `'N'`, `'S'`, `'E'`, and `'W'`, where `s[i]` indicates movements in an infinite grid:
///
/// * `'N'` : Move north by 1 unit.
/// * `'S'` : Move south by 1 unit.
/// * `'E'` : Move east by 1 unit.
/// * `'W'` : Move west by 1 unit.
///
/// Initially, you are at the origin `(0, 0)`. You can change **at most** `k` characters to any of the four directions.
///
/// Find the **maximum** **Manhattan distance** from the origin that can be achieved **at any time** while performing the movements **in order**.
///
/// The **Manhattan Distance** between two cells `(x<sub>i</sub>, y<sub>i</sub>)` and `(x<sub>j</sub>, y<sub>j</sub>)` is `|x<sub>i</sub> - x<sub>j</sub>| + |y<sub>i</sub> - y<sub>j</sub>|`.
///
/// **Example 1:**
///
/// **Input:** s = "NWSE", k = 1
///
/// **Output:** 3
///
/// **Explanation:**
///
/// Change `s[2]` from `'S'` to `'N'`. The string `s` becomes `"NWNE"`.
///
/// | Movement  |Position (x, y)|Manhattan Distance|Maximum|
/// |-----------|---------------|------------------|-------|
/// |s[0] == 'N'|    (0, 1)     |    0 + 1 = 1     |   1   |
/// |s[1] == 'W'|    (-1, 1)    |    1 + 1 = 2     |   2   |
/// |s[2] == 'N'|    (-1, 2)    |    1 + 2 = 3     |   3   |
/// |s[3] == 'E'|    (0, 2)     |    0 + 2 = 2     |   3   |
///
/// The maximum Manhattan distance from the origin that can be achieved is 3. Hence, 3 is the output.
///
/// **Example 2:**
///
/// **Input:** s = "NSWWEW", k = 3
///
/// **Output:** 6
///
/// **Explanation:**
///
/// Change `s[1]` from `'S'` to `'N'`, and `s[4]` from `'E'` to `'W'`. The string `s` becomes `"NNWWWW"`.
///
/// The maximum Manhattan distance from the origin that can be achieved is 6. Hence, 6 is the output.
///
/// **Constraints:**
///
/// * `1 <= s.length <= 10<sup>5</sup>`
/// * `0 <= k <= s.length`
/// * `s` consists of only `'N'`, `'S'`, `'E'`, and `'W'`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-manhattan-distance-after-k-changes/
// discuss: https://leetcode.com/problems/maximum-manhattan-distance-after-k-changes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::VecDeque;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let mut x = 0i32;
        let mut y = 0i32;
        let mut cur = 0;

        let mut result = 0;
        let mut q = VecDeque::new();
        let mut decline_count = 0;
        let mut cur_distance = 0;

        for ch in s.bytes() {
            match ch {
                b'N' => y += 1,
                b'S' => y -= 1,
                b'E' => x += 1,
                b'W' => x -= 1,
                _ => unreachable!(),
            };

            let next = x.abs() + y.abs();
            let diff = next - cur;

            q.push_back(diff);

            if diff < 0 {
                decline_count += 1;
            }

            if decline_count > k {
                while let Some(x) = q.pop_front() {
                    cur_distance += x;
                    if x == -1 {
                        break;
                    }
                }

                decline_count -= 1;
            }

            result = result.max(cur_distance + q.len() as i32);

            cur = next;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3443() {
        let s = "NWSE".to_owned();
        let k = 1;
        let expected = 3;
        assert_eq!(Solution::max_distance(s, k), expected);
        let s = "NSWWEW".to_owned();
        let k = 3;
        let expected = 6;
        assert_eq!(Solution::max_distance(s, k), expected);
    }
}
