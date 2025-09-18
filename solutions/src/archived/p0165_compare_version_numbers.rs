///
/// # 165. Compare Version Numbers
///
/// Given two **version strings**, `version1` and `version2`, compare them. A version string consists of **revisions** separated by dots `'.'`. The **value of the revision** is its **integer conversion** ignoring leading zeros.
///
/// To compare version strings, compare their revision values in **left-to-right order**. If one of the version strings has fewer revisions, treat the missing revision values as `0`.
///
/// Return the following:
///
/// * If `version1 < version2`, return -1.
/// * If `version1 > version2`, return 1.
/// * Otherwise, return 0.
///
/// **Example 1:**
///
/// **Input:** version1 = "1.2", version2 = "1.10"
///
/// **Output:** -1
///
/// **Explanation:**
///
/// version1's second revision is "2" and version2's second revision is "10": 2 \< 10, so version1 \< version2.
///
/// **Example 2:**
///
/// **Input:** version1 = "1.01", version2 = "1.001"
///
/// **Output:** 0
///
/// **Explanation:**
///
/// Ignoring leading zeroes, both "01" and "001" represent the same integer "1".
///
/// **Example 3:**
///
/// **Input:** version1 = "1.0", version2 = "1.0.0.0"
///
/// **Output:** 0
///
/// **Explanation:**
///
/// version1 has less revisions, which means every missing revision are treated as "0".
///
/// **Constraints:**
///
/// * `1 <= version1.length, version2.length <= 500`
/// * `version1` and `version2` only contain digits and `'.'`.
/// * `version1` and `version2` **are valid version numbers**.
/// * All the given revisions in `version1` and `version2` can be stored in a **32-bit integer**.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/compare-version-numbers/
// discuss: https://leetcode.com/problems/compare-version-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        for x in version1
            .split('.')
            .map(|x| x.parse::<i32>().unwrap())
            .zip_longest(version2.split('.').map(|x| x.parse::<i32>().unwrap()))
        {
            match x {
                itertools::EitherOrBoth::Both(v1, v2) if v1 > v2 => return 1,
                itertools::EitherOrBoth::Both(v1, v2) if v1 < v2 => return -1,
                itertools::EitherOrBoth::Left(v1) if v1 > 0 => return 1,
                itertools::EitherOrBoth::Right(v2) if v2 > 0 => return -1,
                _ => (),
            }
        }

        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_165() {
        let version1 = "1.2".to_owned();
        let version2 = "1.10".to_owned();
        let expected = -1;
        assert_eq!(Solution::compare_version(version1, version2), expected);
        let version1 = "1.01".to_owned();
        let version2 = "1.001".to_owned();
        let expected = 0;
        assert_eq!(Solution::compare_version(version1, version2), expected);
        let version1 = "1.0".to_owned();
        let version2 = "1.0.0.0".to_owned();
        let expected = 0;
        assert_eq!(Solution::compare_version(version1, version2), expected);
    }
}
