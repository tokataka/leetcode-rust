///
/// # 1488. Avoid Flood in The City
///
/// Your country has an infinite number of lakes. Initially, all the lakes are empty, but when it rains over the `n<sup>th</sup>` lake, the `n<sup>th</sup>` lake becomes full of water. If it rains over a lake that is **full of water**, there will be a **flood**. Your goal is to avoid floods in any lake.
///
/// Given an integer array `rains` where:
///
/// * `rains[i] > 0` means there will be rains over the `rains[i]` lake.
/// * `rains[i] == 0` means there are no rains this day and you can choose **one lake** this day and **dry it**.
///
/// Return *an array `ans`* where:
///
/// * `ans.length == rains.length`
/// * `ans[i] == -1` if `rains[i] > 0`.
/// * `ans[i]` is the lake you choose to dry in the `ith` day if `rains[i] == 0`.
///
/// If there are multiple valid answers return **any** of them. If it is impossible to avoid flood return **an empty array**.
///
/// Notice that if you chose to dry a full lake, it becomes empty, but if you chose to dry an empty lake, nothing changes.
///
/// **Example 1:**
///
/// ```
/// Input: rains = [1,2,3,4]
/// Output: [-1,-1,-1,-1]
/// Explanation: After the first day full lakes are [1]
/// After the second day full lakes are [1,2]
/// After the third day full lakes are [1,2,3]
/// After the fourth day full lakes are [1,2,3,4]
/// There's no day to dry any lake and there is no flood in any lake.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: rains = [1,2,0,0,2,1]
/// Output: [-1,-1,2,1,-1,-1]
/// Explanation: After the first day full lakes are [1]
/// After the second day full lakes are [1,2]
/// After the third day, we dry lake 2. Full lakes are [1]
/// After the fourth day, we dry lake 1. There is no full lakes.
/// After the fifth day, full lakes are [2].
/// After the sixth day, full lakes are [1,2].
/// It is easy that this scenario is flood-free. [-1,-1,1,2,-1,-1] is another acceptable scenario.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: rains = [1,2,0,1,2]
/// Output: []
/// Explanation: After the second day, full lakes are  [1,2]. We have to dry one lake in the third day.
/// After that, it will rain over lakes [1,2]. It's easy to prove that no matter which lake you choose to dry in the 3rd day, the other one will flood.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= rains.length <= 10<sup>5</sup>`
/// * `0 <= rains[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/avoid-flood-in-the-city/
// discuss: https://leetcode.com/problems/avoid-flood-in-the-city/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::{BTreeSet, HashMap};

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut dries = BTreeSet::new();
        let mut filled = HashMap::new();

        let mut result = rains
            .iter()
            .map(|&x| if x == 0 { 1 } else { -1 })
            .collect::<Vec<_>>();

        for (i, rain) in rains.into_iter().enumerate() {
            if rain == 0 {
                dries.insert(i);
            } else if let Some(prev_i) = filled.insert(rain, i) {
                if let Some(&dry) = dries.range(prev_i..).next() {
                    result[dry] = rain;
                    dries.remove(&dry);
                } else {
                    return vec![];
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1488() {
        let rains = vec![1, 2, 3, 4];
        let expected = vec![-1, -1, -1, -1];
        assert_eq!(Solution::avoid_flood(rains), expected);
        let rains = vec![1, 2, 0, 0, 2, 1];
        let expected = vec![-1, -1, 2, 1, -1, -1];
        assert_eq!(Solution::avoid_flood(rains), expected);
        let rains = vec![1, 2, 0, 1, 2];
        let expected = vec![];
        assert_eq!(Solution::avoid_flood(rains), expected);
    }
}
