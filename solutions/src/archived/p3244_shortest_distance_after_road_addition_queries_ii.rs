///
/// # 3244. Shortest Distance After Road Addition Queries II
///
/// You are given an integer `n` and a 2D integer array `queries`.
///
/// There are `n` cities numbered from `0` to `n - 1`. Initially, there is a **unidirectional** road from city `i` to city `i + 1` for all `0 <= i < n - 1`.
///
/// `queries[i] = [u<sub>i</sub>, v<sub>i</sub>]` represents the addition of a new **unidirectional** road from city `u<sub>i</sub>` to city `v<sub>i</sub>`. After each query, you need to find the **length** of the **shortest path** from city `0` to city `n - 1`.
///
/// There are no two queries such that `queries[i][0] < queries[j][0] < queries[i][1] < queries[j][1]`.
///
/// Return an array `answer` where for each `i` in the range `[0, queries.length - 1]`, `answer[i]` is the *length of the shortest path* from city `0` to city `n - 1` after processing the **first** `i + 1` queries.
///
/// **Example 1:**
///
/// **Input:** n = 5, queries = [[2,4],[0,2],[0,4]]
///
/// **Output:** [3,2,1]
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/06/28/image8.jpg)
///
/// After the addition of the road from 2 to 4, the length of the shortest path from 0 to 4 is 3.
///
/// ![](https://assets.leetcode.com/uploads/2024/06/28/image9.jpg)
///
/// After the addition of the road from 0 to 2, the length of the shortest path from 0 to 4 is 2.
///
/// ![](https://assets.leetcode.com/uploads/2024/06/28/image10.jpg)
///
/// After the addition of the road from 0 to 4, the length of the shortest path from 0 to 4 is 1.
///
/// **Example 2:**
///
/// **Input:** n = 4, queries = [[0,3],[0,2]]
///
/// **Output:** [1,1]
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/06/28/image11.jpg)
///
/// After the addition of the road from 0 to 3, the length of the shortest path from 0 to 3 is 1.
///
/// ![](https://assets.leetcode.com/uploads/2024/06/28/image12.jpg)
///
/// After the addition of the road from 0 to 2, the length of the shortest path remains 1.
///
/// **Constraints:**
///
/// * `3 <= n <= 10<sup>5</sup>`
/// * `1 <= queries.length <= 10<sup>5</sup>`
/// * `queries[i].length == 2`
/// * `0 <= queries[i][0] < queries[i][1] < n`
/// * `1 < queries[i][1] - queries[i][0]`
/// * There are no repeated roads among the queries.
/// * There are no two queries such that `i != j` and `queries[i][0] < queries[j][0] < queries[i][1] < queries[j][1]`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-distance-after-road-addition-queries-ii/
// discuss: https://leetcode.com/problems/shortest-distance-after-road-addition-queries-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = (0..n as usize - 1).map(|x| x + 1).collect::<Vec<_>>();
        let mut result = Vec::with_capacity(queries.len());
        let mut count = n - 1;

        for query in queries {
            let (a, b) = (query[0] as usize, query[1] as usize);

            if b > graph[a] {
                let mut cur = graph[a];

                while cur < b {
                    let next = graph[cur];
                    graph[cur] = usize::MAX;
                    cur = next;
                    count -= 1;
                }

                graph[a] = b;
            }

            result.push(count);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3244() {
        let n = 5;
        let queries = nd_vec![[2, 4], [0, 2], [0, 4]];
        let expected = vec![3, 2, 1];
        assert_eq!(
            Solution::shortest_distance_after_queries(n, queries),
            expected
        );
        let n = 4;
        let queries = nd_vec![[0, 3], [0, 2]];
        let expected = vec![1, 1];
        assert_eq!(
            Solution::shortest_distance_after_queries(n, queries),
            expected
        );
    }
}
