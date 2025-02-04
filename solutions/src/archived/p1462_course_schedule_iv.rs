use std::collections::VecDeque;

///
/// # 1462. Course Schedule IV
///
/// There are a total of `numCourses` courses you have to take, labeled from `0` to `numCourses - 1`. You are given an array `prerequisites` where `prerequisites[i] = [a<sub>i</sub>, b<sub>i</sub>]` indicates that you **must** take course `a<sub>i</sub>` first if you want to take course `b<sub>i</sub>`.
///
/// * For example, the pair `[0, 1]` indicates that you have to take course `0` before you can take course `1`.
///
/// Prerequisites can also be **indirect**. If course `a` is a prerequisite of course `b`, and course `b` is a prerequisite of course `c`, then course `a` is a prerequisite of course `c`.
///
/// You are also given an array `queries` where `queries[j] = [u<sub>j</sub>, v<sub>j</sub>]`. For the `j<sup>th</sup>` query, you should answer whether course `u<sub>j</sub>` is a prerequisite of course `v<sub>j</sub>` or not.
///
/// Return *a boolean array* `answer`*, where* `answer[j]` *is the answer to the* `j<sup>th</sup>` *query.*
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/05/01/courses4-1-graph.jpg)
///
/// ```
/// Input: numCourses = 2, prerequisites = [[1,0]], queries = [[0,1],[1,0]]
/// Output: [false,true]
/// Explanation: The pair [1, 0] indicates that you have to take course 1 before you can take course 0.
/// Course 0 is not a prerequisite of course 1, but the opposite is true.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: numCourses = 2, prerequisites = [], queries = [[1,0],[0,1]]
/// Output: [false,false]
/// Explanation: There are no prerequisites, and each course is independent.
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2021/05/01/courses4-3-graph.jpg)
///
/// ```
/// Input: numCourses = 3, prerequisites = [[1,2],[1,0],[2,0]], queries = [[1,0],[1,2]]
/// Output: [true,true]
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= numCourses <= 100`
/// * `0 <= prerequisites.length <= (numCourses * (numCourses - 1) / 2)`
/// * `prerequisites[i].length == 2`
/// * `0 <= a<sub>i</sub>, b<sub>i</sub> <= numCourses - 1`
/// * `a<sub>i</sub> != b<sub>i</sub>`
/// * All the pairs `[a<sub>i</sub>, b<sub>i</sub>]` are **unique**.
/// * The prerequisites graph has no cycles.
/// * `1 <= queries.length <= 10<sup>4</sup>`
/// * `0 <= u<sub>i</sub>, v<sub>i</sub> <= numCourses - 1`
/// * `u<sub>i</sub> != v<sub>i</sub>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/course-schedule-iv/
// discuss: https://leetcode.com/problems/course-schedule-iv/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let num_courses = num_courses as usize;

        let mut graph = vec![vec![]; num_courses];
        let mut graph_rev = vec![vec![]; num_courses];
        let mut degree = vec![0; num_courses];

        for p in &prerequisites {
            graph[p[0] as usize].push(p[1] as usize);
            graph_rev[p[1] as usize].push(p[0] as usize);

            degree[p[1] as usize] += 1;
        }

        let mut topologic = vec![];
        let mut q = VecDeque::from_iter(
            degree
                .iter()
                .enumerate()
                .filter(|(_, &x)| x == 0)
                .map(|(i, _)| i),
        );

        while let Some(cur) = q.pop_front() {
            topologic.push(cur);

            for &next in &graph[cur] {
                degree[next] -= 1;
                if degree[next] == 0 {
                    q.push_back(next);
                }
            }
        }

        let mut is_prerequisite = vec![0u128; num_courses];

        for cur in topologic.into_iter().rev() {
            let p = is_prerequisite[cur] | (1 << cur);

            for &prev in &graph_rev[cur] {
                is_prerequisite[prev] |= p;
            }
        }

        queries
            .iter()
            .map(|query| is_prerequisite[query[0] as usize] & (1 << query[1]) > 0)
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1462() {
        // let num_courses = 2;
        // let prerequisites = nd_vec![[1, 0]];
        // let queries = nd_vec![[0, 1], [1, 0]];
        // let expected = vec![false, true];
        // assert_eq!(
        //     Solution::check_if_prerequisite(num_courses, prerequisites, queries),
        //     expected
        // );
        // let num_courses = 2;
        // let prerequisites = nd_vec![];
        // let queries = nd_vec![[1, 0], [0, 1]];
        // let expected = vec![false, false];
        // assert_eq!(
        //     Solution::check_if_prerequisite(num_courses, prerequisites, queries),
        //     expected
        // );
        // let num_courses = 3;
        // let prerequisites = nd_vec![[1, 2], [1, 0], [2, 0]];
        // let queries = nd_vec![[1, 0], [1, 2]];
        // let expected = vec![true, true];
        // assert_eq!(
        //     Solution::check_if_prerequisite(num_courses, prerequisites, queries),
        //     expected
        // );
        let num_courses = 4;
        let prerequisites = nd_vec![[2, 3], [2, 1], [0, 3], [0, 1]];
        let queries = nd_vec![[0, 1], [0, 3], [2, 3], [3, 0], [2, 0], [0, 2]];
        let expected = vec![true, true, true, false, false, false];
        assert_eq!(
            Solution::check_if_prerequisite(num_courses, prerequisites, queries),
            expected
        );
    }
}
