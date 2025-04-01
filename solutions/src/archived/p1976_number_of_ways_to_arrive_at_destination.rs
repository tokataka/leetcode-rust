///
/// # 1976. Number of Ways to Arrive at Destination
///
/// You are in a city that consists of `n` intersections numbered from `0` to `n - 1` with **bi-directional** roads between some intersections. The inputs are generated such that you can reach any intersection from any other intersection and that there is at most one road between any two intersections.
///
/// You are given an integer `n` and a 2D integer array `roads` where `roads[i] = [u<sub>i</sub>, v<sub>i</sub>, time<sub>i</sub>]` means that there is a road between intersections `u<sub>i</sub>` and `v<sub>i</sub>` that takes `time<sub>i</sub>` minutes to travel. You want to know in how many ways you can travel from intersection `0` to intersection `n - 1` in the **shortest amount of time**.
///
/// Return *the **number of ways** you can arrive at your destination in the **shortest amount of time***. Since the answer may be large, return it **modulo** `10<sup>9</sup> + 7`.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2025/02/14/1976_corrected.png)
///
/// ```
/// Input: n = 7, roads = [[0,6,7],[0,1,2],[1,2,3],[1,3,3],[6,3,3],[3,5,1],[6,5,1],[2,5,1],[0,4,5],[4,6,2]]
/// Output: 4
/// Explanation: The shortest amount of time it takes to go from intersection 0 to intersection 6 is 7 minutes.
/// The four ways to get there in 7 minutes are:
/// - 0 ➝ 6
/// - 0 ➝ 4 ➝ 6
/// - 0 ➝ 1 ➝ 2 ➝ 5 ➝ 6
/// - 0 ➝ 1 ➝ 3 ➝ 5 ➝ 6
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 2, roads = [[1,0,10]]
/// Output: 1
/// Explanation: There is only one way to go from intersection 0 to intersection 1, and it takes 10 minutes.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 200`
/// * `n - 1 <= roads.length <= n * (n - 1) / 2`
/// * `roads[i].length == 3`
/// * `0 <= u<sub>i</sub>, v<sub>i</sub> <= n - 1`
/// * `1 <= time<sub>i</sub> <= 10<sup>9</sup>`
/// * `u<sub>i </sub>!= v<sub>i</sub>`
/// * There is at most one road connecting any two intersections.
/// * You can reach any intersection from any other intersection.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-ways-to-arrive-at-destination/
// discuss: https://leetcode.com/problems/number-of-ways-to-arrive-at-destination/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = n as usize;

        let mut cost = vec![i64::MAX; n];
        cost[0] = 0;

        let mut count = vec![0; n];
        count[0] = 1;

        let mut edges = vec![vec![]; n];

        for road in roads {
            edges[road[0] as usize].push((road[1] as usize, road[2] as i64));
            edges[road[1] as usize].push((road[0] as usize, road[2] as i64));
        }

        let mut visited = vec![false; n];

        while let Some((cur_cost, cur)) = visited
            .iter()
            .enumerate()
            .filter(|(_, &x)| !x)
            .map(|(i, _)| (cost[i], i))
            .min()
        {
            if cur == n - 1 {
                break;
            }

            visited[cur] = true;

            let cur_count = count[cur];

            for &(next, weight) in &edges[cur] {
                match cost[next].cmp(&(cur_cost + weight)) {
                    std::cmp::Ordering::Less => (),
                    std::cmp::Ordering::Equal => count[next] = (count[next] + cur_count) % MOD,
                    std::cmp::Ordering::Greater => {
                        cost[next] = cur_cost + weight;
                        count[next] = cur_count;
                    }
                };
            }
        }

        count[n - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1976() {
        let n = 7;
        let roads = nd_vec![
            [0, 6, 7],
            [0, 1, 2],
            [1, 2, 3],
            [1, 3, 3],
            [6, 3, 3],
            [3, 5, 1],
            [6, 5, 1],
            [2, 5, 1],
            [0, 4, 5],
            [4, 6, 2]
        ];
        let expected = 4;
        assert_eq!(Solution::count_paths(n, roads), expected);
        let n = 2;
        let roads = nd_vec![[1, 0, 10]];
        let expected = 1;
        assert_eq!(Solution::count_paths(n, roads), expected);
    }
}
