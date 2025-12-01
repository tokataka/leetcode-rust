///
/// # 3723. Maximize Sum of Squares of Digits
///
/// You are given two **positive** integers `num` and `sum`.
///
/// A positive integer `n` is **good** if it satisfies both of the following:
///
/// * The number of digits in `n` is **exactly** `num`.
/// * The sum of digits in `n` is **exactly** `sum`.
///
/// The **score** of a **good** integer `n` is the sum of the squares of digits in `n`.
///
/// Return a **string** denoting the **good** integer `n` that achieves the **maximum** **score**. If there are multiple possible integers, return the **maximum** ​​​​​​​one. If no such integer exists, return an empty string.
///
/// **Example 1:**
///
/// **Input:** num = 2, sum = 3
///
/// **Output:** "30"
///
/// **Explanation:**
///
/// There are 3 good integers: 12, 21, and 30.
///
/// * The score of 12 is `1<sup>2</sup> + 2<sup>2</sup> = 5`.
/// * The score of 21 is `2<sup>2</sup> + 1<sup>2</sup> = 5`.
/// * The score of 30 is `3<sup>2</sup> + 0<sup>2</sup> = 9`.
///
/// The maximum score is 9, which is achieved by the good integer 30. Therefore, the answer is `"30"`.
///
/// **Example 2:**
///
/// **Input:** num = 2, sum = 17
///
/// **Output:** "98"
///
/// **Explanation:**
///
/// There are 2 good integers: 89 and 98.
///
/// * The score of 89 is `8<sup>2</sup> + 9<sup>2</sup> = 145`.
/// * The score of 98 is `9<sup>2</sup> + 8<sup>2</sup> = 145`.
///
/// The maximum score is 145. The maximum good integer that achieves this score is 98. Therefore, the answer is `"98"`.
///
/// **Example 3:**
///
/// **Input:** num = 1, sum = 10
///
/// **Output:** ""
///
/// **Explanation:**
///
/// There are no integers that have exactly 1 digit and whose digits sum to 10. Therefore, the answer is `""`.
///
/// **Constraints:**
///
/// * `1 <= num <= 2 * 10<sup>5</sup>`
/// * `1 <= sum <= 2 * 10<sup>6</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximize-sum-of-squares-of-digits/
// discuss: https://leetcode.com/problems/maximize-sum-of-squares-of-digits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        match sum.cmp(&(9 * num)) {
            std::cmp::Ordering::Greater => String::new(),
            std::cmp::Ordering::Equal => "9".repeat((sum / 9) as usize),
            std::cmp::Ordering::Less => format!(
                "{}{}{}",
                "9".repeat((sum / 9) as usize),
                sum % 9,
                "0".repeat((num - sum / 9 - 1) as usize)
            ),
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3723() {
        let num = 2;
        let sum = 3;
        let expected = "30".to_owned();
        assert_eq!(Solution::max_sum_of_squares(num, sum), expected);
        let num = 2;
        let sum = 17;
        let expected = "98".to_owned();
        assert_eq!(Solution::max_sum_of_squares(num, sum), expected);
        let num = 1;
        let sum = 10;
        let expected = "".to_owned();
        assert_eq!(Solution::max_sum_of_squares(num, sum), expected);
    }
}
