///
/// # 135. Candy
///
/// There are `n` children standing in a line. Each child is assigned a rating value given in the integer array `ratings`.
///
/// You are giving candies to these children subjected to the following requirements:
///
/// * Each child must have at least one candy.
/// * Children with a higher rating get more candies than their neighbors.
///
/// Return *the minimum number of candies you need to have to distribute the candies to the children*.
///
/// **Example 1:**
///
/// ```
/// Input: ratings = [1,0,2]
/// Output: 5
/// Explanation: You can allocate to the first, second and third child with 2, 1, 2 candies respectively.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: ratings = [1,2,2]
/// Output: 4
/// Explanation: You can allocate to the first, second and third child with 1, 2, 1 candies respectively.
/// The third child gets 1 candy because it satisfies the above two conditions.
///
/// ```
///
/// **Constraints:**
///
/// * `n == ratings.length`
/// * `1 <= n <= 2 * 10<sup>4</sup>`
/// * `0 <= ratings[i] <= 2 * 10<sup>4</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/candy/
// discuss: https://leetcode.com/problems/candy/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut result = 0;

        for chunk in ratings.chunk_by(|a, b| a != b) {
            let mut left = 0;

            let mut prev_prev = -1;
            let mut prev = -1;

            let mut is_inclining = true;
            let mut incline_candy = 0;

            for (idx, &rating) in chunk.iter().enumerate() {
                if is_inclining && prev_prev < prev && prev > rating {
                    let right = idx - 1;

                    result += (right + 1 - left) * (right - left) / 2;

                    is_inclining = false;
                    incline_candy = right + 1 - left;
                    left = right;
                }

                if !is_inclining && prev_prev > prev && prev < rating {
                    let right = idx - 1;

                    if incline_candy > right + 1 - left {
                        result += incline_candy;
                        left += 1;
                    }

                    result += (right + 2 - left) * (right + 1 - left) / 2 - 1;

                    is_inclining = true;
                    left = right;
                }

                (prev_prev, prev) = (prev, rating);
            }

            if !is_inclining && incline_candy > chunk.len() - left {
                result += incline_candy;
                left += 1;
            }

            result += (chunk.len() + 1 - left) * (chunk.len() - left) / 2;
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_135() {
        // let ratings = vec![1, 0, 2];
        // let expected = 5;
        // assert_eq!(Solution::candy(ratings), expected);
        // let ratings = vec![1, 2, 2];
        // let expected = 4;
        // assert_eq!(Solution::candy(ratings), expected);
        let ratings = vec![0, 1, 2, 3, 2, 1];
        let expected = 13;
        assert_eq!(Solution::candy(ratings), expected);
    }
}
