///
/// # 1948. Delete Duplicate Folders in System
///
/// Due to a bug, there are many duplicate folders in a file system. You are given a 2D array `paths`, where `paths[i]` is an array representing an absolute path to the `i<sup>th</sup>` folder in the file system.
///
/// * For example, `["one", "two", "three"]` represents the path `"/one/two/three"`.
///
/// Two folders (not necessarily on the same level) are **identical** if they contain the **same non-empty** set of identical subfolders and underlying subfolder structure. The folders **do not** need to be at the root level to be identical. If two or more folders are **identical**, then **mark** the folders as well as all their subfolders.
///
/// * For example, folders `"/a"` and `"/b"` in the file structure below are identical. They (as well as their subfolders) should **all** be marked:
///   * `/a`
///   * `/a/x`
///   * `/a/x/y`
///   * `/a/z`
///   * `/b`
///   * `/b/x`
///   * `/b/x/y`
///   * `/b/z`
///
/// * However, if the file structure also included the path `"/b/w"`, then the folders `"/a"` and `"/b"` would not be identical. Note that `"/a/x"` and `"/b/x"` would still be considered identical even with the added folder.
///
/// Once all the identical folders and their subfolders have been marked, the file system will **delete** all of them. The file system only runs the deletion once, so any folders that become identical after the initial deletion are not deleted.
///
/// Return *the 2D array* `ans` *containing the paths of the **remaining** folders after deleting all the marked folders. The paths may be returned in **any** order*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/07/19/lc-dupfolder1.jpg)
///
/// ```
/// Input: paths = [["a"],["c"],["d"],["a","b"],["c","b"],["d","a"]]
/// Output: [["d"],["d","a"]]
/// Explanation: The file structure is as shown.
/// Folders "/a" and "/c" (and their subfolders) are marked for deletion because they both contain an empty
/// folder named "b".
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/07/19/lc-dupfolder2.jpg)
///
/// ```
/// Input: paths = [["a"],["c"],["a","b"],["c","b"],["a","b","x"],["a","b","x","y"],["w"],["w","y"]]
/// Output: [["c"],["c","b"],["a"],["a","b"]]
/// Explanation: The file structure is as shown.
/// Folders "/a/b/x" and "/w" (and their subfolders) are marked for deletion because they both contain an empty folder named "y".
/// Note that folders "/a" and "/c" are identical after the deletion, but they are not deleted because they were not marked beforehand.
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2021/07/19/lc-dupfolder3.jpg)
///
/// ```
/// Input: paths = [["a","b"],["c","d"],["c"],["a"]]
/// Output: [["c"],["c","d"],["a"],["a","b"]]
/// Explanation: All folders are unique in the file system.
/// Note that the returned array can be in a different order as the order does not matter.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= paths.length <= 2 * 10<sup>4</sup>`
/// * `1 <= paths[i].length <= 500`
/// * `1 <= paths[i][j].length <= 10`
/// * `1 <= sum(paths[i][j].length) <= 2 * 10<sup>5</sup>`
/// * `path[i][j]` consists of lowercase English letters.
/// * No two paths lead to the same folder.
/// * For any folder not at the root level, its parent folder will also be in the input.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/delete-duplicate-folders-in-system/
// discuss: https://leetcode.com/problems/delete-duplicate-folders-in-system/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

#[allow(unused_imports)]
use itertools::Itertools;

#[derive(Debug)]
struct Trie {
    idx: usize,
    next: HashMap<String, Trie>,
}

impl Trie {
    fn new() -> Self {
        Self {
            idx: usize::MAX,
            next: HashMap::new(),
        }
    }

    fn insert(&mut self, idx: usize, path: &[String]) {
        let mut cur = self;

        for folder in path {
            cur = cur.next.entry(folder.clone()).or_insert(Trie::new());
        }

        cur.idx = idx;
    }

    fn _build_structure_map(&self, map: &mut HashMap<String, Vec<usize>>) -> String {
        let mut structure_vec = vec![];

        for (k, v) in &self.next {
            let next = v._build_structure_map(map);
            structure_vec.push(format!("({next}/{k})"));
        }

        structure_vec.sort_unstable();

        let structure = structure_vec.join(" ");

        if !structure.is_empty() {
            map.entry(structure.clone()).or_default().push(self.idx);
        }

        structure
    }

    fn _expand_duplicates(&self, duplicates: &mut HashSet<usize>, mut is_duplicate: bool) {
        if duplicates.contains(&self.idx) {
            is_duplicate = true;
        } else if is_duplicate {
            duplicates.insert(self.idx);
        }

        for next in self.next.values() {
            next._expand_duplicates(duplicates, is_duplicate);
        }
    }

    fn find_duplicates(&self) -> Vec<usize> {
        let mut map = HashMap::new();

        self._build_structure_map(&mut map);

        let mut duplicates = map
            .into_values()
            .filter(|x| x.len() > 1)
            .flatten()
            .collect::<HashSet<_>>();

        self._expand_duplicates(&mut duplicates, false);

        duplicates.into_iter().collect::<Vec<_>>()
    }
}

impl Solution {
    pub fn delete_duplicate_folder(mut paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut trie = Trie::new();

        for (idx, path) in paths.iter().enumerate() {
            trie.insert(idx, path);
        }

        let mut duplicates = trie.find_duplicates();

        duplicates.sort_unstable_by_key(|&x| Reverse(x));

        for duplicate in duplicates {
            paths.swap_remove(duplicate);
        }

        paths
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1948() {
        // let paths = nd_vec_string![["a"], ["c"], ["d"], ["a", "b"], ["c", "b"], ["d", "a"]];
        // let expected = nd_vec_string![["d"], ["d", "a"]];
        // assert_eq!(Solution::delete_duplicate_folder(paths), expected);
        // let paths = nd_vec_string![
        //     ["a"],
        //     ["c"],
        //     ["a", "b"],
        //     ["c", "b"],
        //     ["a", "b", "x"],
        //     ["a", "b", "x", "y"],
        //     ["w"],
        //     ["w", "y"]
        // ];
        // let expected = nd_vec_string![["c"], ["c", "b"], ["a"], ["a", "b"]];
        // assert_eq!(Solution::delete_duplicate_folder(paths), expected);
        // let paths = nd_vec_string![["a", "b"], ["c", "d"], ["c"], ["a"]];
        // let expected = nd_vec_string![["c"], ["c", "d"], ["a"], ["a", "b"]];
        // assert_eq!(Solution::delete_duplicate_folder(paths), expected);
        let paths = nd_vec_string![
            ["e"],
            ["e", "e"],
            ["c"],
            ["c", "a"],
            ["c", "a", "e"],
            ["c", "a", "e", "b"],
            ["d"],
            ["d", "d"],
            ["d", "d", "c"],
            ["d", "d", "e"],
            ["d", "d", "e", "e"],
            ["d", "b"],
            ["d", "b", "a"],
            ["d", "b", "a", "e"],
            ["d", "b", "a", "e", "c"],
            ["d", "b", "a", "e", "e"],
            ["d", "b", "b"],
            ["d", "b", "c"],
            ["a"],
            ["b"]
        ];
        let expected = nd_vec_string![
            ["c"],
            ["d"],
            ["a"],
            ["b"],
            ["c", "a"],
            ["d", "d"],
            ["d", "b"],
            ["c", "a", "e"],
            ["d", "d", "c"],
            ["d", "b", "a"],
            ["d", "b", "b"],
            ["d", "b", "c"],
            ["c", "a", "e", "b"],
            ["d", "b", "a", "e"],
            ["d", "b", "a", "e", "c"],
            ["d", "b", "a", "e", "e"]
        ];
        assert_eq!(Solution::delete_duplicate_folder(paths), expected);
    }
}
