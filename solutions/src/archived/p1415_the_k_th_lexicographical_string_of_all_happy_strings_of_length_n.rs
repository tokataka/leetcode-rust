///
/// # 1415. The k-th Lexicographical String of All Happy Strings of Length n
///
/// A **happy string** is a string that:
///
/// * consists only of letters of the set `['a', 'b', 'c']`.
/// * `s[i] != s[i + 1]` for all values of `i` from `1` to `s.length - 1` (string is 1-indexed).
///
/// For example, strings **"abc", "ac", "b"** and **"abcbabcbcb"** are all happy strings and strings **"aa", "baa"** and **"ababbc"** are not happy strings.
///
/// Given two integers `n` and `k`, consider a list of all happy strings of length `n` sorted in lexicographical order.
///
/// Return *the kth string* of this list or return an **empty string** if there are less than `k` happy strings of length `n`.
///
/// **Example 1:**
///
/// ```
/// Input: n = 1, k = 3
/// Output: "c"
/// Explanation: The list ["a", "b", "c"] contains all happy strings of length 1. The third string is "c".
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 1, k = 4
/// Output: ""
/// Explanation: There are only 3 happy strings of length 1.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: n = 3, k = 9
/// Output: "cab"
/// Explanation: There are 12 different happy string of length 3 ["aba", "abc", "aca", "acb", "bab", "bac", "bca", "bcb", "cab", "cac", "cba", "cbc"]. You will find the 9th string = "cab"
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 10`
/// * `1 <= k <= 100`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/
// discuss: https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        const CHARS: [char; 3] = ['a', 'b', 'c'];

        fn backtrack(
            cur: &mut Vec<char>,
            count: &mut i32,
            last: char,
            n: i32,
            k: i32,
        ) -> Option<String> {
            if cur.len() == n as usize {
                *count += 1;

                return (k == *count).then_some(cur.iter().collect());
            }

            for ch in CHARS {
                if ch == last {
                    continue;
                }

                cur.push(ch);

                if let Some(x) = backtrack(cur, count, ch, n, k) {
                    return Some(x);
                }

                cur.pop();
            }

            None
        }

        backtrack(&mut vec![], &mut 0, ' ', n, k).unwrap_or_default()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1415() {
        let n = 1;
        let k = 3;
        let expected = "c".to_owned();
        assert_eq!(Solution::get_happy_string(n, k), expected);
        let n = 1;
        let k = 4;
        let expected = "".to_owned();
        assert_eq!(Solution::get_happy_string(n, k), expected);
        let n = 3;
        let k = 9;
        let expected = "cab".to_owned();
        assert_eq!(Solution::get_happy_string(n, k), expected);
    }
}
