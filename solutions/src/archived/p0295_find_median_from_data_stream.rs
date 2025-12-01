///
/// # 295. Find Median from Data Stream
///
/// The **median** is the middle value in an ordered integer list. If the size of the list is even, there is no middle value, and the median is the mean of the two middle values.
///
/// * For example, for `arr = [2,3,4]`, the median is `3`.
/// * For example, for `arr = [2,3]`, the median is `(2 + 3) / 2 = 2.5`.
///
/// Implement the MedianFinder class:
///
/// * `MedianFinder()` initializes the `MedianFinder` object.
/// * `void addNum(int num)` adds the integer `num` from the data stream to the data structure.
/// * `double findMedian()` returns the median of all elements so far. Answers within `10<sup>-5</sup>` of the actual answer will be accepted.
///
/// **Example 1:**
///
/// ```
/// Input
/// ["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"]
/// [[], [1], [2], [], [3], []]
/// Output
/// [null, null, null, 1.5, null, 2.0]
///
/// Explanation
/// MedianFinder medianFinder = new MedianFinder();
/// medianFinder.addNum(1);    // arr = [1]
/// medianFinder.addNum(2);    // arr = [1, 2]
/// medianFinder.findMedian(); // return 1.5 (i.e., (1 + 2) / 2)
/// medianFinder.addNum(3);    // arr[1, 2, 3]
/// medianFinder.findMedian(); // return 2.0
///
/// ```
///
/// **Constraints:**
///
/// * `-10<sup>5</sup> <= num <= 10<sup>5</sup>`
/// * There will be at least one element in the data structure before calling `findMedian`.
/// * At most `5 * 10<sup>4</sup>` calls will be made to `addNum` and `findMedian`.
///
/// **Follow up:**
///
/// * If all integer numbers from the stream are in the range `[0, 100]`, how would you optimize your solution?
/// * If `99%` of all integer numbers from the stream are in the range `[0, 100]`, how would you optimize your solution?
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-median-from-data-stream/
// discuss: https://leetcode.com/problems/find-median-from-data-stream/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

use std::{cmp::Reverse, collections::BinaryHeap};

struct MedianFinder {
    median: (Option<i32>, Option<i32>),
    left_heap: BinaryHeap<i32>,
    right_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            median: (None, None),
            left_heap: BinaryHeap::new(),
            right_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        let mut num = num;

        if let Some(left_max) = self.left_heap.peek() {
            if num < *left_max {
                self.left_heap.push(num);
                num = self.left_heap.pop().unwrap();
            }
        }

        if let Some(Reverse(right_min)) = self.right_heap.peek() {
            if num > *right_min {
                self.right_heap.push(Reverse(num));
                Reverse(num) = self.right_heap.pop().unwrap();
            }
        }

        match self.median {
            (None, None) => self.median = (Some(num), None),
            (None, Some(x)) | (Some(x), None) => {
                self.median = (Some(x), Some(num));
            }
            (Some(x), Some(y)) => {
                let mut list = [x, y, num];
                list.sort();

                self.left_heap.push(list[0]);
                self.right_heap.push(Reverse(list[2]));

                self.median = (Some(list[1]), None);
            }
        }
    }

    fn find_median(&self) -> f64 {
        match self.median {
            (None, None) => panic!(),
            (None, Some(x)) | (Some(x), None) => x as f64,
            (Some(x), Some(y)) => (x + y) as f64 / 2.0,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_295() {
        let mut obj = MedianFinder::new();
        obj.add_num(1);
        obj.add_num(2);
        assert_eq!(obj.find_median(), 1.5);
        obj.add_num(3);
        assert_eq!(obj.find_median(), 2.0);
    }
}
