use std::collections::HashMap;

///
/// # 3160. Find the Number of Distinct Colors Among the Balls
///
/// You are given an integer `limit` and a 2D array `queries` of size `n x 2`.
///
/// There are `limit + 1` balls with **distinct** labels in the range `[0, limit]`. Initially, all balls are uncolored. For every query in `queries` that is of the form `[x, y]`, you mark ball `x` with the color `y`. After each query, you need to find the number of **distinct** colors among the balls.
///
/// Return an array `result` of length `n`, where `result[i]` denotes the number of distinct colors *after* `i<sup>th</sup>` query.
///
/// **Note** that when answering a query, lack of a color *will not* be considered as a color.
///
/// **Example 1:**
///
/// **Input:** limit = 4, queries = [[1,4],[2,5],[1,3],[3,4]]
///
/// **Output:** [1,2,2,3]
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/04/17/ezgifcom-crop.gif)
///
/// * After query 0, ball 1 has color 4.
/// * After query 1, ball 1 has color 4, and ball 2 has color 5.
/// * After query 2, ball 1 has color 3, and ball 2 has color 5.
/// * After query 3, ball 1 has color 3, ball 2 has color 5, and ball 3 has color 4.
///
/// **Example 2:**
///
/// **Input:** limit = 4, queries = [[0,1],[1,2],[2,2],[3,4],[4,5]]
///
/// **Output:** [1,2,2,3,4]
///
/// **Explanation:**
///
/// **![](https://assets.leetcode.com/uploads/2024/04/17/ezgifcom-crop2.gif)**
///
/// * After query 0, ball 0 has color 1.
/// * After query 1, ball 0 has color 1, and ball 1 has color 2.
/// * After query 2, ball 0 has color 1, and balls 1 and 2 have color 2.
/// * After query 3, ball 0 has color 1, balls 1 and 2 have color 2, and ball 3 has color 4.
/// * After query 4, ball 0 has color 1, balls 1 and 2 have color 2, ball 3 has color 4, and ball 4 has color 5.
///
/// **Constraints:**
///
/// * `1 <= limit <= 10<sup>9</sup>`
/// * `1 <= n == queries.length <= 10<sup>5</sup>`
/// * `queries[i].length == 2`
/// * `0 <= queries[i][0] <= limit`
/// * `1 <= queries[i][1] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-number-of-distinct-colors-among-the-balls/
// discuss: https://leetcode.com/problems/find-the-number-of-distinct-colors-among-the-balls/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn query_results(_limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut colored = HashMap::new();
        let mut color_num = HashMap::new();

        let mut result = vec![];

        for query in queries {
            let (idx, color) = (query[0], query[1]);

            if let Some(prev_color) = colored.insert(idx, color) {
                if let Some(1) = color_num.get(&prev_color) {
                    color_num.remove(&prev_color);
                } else {
                    color_num.entry(prev_color).and_modify(|x| *x -= 1);
                }
            }

            color_num.entry(color).and_modify(|x| *x += 1).or_insert(1);

            result.push(color_num.len() as i32);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3160() {
        let limit = 4;
        let queries = nd_vec![[1, 4], [2, 5], [1, 3], [3, 4]];
        let expected = vec![1, 2, 2, 3];
        assert_eq!(Solution::query_results(limit, queries), expected);
        let limit = 4;
        let queries = nd_vec![[0, 1], [1, 2], [2, 2], [3, 4], [4, 5]];
        let expected = vec![1, 2, 2, 3, 4];
        assert_eq!(Solution::query_results(limit, queries), expected);
    }
}
