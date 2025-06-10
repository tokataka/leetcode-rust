///
/// # 440. K-th Smallest in Lexicographical Order
///
/// Given two integers `n` and `k`, return *the* `k<sup>th</sup>` *lexicographically smallest integer in the range* `[1, n]`.
///
/// **Example 1:**
///
/// ```
/// Input: n = 13, k = 2
/// Output: 10
/// Explanation: The lexicographical order is [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9], so the second smallest number is 10.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 1, k = 1
/// Output: 1
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= k <= n <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/k-th-smallest-in-lexicographical-order/
// discuss: https://leetcode.com/problems/k-th-smallest-in-lexicographical-order/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_kth_number(mut n: i32, k: i32) -> i32 {
        let mut n_digits = vec![];
        let mut cur_n = n as usize;

        while cur_n > 0 {
            n_digits.push(cur_n % 10);
            cur_n /= 10;
        }

        let mut remain = k;

        let mut cur = vec![];

        while remain > 0 {
            let n_digit = n_digits.pop().unwrap();

            let base_count = (10i32.pow(n_digits.len() as u32) - 1) / 9;

            let mut counts = [base_count; 10];

            for x in 0..=9 {
                match x.cmp(&n_digit) {
                    std::cmp::Ordering::Less => counts[x] += 10i32.pow(n_digits.len() as u32),
                    std::cmp::Ordering::Equal => {
                        counts[x] += n % 10i32.pow(n_digits.len() as u32) + 1
                    }
                    std::cmp::Ordering::Greater => (),
                }
            }

            // remove leading zeros
            if cur.is_empty() {
                counts[0] = 0;
            }

            for x in 0..=9 {
                if remain <= counts[x] {
                    cur.push(x);

                    match x.cmp(&n_digit) {
                        std::cmp::Ordering::Less => {
                            n_digits = vec![9; n_digits.len()];
                            n = 10i32.pow(n_digits.len() as u32 + 1) - 1;
                        }
                        std::cmp::Ordering::Equal => n %= 10i32.pow(n_digits.len() as u32),
                        std::cmp::Ordering::Greater => {
                            n_digits = vec![9; n_digits.len() - 1];
                            n = 10i32.pow(n_digits.len() as u32) - 1;
                        }
                    }

                    break;
                }

                remain -= counts[x];
            }

            remain -= 1;
        }

        let mut result = 0;

        for x in cur {
            result = result * 10 + x as i32;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_440() {
        // let n = 13;
        // let k = 2;
        // let expected = 10;
        // assert_eq!(Solution::find_kth_number(n, k), expected);
        // let n = 1;
        // let k = 1;
        // let expected = 1;
        // assert_eq!(Solution::find_kth_number(n, k), expected);
        let n = 4289384;
        let k = 1922239;
        let expected = 0;
        assert_eq!(Solution::find_kth_number(n, k), expected);
    }
}
