use std::collections::HashMap;

///
/// # 87. Scramble String
///
/// We can scramble a string s to get a string t using the following algorithm:
///
/// 1. If the length of the string is 1, stop.
/// 2. If the length of the string is \> 1, do the following:
///    * Split the string into two non-empty substrings at a random index, i.e., if the string is `s`, divide it to `x` and `y` where `s = x + y`.
///    * **Randomly** decide to swap the two substrings or to keep them in the same order. i.e., after this step, `s` may become `s = x + y` or `s = y + x`.
///    * Apply step 1 recursively on each of the two substrings `x` and `y`.
///
/// Given two strings `s1` and `s2` of **the same length**, return `true` if `s2` is a scrambled string of `s1`, otherwise, return `false`.
///
/// **Example 1:**
///
/// ```
/// Input: s1 = "great", s2 = "rgeat"
/// Output: true
/// Explanation: One possible scenario applied on s1 is:
/// "great" --> "gr/eat" // divide at random index.
/// "gr/eat" --> "gr/eat" // random decision is not to swap the two substrings and keep them in order.
/// "gr/eat" --> "g/r / e/at" // apply the same algorithm recursively on both substrings. divide at random index each of them.
/// "g/r / e/at" --> "r/g / e/at" // random decision was to swap the first substring and to keep the second substring in the same order.
/// "r/g / e/at" --> "r/g / e/ a/t" // again apply the algorithm recursively, divide "at" to "a/t".
/// "r/g / e/ a/t" --> "r/g / e/ a/t" // random decision is to keep both substrings in the same order.
/// The algorithm stops now, and the result string is "rgeat" which is s2.
/// As one possible scenario led s1 to be scrambled to s2, we return true.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s1 = "abcde", s2 = "caebd"
/// Output: false
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: s1 = "a", s2 = "a"
/// Output: true
///
/// ```
///
/// **Constraints:**
///
/// * `s1.length == s2.length`
/// * `1 <= s1.length <= 30`
/// * `s1` and `s2` consist of lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/scramble-string/
// discuss: https://leetcode.com/problems/scramble-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let mut cache = HashMap::new();

        fn _is_scramble(
            cache: &mut HashMap<(String, String), bool>,
            s1: String,
            s2: String,
        ) -> bool {
            let (s1, s2) = (s1.clone().min(s2.clone()), s1.max(s2));

            if let Some(&x) = cache.get(&(s1.clone(), s2.clone())) {
                return x;
            }

            if s1 == s2 {
                return true;
            }

            if s1.len() == 1 && s1 != s2 {
                return false;
            }

            let mut candidates = vec![];

            let mut s1_count = vec![0; 26];
            let mut s2_count = vec![0; 26];

            for (i, (c1, c2)) in s1.chars().zip(s2.chars()).enumerate().take(s1.len() - 1) {
                s1_count[c1 as usize - 'a' as usize] += 1;
                s2_count[c2 as usize - 'a' as usize] += 1;

                if s1_count == s2_count {
                    candidates.push((
                        (s1[..(i + 1)].to_string(), s2[..(i + 1)].to_string()),
                        (s1[(i + 1)..].to_string(), s2[(i + 1)..].to_string()),
                    ));
                }
            }

            let mut s1_count = vec![0; 26];
            let mut s2_count = vec![0; 26];

            for (i, (c1, c2)) in s1
                .chars()
                .zip(s2.chars().rev())
                .enumerate()
                .take(s1.len() - 1)
            {
                s1_count[c1 as usize - 'a' as usize] += 1;
                s2_count[c2 as usize - 'a' as usize] += 1;

                if s1_count == s2_count {
                    candidates.push((
                        (
                            s1[..(i + 1)].to_string(),
                            s2[(s1.len() - i - 1)..].to_string(),
                        ),
                        (
                            s1[(i + 1)..].to_string(),
                            s2[..(s1.len() - i - 1)].to_string(),
                        ),
                    ));
                }
            }

            let result =
                candidates
                    .into_iter()
                    .any(|((left_s1, left_s2), (right_s1, right_s2))| {
                        _is_scramble(cache, left_s1, left_s2)
                            && _is_scramble(cache, right_s1, right_s2)
                    });

            cache.insert((s1, s2), result);

            result
        }

        _is_scramble(&mut cache, s1, s2)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_87() {
        // let s1 = "eatgr".to_string();
        // let s2 = "rgeat".to_string();
        // let expected = true;
        // assert_eq!(Solution::is_scramble(s1, s2), expected);
        // let s1 = "abcde".to_string();
        // let s2 = "caebd".to_string();
        // let expected = false;
        // assert_eq!(Solution::is_scramble(s1, s2), expected);
        // let s1 = "a".to_string();
        // let s2 = "a".to_string();
        // let expected = true;
        // assert_eq!(Solution::is_scramble(s1, s2), expected);
        let s1 = "abcdd".to_string();
        let s2 = "dbdac".to_string();
        let expected = false;
        assert_eq!(Solution::is_scramble(s1, s2), expected);
    }
}
