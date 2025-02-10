///
/// # 68. Text Justification
///
/// Given an array of strings `words` and a width `maxWidth`, format the text such that each line has exactly `maxWidth` characters and is fully (left and right) justified.
///
/// You should pack your words in a greedy approach; that is, pack as many words as you can in each line. Pad extra spaces `' '` when necessary so that each line has exactly `maxWidth` characters.
///
/// Extra spaces between words should be distributed as evenly as possible. If the number of spaces on a line does not divide evenly between words, the empty slots on the left will be assigned more spaces than the slots on the right.
///
/// For the last line of text, it should be left-justified, and no extra space is inserted between words.
///
/// **Note:**
///
/// * A word is defined as a character sequence consisting of non-space characters only.
/// * Each word's length is guaranteed to be greater than `0` and not exceed `maxWidth`.
/// * The input array `words` contains at least one word.
///
/// **Example 1:**
///
/// ```
/// Input: words = ["This", "is", "an", "example", "of", "text", "justification."], maxWidth = 16
/// Output:
/// [
///    "This    is    an",
///    "example  of text",
///    "justification.  "
/// ]
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: words = ["What","must","be","acknowledgment","shall","be"], maxWidth = 16
/// Output:
/// [
///   "What   must   be",
///   "acknowledgment  ",
///   "shall be        "
/// ]
/// Explanation: Note that the last line is "shall be    " instead of "shall     be", because the last line must be left-justified instead of fully-justified.
/// Note that the second line is also left-justified because it contains only one word.
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: words = ["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"], maxWidth = 20
/// Output:
/// [
///   "Science  is  what we",
///   "understand      well",
///   "enough to explain to",
///   "a  computer.  Art is",
///   "everything  else  we",
///   "do                  "
/// ]
/// ```
///
/// **Constraints:**
///
/// * `1 <= words.length <= 300`
/// * `1 <= words[i].length <= 20`
/// * `words[i]` consists of only English letters and symbols.
/// * `1 <= maxWidth <= 100`
/// * `words[i].length <= maxWidth`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/text-justification/
// discuss: https://leetcode.com/problems/text-justification/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;

        let mut result = vec![];

        let mut cur = vec![];
        let mut cur_width = 0;

        for word in words {
            if cur_width + word.len() > max_width {
                let mut remain = max_width - cur.iter().map(|x: &String| x.len()).sum::<usize>();
                if cur.len() == 1 {
                    result.push(cur[0].clone() + &" ".repeat(remain));
                } else {
                    let base_gap = remain / (cur.len() - 1);
                    remain -= base_gap * (cur.len() - 1);

                    let mut cur_string = String::new();

                    for x in cur {
                        cur_string += &x;
                        cur_string += &" ".repeat(base_gap + if remain > 0 { 1 } else { 0 });
                        remain = remain.saturating_sub(1);
                    }

                    result.push(cur_string.trim().to_owned());
                }

                cur = vec![];
                cur_width = 0;
            }

            cur_width += word.len() + 1;
            cur.push(word);
        }

        if !cur.is_empty() {
            let mut last = cur.join(" ");
            last += &" ".repeat(max_width - last.len());
            result.push(last);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_68() {
        let words = vec_string![
            "This",
            "is",
            "an",
            "example",
            "of",
            "text",
            "justification."
        ];
        let max_width = 16;
        let expected = vec_string!["This    is    an", "example  of text", "justification.  "];
        assert_eq!(Solution::full_justify(words, max_width), expected);
        let words = vec_string!["What", "must", "be", "acknowledgment", "shall", "be"];
        let max_width = 16;
        let expected = vec_string!["What   must   be", "acknowledgment  ", "shall be        "];
        assert_eq!(Solution::full_justify(words, max_width), expected);
        let words = vec_string![
            "Science",
            "is",
            "what",
            "we",
            "understand",
            "well",
            "enough",
            "to",
            "explain",
            "to",
            "a",
            "computer.",
            "Art",
            "is",
            "everything",
            "else",
            "we",
            "do"
        ];
        let max_width = 20;
        let expected = vec_string![
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  "
        ];
        assert_eq!(Solution::full_justify(words, max_width), expected);
    }
}
