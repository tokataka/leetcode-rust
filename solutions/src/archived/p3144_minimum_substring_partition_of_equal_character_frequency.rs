///
/// # 3144. Minimum Substring Partition of Equal Character Frequency
///
/// Given a string `s`, you need to partition it into one or more **balanced** substrings. For example, if `s == "ababcc"` then `("abab", "c", "c")`, `("ab", "abc", "c")`, and `("ababcc")` are all valid partitions, but `("a", **"bab"**, "cc")`, `(**"aba"**, "bc", "c")`, and `("ab", **"abcc"**)` are not. The unbalanced substrings are bolded.
///
/// Return the **minimum** number of substrings that you can partition `s` into.
///
/// **Note:** A **balanced** string is a string where each character in the string occurs the same number of times.
///
/// **Example 1:**
///
/// **Input:** s = "fabccddg"
///
/// **Output:** 3
///
/// **Explanation:**
///
/// We can partition the string `s` into 3 substrings in one of the following ways: `("fab, "ccdd", "g")`, or `("fabc", "cd", "dg")`.
///
/// **Example 2:**
///
/// **Input:** s = "abababaccddb"
///
/// **Output:** 2
///
/// **Explanation:**
///
/// We can partition the string `s` into 2 substrings like so: `("abab", "abaccddb")`.
///
/// **Constraints:**
///
/// * `1 <= s.length <= 1000`
/// * `s` consists only of English lowercase letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-substring-partition-of-equal-character-frequency/
// discuss: https://leetcode.com/problems/minimum-substring-partition-of-equal-character-frequency/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_substrings_in_partition(s: String) -> i32 {
        fn _minimum_substrings_in_partition(cache: &mut [i32], s: &str, idx: usize) -> i32 {
            if idx >= s.len() {
                return 0;
            }

            if cache[idx] != -1 {
                return cache[idx];
            }

            let mut count = [0; 26];
            let mut result = 1001;

            for (i, ch) in s.bytes().enumerate().skip(idx) {
                let ch = ch as usize - b'a' as usize;
                count[ch] += 1;
                if count.iter().filter(|&&x| x > 0).all(|&x| x == count[ch]) {
                    result = result.min(1 + _minimum_substrings_in_partition(cache, s, i + 1));
                }
            }

            cache[idx] = result;

            result
        }

        _minimum_substrings_in_partition(&mut vec![-1; s.len()], &s, 0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3144() {
        let s = "fabccddg".to_owned();
        let expected = 3;
        assert_eq!(Solution::minimum_substrings_in_partition(s), expected);
        let s = "abababaccddb".to_owned();
        let expected = 2;
        assert_eq!(Solution::minimum_substrings_in_partition(s), expected);
    }
}
