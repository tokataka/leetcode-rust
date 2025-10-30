///
/// # 3186. Maximum Total Damage With Spell Casting
///
/// A magician has various spells.
///
/// You are given an array `power`, where each element represents the damage of a spell. Multiple spells can have the same damage value.
///
/// It is a known fact that if a magician decides to cast a spell with a damage of `power[i]`, they **cannot** cast any spell with a damage of `power[i] - 2`, `power[i] - 1`, `power[i] + 1`, or `power[i] + 2`.
///
/// Each spell can be cast **only once**.
///
/// Return the **maximum** possible *total damage* that a magician can cast.
///
/// **Example 1:**
///
/// **Input:** power = [1,1,3,4]
///
/// **Output:** 6
///
/// **Explanation:**
///
/// The maximum possible damage of 6 is produced by casting spells 0, 1, 3 with damage 1, 1, 4.
///
/// **Example 2:**
///
/// **Input:** power = [7,1,6,6]
///
/// **Output:** 13
///
/// **Explanation:**
///
/// The maximum possible damage of 13 is produced by casting spells 1, 2, 3 with damage 1, 6, 6.
///
/// **Constraints:**
///
/// * `1 <= power.length <= 10<sup>5</sup>`
/// * `1 <= power[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-total-damage-with-spell-casting/
// discuss: https://leetcode.com/problems/maximum-total-damage-with-spell-casting/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::VecDeque;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        let mut power = power;

        power.sort_unstable();

        let mut q = VecDeque::new();
        let mut prev_damage = 0;

        for chunk in power.chunk_by(|a, b| a == b) {
            let cur_power = chunk[0] as i64;
            let same_count = chunk.len() as i64;

            while let Some(&(p, damage)) = q.front() {
                if p >= cur_power - 2 {
                    break;
                }

                q.pop_front();
                prev_damage = prev_damage.max(damage);
            }

            q.push_back((cur_power, prev_damage + cur_power * same_count));
        }

        q.into_iter().map(|x| x.1).max().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3186() {
        let power = vec![1, 1, 3, 4];
        let expected = 6;
        assert_eq!(Solution::maximum_total_damage(power), expected);
        let power = vec![7, 1, 6, 6];
        let expected = 13;
        assert_eq!(Solution::maximum_total_damage(power), expected);
    }
}
