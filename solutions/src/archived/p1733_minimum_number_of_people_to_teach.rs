///
/// # 1733. Minimum Number of People to Teach
///
/// On a social network consisting of `m` users and some friendships between users, two users can communicate with each other if they know a common language.
///
/// You are given an integer `n`, an array `languages`, and an array `friendships` where:
///
/// * There are `n` languages numbered `1` through `n`,
/// * `languages[i]` is the set of languages the `i<sup>​​​​​​th</sup>`​​​​ user knows, and
/// * `friendships[i] = [u<sub>​​​​​​i</sub>​​​, v<sub>​​​​​​i</sub>]` denotes a friendship between the users `u<sup>​​​​​</sup><sub>​​​​​​i</sub>`​​​​​ and `v<sub>i</sub>`.
///
/// You can choose **one** language and teach it to some users so that all friends can communicate with each other. Return *the* ***minimum*** *number of users you need to teach.*
///
/// Note that friendships are not transitive, meaning if `x` is a friend of `y` and `y` is a friend of `z`, this doesn't guarantee that `x` is a friend of `z`.
///
/// **Example 1:**
///
/// ```
/// Input: n = 2, languages = [[1],[2],[1,2]], friendships = [[1,2],[1,3],[2,3]]
/// Output: 1
/// Explanation: You can either teach user 1 the second language or user 2 the first language.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 3, languages = [[2],[1,3],[1,2],[3]], friendships = [[1,4],[1,2],[3,4],[2,3]]
/// Output: 2
/// Explanation: Teach the third language to users 1 and 3, yielding two users to teach.
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= n <= 500`
/// * `languages.length == m`
/// * `1 <= m <= 500`
/// * `1 <= languages[i].length <= n`
/// * `1 <= languages[i][j] <= n`
/// * `1 <= u<sub>​​​​​​i</sub> < v<sub>​​​​​​i</sub> <= languages.length`
/// * `1 <= friendships.length <= 500`
/// * All tuples `(u<sub>​​​​​i, </sub>v<sub>​​​​​​i</sub>)` are unique
/// * `languages[i]` contains only unique values
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-people-to-teach/
// discuss: https://leetcode.com/problems/minimum-number-of-people-to-teach/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let m = languages.len();

        let mut need_teach = vec![false; m];

        'outer: for friendship in &friendships {
            let (a, b) = (friendship[0] as usize - 1, friendship[1] as usize - 1);

            let mut langs = vec![false; n];

            for &l in &languages[a] {
                langs[l as usize - 1] = true;
            }

            for &l in &languages[b] {
                if langs[l as usize - 1] {
                    continue 'outer;
                }
            }

            need_teach[a] = true;
            need_teach[b] = true;
        }

        let mut teach_count = vec![need_teach.iter().filter(|&&x| x).count() as i32; n];

        for (language, _) in languages.into_iter().zip(need_teach).filter(|&(_, x)| x) {
            for l in language {
                teach_count[l as usize - 1] -= 1;
            }
        }

        teach_count.into_iter().min().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1733() {
        let n = 2;
        let languages = nd_vec![[1], [2], [1, 2]];
        let friendships = nd_vec![[1, 2], [1, 3], [2, 3]];
        let expected = 1;
        assert_eq!(
            Solution::minimum_teachings(n, languages, friendships),
            expected
        );
        let n = 3;
        let languages = nd_vec![[2], [1, 3], [1, 2], [3]];
        let friendships = nd_vec![[1, 4], [1, 2], [3, 4], [2, 3]];
        let expected = 2;
        assert_eq!(
            Solution::minimum_teachings(n, languages, friendships),
            expected
        );
    }
}
