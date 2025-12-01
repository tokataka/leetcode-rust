///
/// # 3234. Count the Number of Substrings With Dominant Ones
///
/// You are given a binary string `s`.
///
/// Return the number of substrings with **dominant** ones.
///
/// A string has **dominant** ones if the number of ones in the string is **greater than or equal to** the **square** of the number of zeros in the string.
///
/// **Example 1:**
///
/// **Input:** s = "00011"
///
/// **Output:** 5
///
/// **Explanation:**
///
/// The substrings with dominant ones are shown in the table below.
///
/// | i | j |s[i..j]|Number of Zeros|Number of Ones|
/// |---|---|-------|---------------|--------------|
/// | 3 | 3 |   1   |       0       |      1       |
/// | 4 | 4 |   1   |       0       |      1       |
/// | 2 | 3 |  01   |       1       |      1       |
/// | 3 | 4 |  11   |       0       |      2       |
/// | 2 | 4 |  011  |       1       |      2       |
///
/// **Example 2:**
///
/// **Input:** s = "101101"
///
/// **Output:** 16
///
/// **Explanation:**
///
/// The substrings with **non-dominant** ones are shown in the table below.
///
/// Since there are 21 substrings total and 5 of them have non-dominant ones, it follows that there are 16 substrings with dominant ones.
///
/// | i | j |s[i..j]|Number of Zeros|Number of Ones|
/// |---|---|-------|---------------|--------------|
/// | 1 | 1 |   0   |       1       |      0       |
/// | 4 | 4 |   0   |       1       |      0       |
/// | 1 | 4 | 0110  |       2       |      2       |
/// | 0 | 4 | 10110 |       2       |      3       |
/// | 1 | 5 | 01101 |       2       |      3       |
///
/// **Constraints:**
///
/// * `1 <= s.length <= 4 * 10<sup>4</sup>`
/// * `s` consists only of characters `'0'` and `'1'`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-the-number-of-substrings-with-dominant-ones/
// discuss: https://leetcode.com/problems/count-the-number-of-substrings-with-dominant-ones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.as_bytes();

        let mut zero_idxs = vec![];
        let mut one_idxs = vec![];

        for (i, &ch) in s.iter().enumerate() {
            if ch == b'0' {
                zero_idxs.push(i);
            } else {
                one_idxs.push(i);
            }
        }

        zero_idxs.push(s.len());

        let mut result = 0;
        let mut cur_zero_idx = 0;
        let mut cur_one_idx = 0;

        for left in 0..s.len() {
            let mut cur = left;

            for (zero_count, &next_zero) in zero_idxs
                .iter()
                .skip(cur_zero_idx)
                .enumerate()
                .take((one_idxs.len() - cur_one_idx).isqrt() + 1)
            {
                let one_count = (cur_one_idx + zero_count * zero_count).saturating_sub(1);

                if one_count < one_idxs.len() {
                    result += next_zero - one_idxs[one_count].min(next_zero).max(cur);
                }

                cur = next_zero;
            }

            if s[left] == b'0' {
                cur_zero_idx += 1;
            } else {
                cur_one_idx += 1;
            }
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3234() {
        let s = "00011".to_owned();
        let expected = 5;
        assert_eq!(Solution::number_of_substrings(s), expected);
        let s = "101101".to_owned();
        let expected = 16;
        assert_eq!(Solution::number_of_substrings(s), expected);
    }
}
