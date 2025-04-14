///
/// # 97. Interleaving String
///
/// Given strings `s1`, `s2`, and `s3`, find whether `s3` is formed by an **interleaving** of `s1` and `s2`.
///
/// An **interleaving** of two strings `s` and `t` is a configuration where `s` and `t` are divided into `n` and `m` substrings respectively, such that:
///
/// * `s = s<sub>1</sub> + s<sub>2</sub> + ... + s<sub>n</sub>`
/// * `t = t<sub>1</sub> + t<sub>2</sub> + ... + t<sub>m</sub>`
/// * `|n - m| <= 1`
/// * The **interleaving** is `s<sub>1</sub> + t<sub>1</sub> + s<sub>2</sub> + t<sub>2</sub> + s<sub>3</sub> + t<sub>3</sub> + ...` or `t<sub>1</sub> + s<sub>1</sub> + t<sub>2</sub> + s<sub>2</sub> + t<sub>3</sub> + s<sub>3</sub> + ...`
///
/// **Note:** `a + b` is the concatenation of strings `a` and `b`.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2020/09/02/interleave.jpg)
///
/// ```
/// Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
/// Output: true
/// Explanation: One way to obtain s3 is:
/// Split s1 into s1 = "aa" + "bc" + "c", and s2 into s2 = "dbbc" + "a".
/// Interleaving the two splits, we get "aa" + "dbbc" + "bc" + "a" + "c" = "aadbbcbcac".
/// Since s3 can be obtained by interleaving s1 and s2, we return true.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
/// Output: false
/// Explanation: Notice how it is impossible to interleave s2 with any other string to obtain s3.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: s1 = "", s2 = "", s3 = ""
/// Output: true
///
/// ```
///
/// **Constraints:**
///
/// * `0 <= s1.length, s2.length <= 100`
/// * `0 <= s3.length <= 200`
/// * `s1`, `s2`, and `s3` consist of lowercase English letters.
///
/// **Follow up:** Could you solve it using only `O(s2.length)` additional memory space?
///
pub struct Solution {}

// problem: https://leetcode.com/problems/interleaving-string/
// discuss: https://leetcode.com/problems/interleaving-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        let mut visited = vec![vec![false; s2.len() + 1]; s1.len() + 1];

        let mut st = vec![(0, 0)];

        while let Some((s1_idx, s2_idx)) = st.pop() {
            if visited[s1_idx][s2_idx] {
                continue;
            }

            visited[s1_idx][s2_idx] = true;

            if s1_idx == s1.len() && s2_idx == s2.len() {
                return true;
            }

            if s1.as_bytes().get(s1_idx) == s3.as_bytes().get(s1_idx + s2_idx) {
                st.push((s1_idx + 1, s2_idx));
            }

            if s2.as_bytes().get(s2_idx) == s3.as_bytes().get(s1_idx + s2_idx) {
                st.push((s1_idx, s2_idx + 1));
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_97() {
        let s1 = "aabcc".to_owned();
        let s2 = "dbbca".to_owned();
        let s3 = "aadbbcbcac".to_owned();
        let expected = true;
        assert_eq!(Solution::is_interleave(s1, s2, s3), expected);
        let s1 = "aabcc".to_owned();
        let s2 = "dbbca".to_owned();
        let s3 = "aadbbbaccc".to_owned();
        let expected = false;
        assert_eq!(Solution::is_interleave(s1, s2, s3), expected);
        let s1 = "".to_owned();
        let s2 = "".to_owned();
        let s3 = "".to_owned();
        let expected = true;
        assert_eq!(Solution::is_interleave(s1, s2, s3), expected);
    }
}
