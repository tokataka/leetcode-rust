///
/// # 1233. Remove Sub-Folders from the Filesystem
///
/// Given a list of folders `folder`, return *the folders after removing all **sub-folders** in those folders*. You may return the answer in **any order**.
///
/// If a `folder[i]` is located within another `folder[j]`, it is called a **sub-folder** of it. A sub-folder of `folder[j]` must start with `folder[j]`, followed by a `"/"`. For example, `"/a/b"` is a sub-folder of `"/a"`, but `"/b"` is not a sub-folder of `"/a/b/c"`.
///
/// The format of a path is one or more concatenated strings of the form: `'/'` followed by one or more lowercase English letters.
///
/// * For example, `"/leetcode"` and `"/leetcode/problems"` are valid paths while an empty string and `"/"` are not.
///
/// **Example 1:**
///
/// ```
/// Input: folder = ["/a","/a/b","/c/d","/c/d/e","/c/f"]
/// Output: ["/a","/c/d","/c/f"]
/// Explanation: Folders "/a/b" is a subfolder of "/a" and "/c/d/e" is inside of folder "/c/d" in our filesystem.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: folder = ["/a","/a/b/c","/a/b/d"]
/// Output: ["/a"]
/// Explanation: Folders "/a/b/c" and "/a/b/d" will be removed because they are subfolders of "/a".
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: folder = ["/a/b/c","/a/b/ca","/a/b/d"]
/// Output: ["/a/b/c","/a/b/ca","/a/b/d"]
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= folder.length <= 4 * 10<sup>4</sup>`
/// * `2 <= folder[i].length <= 100`
/// * `folder[i]` contains only lowercase letters and `'/'`.
/// * `folder[i]` always starts with the character `'/'`.
/// * Each folder name is **unique**.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-sub-folders-from-the-filesystem/
// discuss: https://leetcode.com/problems/remove-sub-folders-from-the-filesystem/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

use std::collections::HashMap;

struct Node<'a> {
    is_terminate: bool,
    next: HashMap<&'a str, Node<'a>>,
}

impl<'a> Node<'a> {
    fn new() -> Self {
        Self {
            is_terminate: false,
            next: HashMap::new(),
        }
    }
}

impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        folder.sort_unstable();

        let mut result = vec![];

        let mut root = Node::new();

        'outer: for path in &folder {
            let mut cur = &mut root;

            for f in path.split('/').skip(1) {
                cur = cur.next.entry(f).or_insert(Node::new());

                if cur.is_terminate {
                    continue 'outer;
                }
            }

            cur.is_terminate = true;
            result.push(path.clone());
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1233() {
        let folder = vec_string!["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"];
        let expected = vec_string!["/a", "/c/d", "/c/f"];
        assert_eq!(Solution::remove_subfolders(folder), expected);
        let folder = vec_string!["/a", "/a/b/c", "/a/b/d"];
        let expected = vec_string!["/a"];
        assert_eq!(Solution::remove_subfolders(folder), expected);
        let folder = vec_string!["/a/b/c", "/a/b/ca", "/a/b/d"];
        let expected = vec_string!["/a/b/c", "/a/b/ca", "/a/b/d"];
        assert_eq!(Solution::remove_subfolders(folder), expected);
    }
}
