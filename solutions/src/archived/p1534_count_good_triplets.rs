///
/// # 1534. Count Good Triplets
///
/// Given an array of integers `arr`, and three integers `a`, `b` and `c`. You need to find the number of good triplets.
///
/// A triplet `(arr[i], arr[j], arr[k])` is **good** if the following conditions are true:
///
/// * `0 <= i < j < k < arr.length`
/// * `|arr[i] - arr[j]| <= a`
/// * `|arr[j] - arr[k]| <= b`
/// * `|arr[i] - arr[k]| <= c`
///
/// Where `|x|` denotes the absolute value of `x`.
///
/// Return *the number of good triplets*.
///
/// **Example 1:**
///
/// ```
/// Input: arr = [3,0,1,1,9,7], a = 7, b = 2, c = 3
/// Output: 4
/// Explanation: There are 4 good triplets: [(3,0,1), (3,0,1), (3,1,1), (0,1,1)].
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: arr = [1,1,2,2,3], a = 0, b = 0, c = 1
/// Output: 0
/// Explanation: No triplet satisfies all conditions.
///
/// ```
///
/// **Constraints:**
///
/// * `3 <= arr.length <= 100`
/// * `0 <= arr[i] <= 1000`
/// * `0 <= a, b, c <= 1000`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-good-triplets/
// discuss: https://leetcode.com/problems/count-good-triplets/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut result = 0;

        for (i, vi) in arr.iter().enumerate() {
            for (j, vj) in arr.iter().enumerate().skip(i + 1) {
                if (vi - vj).abs() > a {
                    continue;
                }

                for vk in arr.iter().skip(j + 1) {
                    if (vj - vk).abs() <= b && (vi - vk).abs() <= c {
                        result += 1;
                    }
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1534() {
        let arr = vec![3, 0, 1, 1, 9, 7];
        let a = 7;
        let b = 2;
        let c = 3;
        let expected = 4;
        assert_eq!(Solution::count_good_triplets(arr, a, b, c), expected);
        let arr = vec![1, 1, 2, 2, 3];
        let a = 0;
        let b = 0;
        let c = 1;
        let expected = 0;
        assert_eq!(Solution::count_good_triplets(arr, a, b, c), expected);
    }
}
