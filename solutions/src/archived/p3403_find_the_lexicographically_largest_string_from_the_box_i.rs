///
/// # 3403. Find the Lexicographically Largest String From the Box I
///
/// You are given a string `word`, and an integer `numFriends`.
///
/// Alice is organizing a game for her `numFriends` friends. There are multiple rounds in the game, where in each round:
///
/// * `word` is split into `numFriends` **non-empty** strings, such that no previous round has had the **exact** same split.
/// * All the split words are put into a box.
///
/// Find the lexicographically largest string from the box after all the rounds are finished.
///
/// **Example 1:**
///
/// **Input:** word = "dbca", numFriends = 2
///
/// **Output:** "dbc"
///
/// **Explanation:**
///
/// All possible splits are:
///
/// * `"d"` and `"bca"`.
/// * `"db"` and `"ca"`.
/// * `"dbc"` and `"a"`.
///
/// **Example 2:**
///
/// **Input:** word = "gggg", numFriends = 4
///
/// **Output:** "g"
///
/// **Explanation:**
///
/// The only possible split is: `"g"`, `"g"`, `"g"`, and `"g"`.
///
/// **Constraints:**
///
/// * `1 <= word.length <= 5 * 10<sup>3</sup>`
/// * `word` consists only of lowercase English letters.
/// * `1 <= numFriends <= word.length`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-lexicographically-largest-string-from-the-box-i/
// discuss: https://leetcode.com/problems/find-the-lexicographically-largest-string-from-the-box-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        if num_friends == 1 {
            return word;
        }

        let word = word.as_bytes();
        let max_string_len = word.len() + 1 - num_friends as usize;

        let max_string = (0..word.len())
            .map(|idx| &word[idx..(idx + max_string_len)])
            .max()
            .unwrap();

        String::from_utf8_lossy(max_string).into_owned()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3403() {
        let word = "dbca".to_owned();
        let num_friends = 2;
        let expected = "dbc".to_owned();
        assert_eq!(Solution::answer_string(word, num_friends), expected);
        let word = "gggg".to_owned();
        let num_friends = 4;
        let expected = "g".to_owned();
        assert_eq!(Solution::answer_string(word, num_friends), expected);
        let word = "gh".to_owned();
        let num_friends = 1;
        let expected = "gh".to_owned();
        assert_eq!(Solution::answer_string(word, num_friends), expected);
    }
}
