///
/// # 3494. Find the Minimum Amount of Time to Brew Potions
///
/// You are given two integer arrays, `skill` and `mana`, of length `n` and `m`, respectively.
///
/// In a laboratory, `n` wizards must brew `m` potions *in order*. Each potion has a mana capacity `mana[j]` and **must** pass through **all** the wizards sequentially to be brewed properly. The time taken by the `i<sup>th</sup>` wizard on the `j<sup>th</sup>` potion is `time<sub>ij</sub> = skill[i] * mana[j]`.
///
/// Since the brewing process is delicate, a potion **must** be passed to the next wizard immediately after the current wizard completes their work. This means the timing must be *synchronized* so that each wizard begins working on a potion **exactly** when it arrives. ​
///
/// Return the **minimum** amount of time required for the potions to be brewed properly.
///
/// **Example 1:**
///
/// **Input:** skill = [1,5,2,4], mana = [5,1,4,2]
///
/// **Output:** 110
///
/// **Explanation:**
///
/// |Potion Number|Start time|Wizard 0 done by|Wizard 1 done by|Wizard 2 done by|Wizard 3 done by|
/// |-------------|----------|----------------|----------------|----------------|----------------|
/// |      0      |    0     |       5        |       30       |       40       |       60       |
/// |      1      |    52    |       53       |       58       |       60       |       64       |
/// |      2      |    54    |       58       |       78       |       86       |      102       |
/// |      3      |    86    |       88       |       98       |      102       |      110       |
///
/// As an example for why wizard 0 cannot start working on the 1<sup>st</sup> potion before time `t = 52`, consider the case where the wizards started preparing the 1<sup>st</sup> potion at time `t = 50`. At time `t = 58`, wizard 2 is done with the 1<sup>st</sup> potion, but wizard 3 will still be working on the 0<sup>th</sup> potion till time `t = 60`.
///
/// **Example 2:**
///
/// **Input:** skill = [1,1,1], mana = [1,1,1]
///
/// **Output:** 5
///
/// **Explanation:**
///
/// 1. Preparation of the 0<sup>th</sup> potion begins at time `t = 0`, and is completed by time `t = 3`.
/// 2. Preparation of the 1<sup>st</sup> potion begins at time `t = 1`, and is completed by time `t = 4`.
/// 3. Preparation of the 2<sup>nd</sup> potion begins at time `t = 2`, and is completed by time `t = 5`.
///
/// **Example 3:**
///
/// **Input:** skill = [1,2,3,4], mana = [1,2]
///
/// **Output:** 21
///
/// **Constraints:**
///
/// * `n == skill.length`
/// * `m == mana.length`
/// * `1 <= n, m <= 5000`
/// * `1 <= mana[i], skill[i] <= 5000`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-minimum-amount-of-time-to-brew-potions/
// discuss: https://leetcode.com/problems/find-the-minimum-amount-of-time-to-brew-potions/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        let n = skill.len();
        let m = mana.len();

        let mut skill_prefix = Vec::with_capacity(n);
        let mut sum = 0;

        for s in skill {
            sum += s as i64;
            skill_prefix.push(sum);
        }

        let mut offset = 0;
        let mut prev_m = 0;

        for &m in &mana {
            let m = m as i64;

            offset += (0..n - 1)
                .map(|i| skill_prefix[i + 1] * prev_m - skill_prefix[i] * m)
                .max()
                .unwrap_or(0)
                .max(skill_prefix[0] * prev_m);

            prev_m = m;
        }

        skill_prefix[n - 1] * mana[m - 1] as i64 + offset
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3494() {
        let skill = vec![1, 5, 2, 4];
        let mana = vec![5, 1, 4, 2];
        let expected = 110;
        assert_eq!(Solution::min_time(skill, mana), expected);
        let skill = vec![1, 1, 1];
        let mana = vec![1, 1, 1];
        let expected = 5;
        assert_eq!(Solution::min_time(skill, mana), expected);
        let skill = vec![1, 2, 3, 4];
        let mana = vec![1, 2];
        let expected = 21;
        assert_eq!(Solution::min_time(skill, mana), expected);
    }
}
