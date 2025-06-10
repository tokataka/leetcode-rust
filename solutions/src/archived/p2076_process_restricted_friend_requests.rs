///
/// # 2076. Process Restricted Friend Requests
///
/// You are given an integer `n` indicating the number of people in a network. Each person is labeled from `0` to `n - 1`.
///
/// You are also given a **0-indexed** 2D integer array `restrictions`, where `restrictions[i] = [x<sub>i</sub>, y<sub>i</sub>]` means that person `x<sub>i</sub>` and person `y<sub>i</sub>` **cannot** become **friends**, either **directly** or **indirectly** through other people.
///
/// Initially, no one is friends with each other. You are given a list of friend requests as a **0-indexed** 2D integer array `requests`, where `requests[j] = [u<sub>j</sub>, v<sub>j</sub>]` is a friend request between person `u<sub>j</sub>` and person `v<sub>j</sub>`.
///
/// A friend request is **successful** if `u<sub>j</sub>` and `v<sub>j</sub>` can be **friends**. Each friend request is processed in the given order (i.e., `requests[j]` occurs before `requests[j + 1]`), and upon a successful request, `u<sub>j</sub>` and `v<sub>j</sub>` **become direct friends** for all future friend requests.
///
/// Return *a **boolean array*** `result`, *where each* `result[j]` *is* `true` *if the* `j<sup>th</sup>` *friend request is **successful** or* `false` *if it is not*.
///
/// **Note:** If `u<sub>j</sub>` and `v<sub>j</sub>` are already direct friends, the request is still **successful**.
///
/// **Example 1:**
///
/// ```
/// Input: n = 3, restrictions = [[0,1]], requests = [[0,2],[2,1]]
/// Output: [true,false]
/// Explanation:
/// Request 0: Person 0 and person 2 can be friends, so they become direct friends.
/// Request 1: Person 2 and person 1 cannot be friends since person 0 and person 1 would be indirect friends (1--2--0).
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 3, restrictions = [[0,1]], requests = [[1,2],[0,2]]
/// Output: [true,false]
/// Explanation:
/// Request 0: Person 1 and person 2 can be friends, so they become direct friends.
/// Request 1: Person 0 and person 2 cannot be friends since person 0 and person 1 would be indirect friends (0--2--1).
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: n = 5, restrictions = [[0,1],[1,2],[2,3]], requests = [[0,4],[1,2],[3,1],[3,4]]
/// Output: [true,false,true,false]
/// Explanation:
/// Request 0: Person 0 and person 4 can be friends, so they become direct friends.
/// Request 1: Person 1 and person 2 cannot be friends since they are directly restricted.
/// Request 2: Person 3 and person 1 can be friends, so they become direct friends.
/// Request 3: Person 3 and person 4 cannot be friends since person 0 and person 1 would be indirect friends (0--4--3--1).
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= n <= 1000`
/// * `0 <= restrictions.length <= 1000`
/// * `restrictions[i].length == 2`
/// * `0 <= x<sub>i</sub>, y<sub>i</sub> <= n - 1`
/// * `x<sub>i</sub> != y<sub>i</sub>`
/// * `1 <= requests.length <= 1000`
/// * `requests[j].length == 2`
/// * `0 <= u<sub>j</sub>, v<sub>j</sub> <= n - 1`
/// * `u<sub>j</sub> != v<sub>j</sub>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/process-restricted-friend-requests/
// discuss: https://leetcode.com/problems/process-restricted-friend-requests/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct UnionFind {
    data: Vec<usize>,
    size: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            data: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, a: usize) -> usize {
        if self.data[a] == a {
            return a;
        }

        self.data[a] = self.find(self.data[a]);
        self.data[a]
    }

    fn union(&mut self, a: usize, b: usize, restrictions: &Vec<Vec<i32>>) -> bool {
        let (a, b) = (self.find(a), self.find(b));

        if a == b {
            return true;
        }

        let (a, b) = (a.min(b), a.max(b));

        for t in restrictions {
            let (x, y) = (self.find(t[0] as usize), self.find(t[1] as usize));
            let (x, y) = (x.min(y), x.max(y));

            if a == x && b == y {
                return false;
            }
        }

        let (a, b) = match self.size[a] < self.size[b] {
            true => (b, a),
            false => (a, b),
        };

        self.data[b] = a;
        self.size[a] += self.size[b];

        true
    }
}

impl Solution {
    pub fn friend_requests(
        n: i32,
        restrictions: Vec<Vec<i32>>,
        requests: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = n as usize;

        let mut uf = UnionFind::new(n);

        requests
            .into_iter()
            .map(|x| uf.union(x[0] as usize, x[1] as usize, &restrictions))
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2076() {
        let n = 3;
        let restrictions = nd_vec![[0, 1]];
        let requests = nd_vec![[0, 2], [2, 1]];
        let expected = vec![true, false];
        assert_eq!(
            Solution::friend_requests(n, restrictions, requests),
            expected
        );
        let n = 3;
        let restrictions = nd_vec![[0, 1]];
        let requests = nd_vec![[1, 2], [0, 2]];
        let expected = vec![true, false];
        assert_eq!(
            Solution::friend_requests(n, restrictions, requests),
            expected
        );
        let n = 5;
        let restrictions = nd_vec![[0, 1], [1, 2], [2, 3]];
        let requests = nd_vec![[0, 4], [1, 2], [3, 1], [3, 4]];
        let expected = vec![true, false, true, false];
        assert_eq!(
            Solution::friend_requests(n, restrictions, requests),
            expected
        );
        // let n = 8;
        // let restrictions = nd_vec![
        //     [6, 4],
        //     [7, 5],
        //     [2, 6],
        //     [1, 5],
        //     [6, 7],
        //     [6, 5],
        //     [0, 3],
        //     [5, 4],
        //     [0, 4],
        //     [2, 7],
        //     [0, 2]
        // ];
        // let requests = nd_vec![
        //     [6, 3],
        //     [0, 2],
        //     [0, 5],
        //     [0, 3],
        //     [6, 4],
        //     [2, 4],
        //     [1, 0],
        //     [2, 1],
        //     [2, 5],
        //     [6, 7],
        //     [7, 0],
        //     [3, 2],
        //     [3, 5],
        //     [2, 1],
        //     [1, 6],
        //     [7, 4],
        //     [6, 3],
        //     [1, 3],
        //     [6, 5],
        //     [3, 7],
        //     [7, 0],
        //     [6, 5],
        //     [0, 5],
        //     [0, 4],
        //     [7, 5],
        //     [7, 0],
        //     [7, 0],
        //     [1, 3]
        // ];
        // let expected = vec![
        //     true, false, true, false, false, true, false, true, false, false, false, false, false,
        //     true, false, false, true, false, false, false, false, false, true, false, false, false,
        //     false, false,
        // ];
        // assert_eq!(
        //     Solution::friend_requests(n, restrictions, requests),
        //     expected
        // );
    }
}
