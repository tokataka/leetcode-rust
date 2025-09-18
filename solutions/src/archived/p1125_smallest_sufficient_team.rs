///
/// # 1125. Smallest Sufficient Team
///
/// In a project, you have a list of required skills `req_skills`, and a list of people. The `i<sup>th</sup>` person `people[i]` contains a list of skills that the person has.
///
/// Consider a sufficient team: a set of people such that for every required skill in `req_skills`, there is at least one person in the team who has that skill. We can represent these teams by the index of each person.
///
/// * For example, `team = [0, 1, 3]` represents the people with skills `people[0]`, `people[1]`, and `people[3]`.
///
/// Return *any sufficient team of the smallest possible size, represented by the index of each person*. You may return the answer in **any order**.
///
/// It is **guaranteed** an answer exists.
///
/// **Example 1:**
///
/// ```
/// Input: req_skills = ["java","nodejs","reactjs"], people = [["java"],["nodejs"],["nodejs","reactjs"]]
/// Output: [0,2]
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: req_skills = ["algorithms","math","java","reactjs","csharp","aws"], people = [["algorithms","math","java"],["algorithms","math","reactjs"],["java","csharp","aws"],["reactjs","csharp"],["csharp","math"],["aws","java"]]
/// Output: [1,2]
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= req_skills.length <= 16`
/// * `1 <= req_skills[i].length <= 16`
/// * `req_skills[i]` consists of lowercase English letters.
/// * All the strings of `req_skills` are **unique**.
/// * `1 <= people.length <= 60`
/// * `0 <= people[i].length <= 16`
/// * `1 <= people[i][j].length <= 16`
/// * `people[i][j]` consists of lowercase English letters.
/// * All the strings of `people[i]` are **unique**.
/// * Every skill in `people[i]` is a skill in `req_skills`.
/// * It is guaranteed a sufficient team exists.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-sufficient-team/
// discuss: https://leetcode.com/problems/smallest-sufficient-team/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let n = req_skills.len();

        let req_skills = req_skills
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<HashMap<_, _>>();

        let people = people
            .into_iter()
            .map(|person| {
                person
                    .into_iter()
                    .fold(0, |acc, x| acc | (1 << req_skills.get(&x).unwrap()))
            })
            .collect::<Vec<_>>();

        let mut remain_after = vec![0];

        for &person in people.iter().skip(1).rev() {
            remain_after.push(person | remain_after.last().unwrap());
        }

        remain_after.reverse();

        fn _smallest_sufficient_team(
            i: usize,
            cur: i32,
            target: i32,
            remain_after: &[i32],
            people: &[i32],
            cache: &mut HashMap<(usize, i32), Vec<usize>>,
        ) -> Vec<usize> {
            if cur == target {
                return vec![];
            }

            if let Some(x) = cache.get(&(i, cur)) {
                return x.clone();
            }

            let mut result = vec![i];

            result.extend_from_slice(&_smallest_sufficient_team(
                i + 1,
                cur | people[i],
                target,
                remain_after,
                people,
                cache,
            ));

            if cur | remain_after[i] == target {
                let maybe_result =
                    _smallest_sufficient_team(i + 1, cur, target, remain_after, people, cache);

                if maybe_result.len() < result.len() {
                    result = maybe_result;
                }
            }

            cache.insert((i, cur), result.clone());

            result
        }

        _smallest_sufficient_team(
            0,
            0,
            (1 << n) - 1,
            &remain_after,
            &people,
            &mut HashMap::new(),
        )
        .into_iter()
        .map(|x| x as i32)
        .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1125() {
        // let req_skills = vec_string!["java", "nodejs", "reactjs"];
        // let people = nd_vec_string![["java"], ["nodejs"], ["nodejs", "reactjs"]];
        // let expected = vec![0, 2];
        // assert_eq!(
        //     Solution::smallest_sufficient_team(req_skills, people),
        //     expected
        // );
        // let req_skills = vec_string!["algorithms", "math", "java", "reactjs", "csharp", "aws"];
        // let people = nd_vec_string![
        //     ["algorithms", "math", "java"],
        //     ["algorithms", "math", "reactjs"],
        //     ["java", "csharp", "aws"],
        //     ["reactjs", "csharp"],
        //     ["csharp", "math"],
        //     ["aws", "java"]
        // ];
        // let expected = vec![1, 2];
        // assert_eq!(
        //     Solution::smallest_sufficient_team(req_skills, people),
        //     expected
        // );
        let req_skills = vec_string![
            "gvp",
            "jgpzzicdvgxlfix",
            "kqcrfwerywbwi",
            "jzukdzrfgvdbrunw",
            "k"
        ];
        let people = nd_vec_string![
            [],
            [],
            [],
            [],
            ["jgpzzicdvgxlfix"],
            ["jgpzzicdvgxlfix", "k"],
            ["jgpzzicdvgxlfix", "kqcrfwerywbwi"],
            ["gvp"],
            ["jzukdzrfgvdbrunw"],
            ["gvp", "kqcrfwerywbwi"]
        ];
        let expected = vec![5, 8, 9];
        assert_eq!(
            Solution::smallest_sufficient_team(req_skills, people),
            expected
        );
    }
}
