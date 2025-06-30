///
/// # 2050. Parallel Courses III
///
/// You are given an integer `n`, which indicates that there are `n` courses labeled from `1` to `n`. You are also given a 2D integer array `relations` where `relations[j] = [prevCourse<sub>j</sub>, nextCourse<sub>j</sub>]` denotes that course `prevCourse<sub>j</sub>` has to be completed **before** course `nextCourse<sub>j</sub>` (prerequisite relationship). Furthermore, you are given a **0-indexed** integer array `time` where `time[i]` denotes how many **months** it takes to complete the `(i+1)<sup>th</sup>` course.
///
/// You must find the **minimum** number of months needed to complete all the courses following these rules:
///
/// * You may start taking a course at **any time** if the prerequisites are met.
/// * **Any number of courses** can be taken at the **same time**.
///
/// Return *the **minimum** number of months needed to complete all the courses*.
///
/// **Note:** The test cases are generated such that it is possible to complete every course (i.e., the graph is a directed acyclic graph).
///
/// **Example 1:**
///
/// **![](https://assets.leetcode.com/uploads/2021/10/07/ex1.png)**
///
/// ```
/// Input: n = 3, relations = [[1,3],[2,3]], time = [3,2,5]
/// Output: 8
/// Explanation: The figure above represents the given graph and the time required to complete each course.
/// We start course 1 and course 2 simultaneously at month 0.
/// Course 1 takes 3 months and course 2 takes 2 months to complete respectively.
/// Thus, the earliest time we can start course 3 is at month 3, and the total time required is 3 + 5 = 8 months.
///
/// ```
///
/// **Example 2:**
///
/// **![](https://assets.leetcode.com/uploads/2021/10/07/ex2.png)**
///
/// ```
/// Input: n = 5, relations = [[1,5],[2,5],[3,5],[3,4],[4,5]], time = [1,2,3,4,5]
/// Output: 12
/// Explanation: The figure above represents the given graph and the time required to complete each course.
/// You can start courses 1, 2, and 3 at month 0.
/// You can complete them after 1, 2, and 3 months respectively.
/// Course 4 can be taken only after course 3 is completed, i.e., after 3 months. It is completed after 3 + 4 = 7 months.
/// Course 5 can be taken only after courses 1, 2, 3, and 4 have been completed, i.e., after max(1,2,3,7) = 7 months.
/// Thus, the minimum time needed to complete all the courses is 7 + 5 = 12 months.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 5 * 10<sup>4</sup>`
/// * `0 <= relations.length <= min(n * (n - 1) / 2, 5 * 10<sup>4</sup>)`
/// * `relations[j].length == 2`
/// * `1 <= prevCourse<sub>j</sub>, nextCourse<sub>j</sub> <= n`
/// * `prevCourse<sub>j</sub> != nextCourse<sub>j</sub>`
/// * All the pairs `[prevCourse<sub>j</sub>, nextCourse<sub>j</sub>]` are **unique**.
/// * `time.length == n`
/// * `1 <= time[i] <= 10<sup>4</sup>`
/// * The given graph is a directed acyclic graph.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/parallel-courses-iii/
// discuss: https://leetcode.com/problems/parallel-courses-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let n = n as usize;

        let mut graph = vec![vec![]; n];
        let mut in_degree = vec![0; n];

        for relation in relations {
            let (from, to) = (relation[0] as usize - 1, relation[1] as usize - 1);
            graph[from].push(to);
            in_degree[to] += 1;
        }

        let mut st = in_degree
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 0)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        let mut result = 0;
        let mut prerequisites = vec![0; n];

        while let Some(cur) = st.pop() {
            let cur_time = prerequisites[cur] + time[cur];

            result = result.max(cur_time);

            for &next in &graph[cur] {
                prerequisites[next] = prerequisites[next].max(cur_time);
                in_degree[next] -= 1;

                if in_degree[next] == 0 {
                    st.push(next);
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
    fn test_2050() {
        let n = 3;
        let relations = nd_vec![[1, 3], [2, 3]];
        let time = vec![3, 2, 5];
        let expected = 8;
        assert_eq!(Solution::minimum_time(n, relations, time), expected);
        let n = 5;
        let relations = nd_vec![[1, 5], [2, 5], [3, 5], [3, 4], [4, 5]];
        let time = vec![1, 2, 3, 4, 5];
        let expected = 12;
        assert_eq!(Solution::minimum_time(n, relations, time), expected);
    }
}
