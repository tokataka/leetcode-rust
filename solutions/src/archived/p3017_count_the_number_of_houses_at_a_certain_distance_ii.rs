///
/// # 3017. Count the Number of Houses at a Certain Distance II
///
/// You are given three **positive** integers `n`, `x`, and `y`.
///
/// In a city, there exist houses numbered `1` to `n` connected by `n` streets. There is a street connecting the house numbered `i` with the house numbered `i + 1` for all `1 <= i <= n - 1` . An additional street connects the house numbered `x` with the house numbered `y`.
///
/// For each `k`, such that `1 <= k <= n`, you need to find the number of **pairs of houses** `(house<sub>1</sub>, house<sub>2</sub>)` such that the **minimum** number of streets that need to be traveled to reach `house<sub>2</sub>` from `house<sub>1</sub>` is `k`.
///
/// Return *a **1-indexed** array* `result` *of length* `n` *where* `result[k]` *represents the **total** number of pairs of houses such that the **minimum** streets required to reach one house from the other is* `k`.
///
/// **Note** that `x` and `y` can be **equal**.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2023/12/20/example2.png)
///
/// ```
/// Input: n = 3, x = 1, y = 3
/// Output: [6,0,0]
/// Explanation: Let's look at each pair of houses:
/// - For the pair (1, 2), we can go from house 1 to house 2 directly.
/// - For the pair (2, 1), we can go from house 2 to house 1 directly.
/// - For the pair (1, 3), we can go from house 1 to house 3 directly.
/// - For the pair (3, 1), we can go from house 3 to house 1 directly.
/// - For the pair (2, 3), we can go from house 2 to house 3 directly.
/// - For the pair (3, 2), we can go from house 3 to house 2 directly.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2023/12/20/example3.png)
///
/// ```
/// Input: n = 5, x = 2, y = 4
/// Output: [10,8,2,0,0]
/// Explanation: For each distance k the pairs are:
/// - For k == 1, the pairs are (1, 2), (2, 1), (2, 3), (3, 2), (2, 4), (4, 2), (3, 4), (4, 3), (4, 5), and (5, 4).
/// - For k == 2, the pairs are (1, 3), (3, 1), (1, 4), (4, 1), (2, 5), (5, 2), (3, 5), and (5, 3).
/// - For k == 3, the pairs are (1, 5), and (5, 1).
/// - For k == 4 and k == 5, there are no pairs.
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2023/12/20/example5.png)
///
/// ```
/// Input: n = 4, x = 1, y = 1
/// Output: [6,4,2,0]
/// Explanation: For each distance k the pairs are:
/// - For k == 1, the pairs are (1, 2), (2, 1), (2, 3), (3, 2), (3, 4), and (4, 3).
/// - For k == 2, the pairs are (1, 3), (3, 1), (2, 4), and (4, 2).
/// - For k == 3, the pairs are (1, 4), and (4, 1).
/// - For k == 4, there are no pairs.
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= n <= 10<sup>5</sup>`
/// * `1 <= x, y <= n`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-the-number-of-houses-at-a-certain-distance-ii/
// discuss: https://leetcode.com/problems/count-the-number-of-houses-at-a-certain-distance-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i64> {
        let n = n as usize;
        let (x, y) = (x.min(y) as usize - 1, x.max(y) as usize - 1);

        if y - x <= 1 {
            return (0..n).rev().map(|x| x as i64 * 2).collect();
        }

        let mut distance_count = vec![0; n + 1];

        // pairs with [i + 1, n - 1] on 0 -- x -- y -- (n - 1) line
        // it can be considered as straight line of n - y + x + 1 houses.
        for i in 0..n - y + x + 1 {
            distance_count[1] += 2;
            distance_count[n - y + x - i + 1] -= 2;
        }

        for i in x + 1..y {
            // pairs with [1, x]
            let path = (i - x).min(y - i + 1);
            distance_count[path] += 2;
            distance_count[path + x + 1] -= 2;

            // pairs with [y, n - 1]
            let path = (y - i).min(i - x + 1);
            distance_count[path] += 2;
            distance_count[path + n - y] -= 2;

            // pairs with [i + 1, y - 1]
            // calculate smallest j where i -- x -- y -- j is shorter than i -- k
            // => j - i >= (i - x) + 1 + (y - j)
            // => j >= i + (y - x + 1) / 2
            // => j = i + ceil((y - x + 1) / 2)
            // => j = i + (y - x) / 2 + 1
            let j = (i + (y - x) / 2 + 1).min(y);

            // pairs with [i + 1, j - 1]
            distance_count[1] += 2;
            distance_count[j - i] -= 2;

            // pairs with [j, y - 1]
            distance_count[i - x + 2] += 2;
            distance_count[i - x + 2 + y - j] -= 2;
        }

        distance_count
            .into_iter()
            .skip(1)
            .scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3017() {
        let n = 3;
        let x = 1;
        let y = 3;
        let expected = vec![6, 0, 0];
        assert_eq!(Solution::count_of_pairs(n, x, y), expected);
        let n = 5;
        let x = 2;
        let y = 4;
        let expected = vec![10, 8, 2, 0, 0];
        assert_eq!(Solution::count_of_pairs(n, x, y), expected);
        let n = 4;
        let x = 1;
        let y = 1;
        let expected = vec![6, 4, 2, 0];
        assert_eq!(Solution::count_of_pairs(n, x, y), expected);
        let n = 4;
        let x = 2;
        let y = 4;
        let expected = vec![8, 4, 0, 0];
        assert_eq!(Solution::count_of_pairs(n, x, y), expected);
    }
}
