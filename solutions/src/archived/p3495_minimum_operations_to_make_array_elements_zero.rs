///
/// # 3495. Minimum Operations to Make Array Elements Zero
///
/// You are given a 2D array `queries`, where `queries[i]` is of the form `[l, r]`. Each `queries[i]` defines an array of integers `nums` consisting of elements ranging from `l` to `r`, both **inclusive**.
///
/// In one operation, you can:
///
/// * Select two integers `a` and `b` from the array.
/// * Replace them with `floor(a / 4)` and `floor(b / 4)`.
///
/// Your task is to determine the **minimum** number of operations required to reduce all elements of the array to zero for each query. Return the sum of the results for all queries.
///
/// **Example 1:**
///
/// **Input:** queries = [[1,2],[2,4]]
///
/// **Output:** 3
///
/// **Explanation:**
///
/// For `queries[0]`:
///
/// * The initial array is `nums = [1, 2]`.
/// * In the first operation, select `nums[0]` and `nums[1]`. The array becomes `[0, 0]`.
/// * The minimum number of operations required is 1.
///
/// For `queries[1]`:
///
/// * The initial array is `nums = [2, 3, 4]`.
/// * In the first operation, select `nums[0]` and `nums[2]`. The array becomes `[0, 3, 1]`.
/// * In the second operation, select `nums[1]` and `nums[2]`. The array becomes `[0, 0, 0]`.
/// * The minimum number of operations required is 2.
///
/// The output is `1 + 2 = 3`.
///
/// **Example 2:**
///
/// **Input:** queries = [[2,6]]
///
/// **Output:** 4
///
/// **Explanation:**
///
/// For `queries[0]`:
///
/// * The initial array is `nums = [2, 3, 4, 5, 6]`.
/// * In the first operation, select `nums[0]` and `nums[3]`. The array becomes `[0, 3, 4, 1, 6]`.
/// * In the second operation, select `nums[2]` and `nums[4]`. The array becomes `[0, 3, 1, 1, 1]`.
/// * In the third operation, select `nums[1]` and `nums[2]`. The array becomes `[0, 0, 0, 1, 1]`.
/// * In the fourth operation, select `nums[3]` and `nums[4]`. The array becomes `[0, 0, 0, 0, 0]`.
/// * The minimum number of operations required is 4.
///
/// The output is 4.
///
/// **Constraints:**
///
/// * `1 <= queries.length <= 10<sup>5</sup>`
/// * `queries[i].length == 2`
/// * `queries[i] == [l, r]`
/// * `1 <= l < r <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-operations-to-make-array-elements-zero/
// discuss: https://leetcode.com/problems/minimum-operations-to-make-array-elements-zero/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        let mut result = 0;

        for query in queries {
            let (l, r) = (query[0] as i64, query[1] as i64);

            let mut cur = 0;

            for i in 1..=16 {
                let right = ((1 << (2 * i)) - 1).min(r);

                if right < l {
                    continue;
                }

                let left = (1 << (2 * (i - 1))).max(l);

                if left > r {
                    break;
                }

                cur += i * (right - left + 1);
            }

            result += (cur + 1) / 2;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3495() {
        let queries = nd_vec![[1, 2], [2, 4]];
        let expected = 3;
        assert_eq!(Solution::min_operations(queries), expected);
        let queries = nd_vec![[2, 6]];
        let expected = 4;
        assert_eq!(Solution::min_operations(queries), expected);
    }
}
