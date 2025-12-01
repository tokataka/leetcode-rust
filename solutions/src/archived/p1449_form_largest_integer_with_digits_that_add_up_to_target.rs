///
/// # 1449. Form Largest Integer With Digits That Add up to Target
///
/// Given an array of integers `cost` and an integer `target`, return *the **maximum** integer you can paint under the following rules*:
///
/// * The cost of painting a digit `(i + 1)` is given by `cost[i]` (**0-indexed**).
/// * The total cost used must be equal to `target`.
/// * The integer does not have `0` digits.
///
/// Since the answer may be very large, return it as a string. If there is no way to paint any integer given the condition, return `"0"`.
///
/// **Example 1:**
///
/// ```
/// Input: cost = [4,3,2,5,6,7,2,5,5], target = 9
/// Output: "7772"
/// Explanation: The cost to paint the digit '7' is 2, and the digit '2' is 3. Then cost("7772") = 2*3+ 3*1 = 9. You could also paint "977", but "7772" is the largest number.
/// Digit    cost
///   1  ->   4
///   2  ->   3
///   3  ->   2
///   4  ->   5
///   5  ->   6
///   6  ->   7
///   7  ->   2
///   8  ->   5
///   9  ->   5
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: cost = [7,6,5,5,5,6,8,7,8], target = 12
/// Output: "85"
/// Explanation: The cost to paint the digit '8' is 7, and the digit '5' is 5. Then cost("85") = 7 + 5 = 12.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: cost = [2,4,6,2,4,6,4,4,4], target = 5
/// Output: "0"
/// Explanation: It is impossible to paint any integer with total cost equal to target.
///
/// ```
///
/// **Constraints:**
///
/// * `cost.length == 9`
/// * `1 <= cost[i], target <= 5000`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/form-largest-integer-with-digits-that-add-up-to-target/
// discuss: https://leetcode.com/problems/form-largest-integer-with-digits-that-add-up-to-target/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_number(cost: Vec<i32>, target: i32) -> String {
        let target = target as usize;

        let mut dp = vec![-1; target + 1];
        dp[0] = 0;

        for i in 1..=target {
            for &c in &cost {
                let c = c as usize;
                if i >= c && dp[i - c] != -1 {
                    dp[i] = dp[i].max(dp[i - c] + 1);
                }
            }
        }

        if dp[target] == -1 {
            return "0".to_string();
        }

        let mut res = String::with_capacity(dp[target] as usize);
        let mut curr_target = target;

        while curr_target > 0 {
            for (idx, &c) in cost.iter().enumerate().rev() {
                let c = c as usize;
                if curr_target >= c
                    && dp[curr_target - c] != -1
                    && dp[curr_target] == dp[curr_target - c] + 1
                {
                    res.push(char::from_digit((idx + 1) as u32, 10).unwrap());
                    curr_target -= c;
                    break;
                }
            }
        }

        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1449() {
        assert_eq!(
            Solution::largest_number(vec![4, 3, 2, 5, 6, 7, 2, 5, 5], 9),
            "7772".to_owned()
        );
        assert_eq!(
            Solution::largest_number(vec![7, 6, 5, 5, 5, 6, 8, 7, 8], 12),
            "85".to_owned()
        );
        assert_eq!(
            Solution::largest_number(vec![2, 4, 6, 2, 4, 6, 4, 4, 4], 5),
            "0".to_owned()
        );
    }
}
