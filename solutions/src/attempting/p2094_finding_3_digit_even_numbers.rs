///
/// # 2094. Finding 3-Digit Even Numbers
///
/// You are given an integer array `digits`, where each element is a digit. The array may contain duplicates.
///
/// You need to find **all** the **unique** integers that follow the given requirements:
///
/// * The integer consists of the **concatenation** of **three** elements from `digits` in **any** arbitrary order.
/// * The integer does not have **leading zeros**.
/// * The integer is **even**.
///
/// For example, if the given `digits` were `[1, 2, 3]`, integers `132` and `312` follow the requirements.
///
/// Return *a **sorted** array of the unique integers.*
///
/// **Example 1:**
///
/// ```
/// Input: digits = [2,1,3,0]
/// Output: [102,120,130,132,210,230,302,310,312,320]
/// Explanation: All the possible integers that follow the requirements are in the output array.
/// Notice that there are no odd integers or integers with leading zeros.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: digits = [2,2,8,8,2]
/// Output: [222,228,282,288,822,828,882]
/// Explanation: The same digit can be used as many times as it appears in digits.
/// In this example, the digit 8 is used twice each time in 288, 828, and 882.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: digits = [3,7,5]
/// Output: []
/// Explanation: No even integers can be formed using the given digits.
///
/// ```
///
/// **Constraints:**
///
/// * `3 <= digits.length <= 100`
/// * `0 <= digits[i] <= 9`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/finding-3-digit-even-numbers/
// discuss: https://leetcode.com/problems/finding-3-digit-even-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let digit_count = digits.into_iter().fold([0; 10], |mut acc, x| {
            acc[x as usize] += 1;
            acc
        });

        let mut result = vec![];

        'outer: for x in (100..1000).step_by(2) {
            let mut cur_digit_count = [0; 10];
            let mut cur = x as usize;
            for _ in 0..3 {
                if digit_count[cur % 10] <= cur_digit_count[cur % 10] {
                    continue 'outer;
                }

                cur_digit_count[cur % 10] += 1;
                cur /= 10;
            }

            result.push(x);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2094() {
        let digits = vec![2, 1, 3, 0];
        let expected = vec![102, 120, 130, 132, 210, 230, 302, 310, 312, 320];
        assert_eq!(Solution::find_even_numbers(digits), expected);
        let digits = vec![2, 2, 8, 8, 2];
        let expected = vec![222, 228, 282, 288, 822, 828, 882];
        assert_eq!(Solution::find_even_numbers(digits), expected);
        let digits = vec![3, 7, 5];
        let expected = vec![];
        assert_eq!(Solution::find_even_numbers(digits), expected);
    }
}
