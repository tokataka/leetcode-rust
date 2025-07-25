///
/// # 3304. Find the K-th Character in String Game I
///
/// Alice and Bob are playing a game. Initially, Alice has a string `word = "a"`.
///
/// You are given a **positive** integer `k`.
///
/// Now Bob will ask Alice to perform the following operation **forever**:
///
/// * Generate a new string by **changing** each character in `word` to its **next** character in the English alphabet, and **append** it to the *original* `word`.
///
/// For example, performing the operation on `"c"` generates `"cd"` and performing the operation on `"zb"` generates `"zbac"`.
///
/// Return the value of the `k<sup>th</sup>` character in `word`, after enough operations have been done for `word` to have **at least** `k` characters.
///
/// **Note** that the character `'z'` can be changed to `'a'` in the operation.
///
/// **Example 1:**
///
/// **Input:** k = 5
///
/// **Output:** "b"
///
/// **Explanation:**
///
/// Initially, `word = "a"`. We need to do the operation three times:
///
/// * Generated string is `"b"`, `word` becomes `"ab"`.
/// * Generated string is `"bc"`, `word` becomes `"abbc"`.
/// * Generated string is `"bccd"`, `word` becomes `"abbcbccd"`.
///
/// **Example 2:**
///
/// **Input:** k = 10
///
/// **Output:** "c"
///
/// **Constraints:**
///
/// * `1 <= k <= 500`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-k-th-character-in-string-game-i/
// discuss: https://leetcode.com/problems/find-the-k-th-character-in-string-game-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn kth_character(mut k: i32) -> char {
        let mut cur = 0;
        k -= 1;

        while k > 0 {
            if k & 1 == 1 {
                cur = (cur + 1) % 26;
            }

            k >>= 1;
        }

        (cur + b'a') as char
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3304() {
        let k = 5;
        let expected = 'b';
        assert_eq!(Solution::kth_character(k), expected);
        let k = 10;
        let expected = 'c';
        assert_eq!(Solution::kth_character(k), expected);
    }
}
