///
/// # 3335. Total Characters in String After Transformations I
///
/// You are given a string `s` and an integer `t`, representing the number of **transformations** to perform. In one **transformation**, every character in `s` is replaced according to the following rules:
///
/// * If the character is `'z'`, replace it with the string `"ab"`.
/// * Otherwise, replace it with the **next** character in the alphabet. For example, `'a'` is replaced with `'b'`, `'b'` is replaced with `'c'`, and so on.
///
/// Return the **length** of the resulting string after **exactly** `t` transformations.
///
/// Since the answer may be very large, return it **modulo** `10<sup>9</sup> + 7`.
///
/// **Example 1:**
///
/// **Input:** s = "abcyy", t = 2
///
/// **Output:** 7
///
/// **Explanation:**
///
/// * **First Transformation (t = 1)**:
///   * `'a'` becomes `'b'`
///   * `'b'` becomes `'c'`
///   * `'c'` becomes `'d'`
///   * `'y'` becomes `'z'`
///   * `'y'` becomes `'z'`
///   * String after the first transformation: `"bcdzz"`
///
/// * **Second Transformation (t = 2)**:
///   * `'b'` becomes `'c'`
///   * `'c'` becomes `'d'`
///   * `'d'` becomes `'e'`
///   * `'z'` becomes `"ab"`
///   * `'z'` becomes `"ab"`
///   * String after the second transformation: `"cdeabab"`
///
/// * **Final Length of the string**: The string is `"cdeabab"`, which has 7 characters.
///
/// **Example 2:**
///
/// **Input:** s = "azbk", t = 1
///
/// **Output:** 5
///
/// **Explanation:**
///
/// * **First Transformation (t = 1)**:
///   * `'a'` becomes `'b'`
///   * `'z'` becomes `"ab"`
///   * `'b'` becomes `'c'`
///   * `'k'` becomes `'l'`
///   * String after the first transformation: `"babcl"`
///
/// * **Final Length of the string**: The string is `"babcl"`, which has 5 characters.
///
/// **Constraints:**
///
/// * `1 <= s.length <= 10<sup>5</sup>`
/// * `s` consists only of lowercase English letters.
/// * `1 <= t <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/total-characters-in-string-after-transformations-i/
// discuss: https://leetcode.com/problems/total-characters-in-string-after-transformations-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;

        let mut freq = [0; 26];
        for b in s.bytes() {
            freq[(b - b'a') as usize] += 1;
        }

        // for offset in (0..26).cycle().take(t as usize) {
        //     let z_idx = 25 - offset;
        //     let a_idx = (26 - offset) % 26;
        //     freq[a_idx] = (freq[a_idx] + freq[z_idx]) % MOD;
        // }

        for _ in 0..t / 26 {
            for offset in 0..26 {
                let z_idx = 25 - offset;
                let a_idx = (26 - offset) % 26;
                freq[a_idx] = (freq[a_idx] + freq[z_idx]) % MOD;
            }
        }

        for offset in 0..t as usize % 26 {
            let z_idx = 25 - offset;
            let a_idx = (26 - offset) % 26;
            freq[a_idx] = (freq[a_idx] + freq[z_idx]) % MOD;
        }

        freq.iter().fold(0, |acc, &x| (acc + x) % MOD)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3335() {
        let s = "abcyy".to_owned();
        let t = 2;
        let expected = 7;
        assert_eq!(Solution::length_after_transformations(s, t), expected);
        let s = "azbk".to_owned();
        let t = 1;
        let expected = 5;
        assert_eq!(Solution::length_after_transformations(s, t), expected);
    }
}
