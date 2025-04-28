///
/// # 2101. Detonate the Maximum Bombs
///
/// You are given a list of bombs. The **range** of a bomb is defined as the area where its effect can be felt. This area is in the shape of a **circle** with the center as the location of the bomb.
///
/// The bombs are represented by a **0-indexed** 2D integer array `bombs` where `bombs[i] = [x<sub>i</sub>, y<sub>i</sub>, r<sub>i</sub>]`. `x<sub>i</sub>` and `y<sub>i</sub>` denote the X-coordinate and Y-coordinate of the location of the `i<sup>th</sup>` bomb, whereas `r<sub>i</sub>` denotes the **radius** of its range.
///
/// You may choose to detonate a **single** bomb. When a bomb is detonated, it will detonate **all bombs** that lie in its range. These bombs will further detonate the bombs that lie in their ranges.
///
/// Given the list of `bombs`, return *the **maximum** number of bombs that can be detonated if you are allowed to detonate **only one** bomb*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/11/06/desmos-eg-3.png)
///
/// ```
/// Input: bombs = [[2,1,3],[6,1,4]]
/// Output: 2
/// Explanation:
/// The above figure shows the positions and ranges of the 2 bombs.
/// If we detonate the left bomb, the right bomb will not be affected.
/// But if we detonate the right bomb, both bombs will be detonated.
/// So the maximum bombs that can be detonated is max(1, 2) = 2.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/11/06/desmos-eg-2.png)
///
/// ```
/// Input: bombs = [[1,1,5],[10,10,5]]
/// Output: 1
/// Explanation:
/// Detonating either bomb will not detonate the other bomb, so the maximum number of bombs that can be detonated is 1.
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2021/11/07/desmos-eg1.png)
///
/// ```
/// Input: bombs = [[1,2,3],[2,3,1],[3,4,2],[4,5,3],[5,6,4]]
/// Output: 5
/// Explanation:
/// The best bomb to detonate is bomb 0 because:
/// - Bomb 0 detonates bombs 1 and 2. The red circle denotes the range of bomb 0.
/// - Bomb 2 detonates bomb 3. The blue circle denotes the range of bomb 2.
/// - Bomb 3 detonates bomb 4. The green circle denotes the range of bomb 3.
/// Thus all 5 bombs are detonated.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= bombs.length <= 100`
/// * `bombs[i].length == 3`
/// * `1 <= x<sub>i</sub>, y<sub>i</sub>, r<sub>i</sub> <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/detonate-the-maximum-bombs/
// discuss: https://leetcode.com/problems/detonate-the-maximum-bombs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let n = bombs.len();
        let mut graph = vec![vec![]; n];

        for (i, bomb1) in bombs.iter().enumerate() {
            let (x1, y1, r1) = (bomb1[0] as i64, bomb1[1] as i64, bomb1[2] as i64);

            for (j, bomb2) in bombs.iter().enumerate().skip(i + 1) {
                let (x2, y2, r2) = (bomb2[0] as i64, bomb2[1] as i64, bomb2[2] as i64);

                if r1.pow(2) >= (x2 - x1).abs().pow(2) + (y2 - y1).abs().pow(2) {
                    graph[i].push(j);
                }

                if r2.pow(2) >= (x2 - x1).abs().pow(2) + (y2 - y1).abs().pow(2) {
                    graph[j].push(i);
                }
            }
        }

        let mut result = 0;

        for i in 0..n {
            let mut visited = vec![false; n];
            let mut st = vec![i];
            let mut cur_result = 0;

            while let Some(cur) = st.pop() {
                if visited[cur] {
                    continue;
                }

                visited[cur] = true;
                cur_result += 1;
                st.extend_from_slice(&graph[cur]);
            }

            result = result.max(cur_result);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2101() {
        // let bombs = nd_vec![[2, 1, 3], [6, 1, 4]];
        // let expected = 2;
        // assert_eq!(Solution::maximum_detonation(bombs), expected);
        // let bombs = nd_vec![[1, 1, 5], [10, 10, 5]];
        // let expected = 1;
        // assert_eq!(Solution::maximum_detonation(bombs), expected);
        // let bombs = nd_vec![[1, 2, 3], [2, 3, 1], [3, 4, 2], [4, 5, 3], [5, 6, 4]];
        // let expected = 5;
        // assert_eq!(Solution::maximum_detonation(bombs), expected);
        let bombs = nd_vec![[1, 1, 100000], [100000, 100000, 1]];
        let expected = 1;
        assert_eq!(Solution::maximum_detonation(bombs), expected);
    }
}
