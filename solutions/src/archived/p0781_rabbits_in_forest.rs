///
/// # 781. Rabbits in Forest
///
/// There is a forest with an unknown number of rabbits. We asked n rabbits **"How many rabbits have the same color as you?"** and collected the answers in an integer array `answers` where `answers[i]` is the answer of the `i<sup>th</sup>` rabbit.
///
/// Given the array `answers`, return *the minimum number of rabbits that could be in the forest*.
///
/// **Example 1:**
///
/// ```
/// Input: answers = [1,1,2]
/// Output: 5
/// Explanation:
/// The two rabbits that answered "1" could both be the same color, say red.
/// The rabbit that answered "2" can't be red or the answers would be inconsistent.
/// Say the rabbit that answered "2" was blue.
/// Then there should be 2 other blue rabbits in the forest that didn't answer into the array.
/// The smallest possible number of rabbits in the forest is therefore 5: 3 that answered plus 2 that didn't.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: answers = [10,10,10]
/// Output: 11
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= answers.length <= 1000`
/// * `0 <= answers[i] < 1000`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/rabbits-in-forest/
// discuss: https://leetcode.com/problems/rabbits-in-forest/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let answer_counts = answers.into_iter().fold([0; 1000], |mut acc, x| {
            acc[x as usize] += 1;
            acc
        });

        answer_counts
            .into_iter()
            .enumerate()
            .filter(|&(_, count)| count > 0)
            .map(|(x, count)| ((count - 1) / (x as i32 + 1) + 1) * (x as i32 + 1))
            .sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_781() {
        let answers = vec![1, 1, 2];
        let expected = 5;
        assert_eq!(Solution::num_rabbits(answers), expected);
        let answers = vec![10, 10, 10];
        let expected = 11;
        assert_eq!(Solution::num_rabbits(answers), expected);
    }
}
