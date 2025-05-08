use std::collections::BTreeMap;

///
/// # 2071. Maximum Number of Tasks You Can Assign
///
/// You have `n` tasks and `m` workers. Each task has a strength requirement stored in a **0-indexed** integer array `tasks`, with the `i<sup>th</sup>` task requiring `tasks[i]` strength to complete. The strength of each worker is stored in a **0-indexed** integer array `workers`, with the `j<sup>th</sup>` worker having `workers[j]` strength. Each worker can only be assigned to a **single** task and must have a strength **greater than or equal** to the task's strength requirement (i.e., `workers[j] >= tasks[i]`).
///
/// Additionally, you have `pills` magical pills that will **increase a worker's strength** by `strength`. You can decide which workers receive the magical pills, however, you may only give each worker **at most one** magical pill.
///
/// Given the **0-indexed** integer arrays `tasks` and `workers` and the integers `pills` and `strength`, return *the **maximum** number of tasks that can be completed.*
///
/// **Example 1:**
///
/// ```
/// Input: tasks = [3,2,1], workers = [0,3,3], pills = 1, strength = 1
/// Output: 3
/// Explanation:
/// We can assign the magical pill and tasks as follows:
/// - Give the magical pill to worker 0.
/// - Assign worker 0 to task 2 (0 + 1 >= 1)
/// - Assign worker 1 to task 1 (3 >= 2)
/// - Assign worker 2 to task 0 (3 >= 3)
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: tasks = [5,4], workers = [0,0,0], pills = 1, strength = 5
/// Output: 1
/// Explanation:
/// We can assign the magical pill and tasks as follows:
/// - Give the magical pill to worker 0.
/// - Assign worker 0 to task 0 (0 + 5 >= 5)
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: tasks = [10,15,30], workers = [0,10,10,10,10], pills = 3, strength = 10
/// Output: 2
/// Explanation:
/// We can assign the magical pills and tasks as follows:
/// - Give the magical pill to worker 0 and worker 1.
/// - Assign worker 0 to task 0 (0 + 10 >= 10)
/// - Assign worker 1 to task 1 (10 + 10 >= 15)
/// The last pill is not given because it will not make any worker strong enough for the last task.
///
/// ```
///
/// **Constraints:**
///
/// * `n == tasks.length`
/// * `m == workers.length`
/// * `1 <= n, m <= 5 * 10<sup>4</sup>`
/// * `0 <= pills <= m`
/// * `0 <= tasks[i], workers[j], strength <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-tasks-you-can-assign/
// discuss: https://leetcode.com/problems/maximum-number-of-tasks-you-can-assign/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_task_assign(
        mut tasks: Vec<i32>,
        mut workers: Vec<i32>,
        pills: i32,
        strength: i32,
    ) -> i32 {
        tasks.sort_unstable();
        workers.sort_unstable();

        (1..=tasks.len()).collect::<Vec<_>>().partition_point(|&t| {
            if workers.len() < t {
                return false;
            }

            let mut workers: BTreeMap<i32, i32> =
                workers
                    .iter()
                    .skip(workers.len() - t)
                    .fold(BTreeMap::new(), |mut acc, &x| {
                        *acc.entry(x).or_default() += 1;
                        acc
                    });

            let mut pills_remain = pills;

            for &task in tasks.iter().take(t).rev() {
                if let Some((&worker, count)) = workers.iter_mut().last() {
                    if worker >= task {
                        *count -= 1;
                        if *count == 0 {
                            workers.remove(&worker);
                        }

                        continue;
                    }
                }

                if pills_remain == 0 {
                    return false;
                }

                match workers.range_mut(task - strength..).next() {
                    Some((&worker, count)) => {
                        *count -= 1;
                        if *count == 0 {
                            workers.remove(&worker);
                        }

                        pills_remain -= 1;
                    }
                    None => {
                        return false;
                    }
                }
            }

            true
        }) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2071() {
        // let tasks = vec![3, 2, 1];
        // let workers = vec![0, 3, 3];
        // let pills = 1;
        // let strength = 1;
        // let expected = 3;
        // assert_eq!(
        //     Solution::max_task_assign(tasks, workers, pills, strength),
        //     expected
        // );
        // let tasks = vec![5, 4];
        // let workers = vec![0, 0, 0];
        // let pills = 1;
        // let strength = 5;
        // let expected = 1;
        // assert_eq!(
        //     Solution::max_task_assign(tasks, workers, pills, strength),
        //     expected
        // );
        // let tasks = vec![10, 15, 30];
        // let workers = vec![0, 10, 10, 10, 10];
        // let pills = 3;
        // let strength = 10;
        // let expected = 2;
        // assert_eq!(
        //     Solution::max_task_assign(tasks, workers, pills, strength),
        //     expected
        // );
        let tasks = vec![
            1943, 2068, 4077, 7832, 8061, 6939, 6263, 8917, 8008, 5348, 8837, 4753, 4607, 7638,
            9000, 7222, 4552, 1123, 9225, 6896, 4154, 6303, 3186, 2325, 9994, 5855, 8851, 7377,
            1930, 1187, 5094, 2689, 8852, 1507, 1567, 9575, 1193, 1557, 8840, 9075, 5032, 3642,
            6917, 7968, 5310, 2315, 7516, 4776, 3091, 7027, 1788, 2007, 2651, 6112, 4264, 5644,
            3585, 9408, 7410, 9605, 8151, 1538, 6905, 6759, 4518, 3444, 5036, 1589, 3902, 3037,
            1468, 9179, 3000, 5339, 6805, 7394, 9418, 9262, 2888, 4708, 3402, 5554, 8714, 7393,
            2848, 5946, 9808, 4301, 6675, 8564, 6300, 4359, 9506, 1946, 9673, 7412, 1164, 2986,
            2198, 5144, 3763, 4782, 8835, 6994, 8035, 3332, 2342, 5243, 3150, 9084, 6519, 9798,
            7682, 9919, 7473, 7686, 9978, 8092, 9897, 3985, 9874, 5842, 9740, 2145, 2426, 7069,
            8963, 9250, 4142, 9434, 1895, 6559, 3233, 8431, 6278, 6748, 7305, 4359, 2144, 8009,
            4890, 6486, 7464, 8645, 1704, 5915, 9586, 1394, 7504, 2124, 3150, 2051, 5026, 7612,
            3715, 5757, 4355, 6394, 3202, 2777, 3949, 2349, 7398, 3029, 3081, 5116, 5078, 8048,
            9934, 4348, 8518, 5201, 1203, 7935, 5006, 6388, 8680, 3427, 6048, 1957, 4026, 4618,
            4080,
        ];
        let workers = vec![
            875, 2347, 939, 3664, 3926, 4555, 1947, 4406, 4601, 3502, 4964, 1307, 4232, 2968, 4572,
            3139, 2788, 1847, 1208, 2019, 4184, 1664, 1747, 3690, 4333, 891, 686, 1959, 2218, 4972,
            806, 741, 1490, 4529, 2909, 925, 2040, 1234, 1264, 1135, 3640, 1455, 2933, 3699, 2856,
            3074, 4579, 2458, 2090, 833, 4140, 4534, 2336, 4363, 1948, 4546, 4155, 3735, 3577,
            2780, 4874, 1747, 4844, 3482, 3053, 3534, 549, 4500, 2237, 2128, 1554, 3210, 4161,
            2211, 950, 3732, 2182, 1148, 4368, 4050, 1452, 1015, 3192, 4318, 3908, 2590, 1103,
            2811, 2821, 690, 2718, 3360, 2659, 3315, 579, 3108, 2979, 3903, 4367, 1906, 4964, 889,
            4803, 825, 2270, 4794, 4825, 4485, 4461, 1639, 3857, 1330, 3169, 2425, 3694, 1980,
            2268, 3002, 2177, 3225, 2499, 2517, 1916, 2844, 760, 2167, 1786, 3179, 3222, 1432,
            3775, 4747, 1764, 690, 3223, 4684, 890, 2701, 1045, 3034, 1381, 1011, 2150, 4798, 2247,
            1334, 3058, 934, 2895, 1484, 2784, 3341, 4412, 1748, 625, 2610, 3488, 4810, 669, 4275,
            4929, 1014, 2104, 3111,
        ];
        let pills = 122;
        let strength = 3131;
        let expected = 143;
        assert_eq!(
            Solution::max_task_assign(tasks, workers, pills, strength),
            expected
        );
    }
}
