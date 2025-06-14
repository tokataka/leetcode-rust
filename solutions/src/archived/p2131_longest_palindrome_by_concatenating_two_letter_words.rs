///
/// # 2131. Longest Palindrome by Concatenating Two Letter Words
///
/// You are given an array of strings `words`. Each element of `words` consists of **two** lowercase English letters.
///
/// Create the **longest possible palindrome** by selecting some elements from `words` and concatenating them in **any order**. Each element can be selected **at most once**.
///
/// Return *the **length** of the longest palindrome that you can create*. If it is impossible to create any palindrome, return `0`.
///
/// A **palindrome** is a string that reads the same forward and backward.
///
/// **Example 1:**
///
/// ```
/// Input: words = ["lc","cl","gg"]
/// Output: 6
/// Explanation: One longest palindrome is "lc" + "gg" + "cl" = "lcggcl", of length 6.
/// Note that "clgglc" is another longest palindrome that can be created.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: words = ["ab","ty","yt","lc","cl","ab"]
/// Output: 8
/// Explanation: One longest palindrome is "ty" + "lc" + "cl" + "yt" = "tylcclyt", of length 8.
/// Note that "lcyttycl" is another longest palindrome that can be created.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: words = ["cc","ll","xx"]
/// Output: 2
/// Explanation: One longest palindrome is "cc", of length 2.
/// Note that "ll" is another longest palindrome that can be created, and so is "xx".
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= words.length <= 10<sup>5</sup>`
/// * `words[i].length == 2`
/// * `words[i]` consists of lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/
// discuss: https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut result = 0;
        let mut freq = vec![vec![0; 128]; 128];

        for word in words {
            let word = word.as_bytes();
            let (a, b) = (word[0] as usize, word[1] as usize);

            if freq[b][a] > 0 {
                freq[b][a] -= 1;
                result += 4;
            } else {
                freq[a][b] += 1;
            }
        }

        if (b'a'..=b'z').any(|x| freq[x as usize][x as usize] > 0) {
            result += 2;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2131() {
        let words = vec_string!["lc", "cl", "gg"];
        let expected = 6;
        assert_eq!(Solution::longest_palindrome(words), expected);
        let words = vec_string!["ab", "ty", "yt", "lc", "cl", "ab"];
        let expected = 8;
        assert_eq!(Solution::longest_palindrome(words), expected);
        let words = vec_string!["cc", "ll", "xx"];
        let expected = 2;
        assert_eq!(Solution::longest_palindrome(words), expected);
    }
}
