use std::collections::HashMap;

///
/// # 2127. Maximum Employees to Be Invited to a Meeting
///
/// A company is organizing a meeting and has a list of `n` employees, waiting to be invited. They have arranged for a large **circular** table, capable of seating **any number** of employees.
///
/// The employees are numbered from `0` to `n - 1`. Each employee has a **favorite** person and they will attend the meeting **only if** they can sit next to their favorite person at the table. The favorite person of an employee is **not** themself.
///
/// Given a **0-indexed** integer array `favorite`, where `favorite[i]` denotes the favorite person of the `i<sup>th</sup>` employee, return *the **maximum number of employees** that can be invited to the meeting*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/12/14/ex1.png)
///
/// ```
/// Input: favorite = [2,2,1,2]
/// Output: 3
/// Explanation:
/// The above figure shows how the company can invite employees 0, 1, and 2, and seat them at the round table.
/// All employees cannot be invited because employee 2 cannot sit beside employees 0, 1, and 3, simultaneously.
/// Note that the company can also invite employees 1, 2, and 3, and give them their desired seats.
/// The maximum number of employees that can be invited to the meeting is 3.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: favorite = [1,2,0]
/// Output: 3
/// Explanation:
/// Each employee is the favorite person of at least one other employee, and the only way the company can invite them is if they invite every employee.
/// The seating arrangement will be the same as that in the figure given in example 1:
/// - Employee 0 will sit between employees 2 and 1.
/// - Employee 1 will sit between employees 0 and 2.
/// - Employee 2 will sit between employees 1 and 0.
/// The maximum number of employees that can be invited to the meeting is 3.
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2021/12/14/ex2.png)
///
/// ```
/// Input: favorite = [3,0,1,4,1]
/// Output: 4
/// Explanation:
/// The above figure shows how the company will invite employees 0, 1, 3, and 4, and seat them at the round table.
/// Employee 2 cannot be invited because the two spots next to their favorite employee 1 are taken.
/// So the company leaves them out of the meeting.
/// The maximum number of employees that can be invited to the meeting is 4.
///
/// ```
///
/// **Constraints:**
///
/// * `n == favorite.length`
/// * `2 <= n <= 10<sup>5</sup>`
/// * `0 <= favorite[i] <= n - 1`
/// * `favorite[i] != i`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-employees-to-be-invited-to-a-meeting/
// discuss: https://leetcode.com/problems/maximum-employees-to-be-invited-to-a-meeting/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let n = favorite.len();

        if n <= 3 {
            return n as i32;
        }

        let mut result = 0;

        let mut cache = vec![None; n];

        let mut pair_counts = 0;
        let mut pair_distances: HashMap<usize, usize> = HashMap::new();

        let mut visited = vec![false; n];
        let mut path = vec![];

        for idx in 0..n {
            let mut cur = idx;
            path.clear();

            while !visited[cur] {
                visited[cur] = true;
                path.push(cur);
                cur = favorite[cur] as usize;
            }

            let (loop_vertex, loop_distance, loop_size) = match cache[cur] {
                Some(x) => x,
                None => {
                    let loop_idx = path.iter().position(|&x| x == cur).unwrap();
                    let loop_size = path.len() - loop_idx;

                    result = result.max(loop_size);

                    for _ in 0..loop_size {
                        let v = path.pop().unwrap();
                        cache[v] = Some((v, 0, loop_size));
                    }

                    if loop_size == 2 {
                        pair_counts += 2;
                    }

                    (cur, 0, loop_size)
                }
            };

            for (i, &v) in path.iter().rev().enumerate() {
                cache[v] = Some((loop_vertex, loop_distance + i + 1, loop_size));
            }

            if loop_size == 2 {
                pair_distances
                    .entry(loop_vertex)
                    .and_modify(|x| *x = (*x).max(loop_distance + path.len()))
                    .or_insert(loop_distance + path.len());
            }
        }

        result = result.max(pair_counts + pair_distances.values().sum::<usize>());

        result.max(3) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2127() {
        // let favorite = vec![2, 2, 1, 2];
        // let expected = 3;
        // assert_eq!(Solution::maximum_invitations(favorite), expected);
        // let favorite = vec![1, 2, 0];
        // let expected = 3;
        // assert_eq!(Solution::maximum_invitations(favorite), expected);
        let favorite = vec![1, 0, 3, 2, 5, 6, 7, 4, 9, 8, 11, 10, 11, 12, 10];
        let expected = 11;
        assert_eq!(Solution::maximum_invitations(favorite), expected);
    }
}
