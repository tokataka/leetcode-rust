use std::collections::HashSet;

///
/// # 1079. Letter Tile Possibilities
///
/// You have `n` `tiles`, where each tile has one letter `tiles[i]` printed on it.
///
/// Return *the number of possible non-empty sequences of letters* you can make using the letters printed on those `tiles`.
///
/// **Example 1:**
///
/// ```
/// Input: tiles = "AAB"
/// Output: 8
/// Explanation: The possible sequences are "A", "B", "AA", "AB", "BA", "AAB", "ABA", "BAA".
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: tiles = "AAABBC"
/// Output: 188
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: tiles = "V"
/// Output: 1
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= tiles.length <= 7`
/// * `tiles` consists of uppercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/letter-tile-possibilities/
// discuss: https://leetcode.com/problems/letter-tile-possibilities/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        // let mut char_count = tiles.chars().fold([0; 26], |mut acc, x| {
        //     acc[x as usize - 'A' as usize] += 1;
        //     acc
        // });

        // fn backtrack(char_count: &mut [i32; 26]) -> i32 {

        //     let mut result = 0;

        //     for ch in 0..26 {
        //         if char_count[ch] == 0 {
        //             continue;
        //         }

        //         char_count[ch] -= 1;

        //         result += 1 + backtrack(char_count);

        //         char_count[ch] += 1;
        //     }

        //     result
        // }

        // backtrack(&mut char_count)

        const FACTORIAL: [i32; 8] = [1, 1, 2, 6, 24, 120, 720, 5040];

        fn backtrack(
            char_count: &mut [i32; 26],
            visited: &mut HashSet<([i32; 26], usize)>,
            tiles: &[char],
            idx: usize,
        ) -> i32 {
            if visited.contains(&(*char_count, idx)) {
                return 0;
            }

            visited.insert((*char_count, idx));

            if idx == tiles.len() {
                return FACTORIAL[char_count.iter().sum::<i32>() as usize]
                    / char_count
                        .iter()
                        .map(|&x| FACTORIAL[x as usize])
                        .product::<i32>();
            }

            let mut result = backtrack(char_count, visited, tiles, idx + 1);

            let ch = tiles[idx] as usize - 'A' as usize;

            char_count[ch] += 1;
            result += backtrack(char_count, visited, tiles, idx + 1);
            char_count[ch] -= 1;

            result
        }

        backtrack(
            &mut [0; 26],
            &mut HashSet::from([([0; 26], tiles.len())]),
            &tiles.chars().collect::<Vec<_>>(),
            0,
        )
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1079() {
        let tiles = "AAB".to_owned();
        let expected = 8;
        assert_eq!(Solution::num_tile_possibilities(tiles), expected);
        let tiles = "AAABBC".to_owned();
        let expected = 188;
        assert_eq!(Solution::num_tile_possibilities(tiles), expected);
        let tiles = "V".to_owned();
        let expected = 1;
        assert_eq!(Solution::num_tile_possibilities(tiles), expected);
    }
}
