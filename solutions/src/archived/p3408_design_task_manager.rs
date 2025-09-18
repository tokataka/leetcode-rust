///
/// # 3408. Design Task Manager
///
/// There is a task management system that allows users to manage their tasks, each associated with a priority. The system should efficiently handle adding, modifying, executing, and removing tasks.
///
/// Implement the `TaskManager` class:
///
/// * `TaskManager(vector<vector<int>>& tasks)` initializes the task manager with a list of user-task-priority triples. Each element in the input list is of the form `[userId, taskId, priority]`, which adds a task to the specified user with the given priority.
///
/// * `void add(int userId, int taskId, int priority)` adds a task with the specified `taskId` and `priority` to the user with `userId`. It is **guaranteed** that `taskId` does not *exist* in the system.
///
/// * `void edit(int taskId, int newPriority)` updates the priority of the existing `taskId` to `newPriority`. It is **guaranteed** that `taskId` *exists* in the system.
///
/// * `void rmv(int taskId)` removes the task identified by `taskId` from the system. It is **guaranteed** that `taskId` *exists* in the system.
///
/// * `int execTop()` executes the task with the **highest** priority across all users. If there are multiple tasks with the same **highest** priority, execute the one with the highest `taskId`. After executing, the `taskId` is **removed** from the system. Return the `userId` associated with the executed task. If no tasks are available, return -1.
///
/// **Note** that a user may be assigned multiple tasks.
///
/// **Example 1:**
///
/// **Input:**
/// ["TaskManager", "add", "edit", "execTop", "rmv", "add", "execTop"]
/// [[[[1, 101, 10], [2, 102, 20], [3, 103, 15]]], [4, 104, 5], [102, 8], [], [101], [5, 105, 15], []]
///
/// **Output:**
/// [null, null, null, 3, null, null, 5]
///
/// **Explanation**
///
/// TaskManager taskManager = new TaskManager([[1, 101, 10], [2, 102, 20], [3, 103, 15]]); // Initializes with three tasks for Users 1, 2, and 3.
/// taskManager.add(4, 104, 5); // Adds task 104 with priority 5 for User 4.
/// taskManager.edit(102, 8); // Updates priority of task 102 to 8.
/// taskManager.execTop(); // return 3. Executes task 103 for User 3.
/// taskManager.rmv(101); // Removes task 101 from the system.
/// taskManager.add(5, 105, 15); // Adds task 105 with priority 15 for User 5.
/// taskManager.execTop(); // return 5. Executes task 105 for User 5.
///
/// **Constraints:**
///
/// * `1 <= tasks.length <= 10<sup>5</sup>`
/// * `0 <= userId <= 10<sup>5</sup>`
/// * `0 <= taskId <= 10<sup>5</sup>`
/// * `0 <= priority <= 10<sup>9</sup>`
/// * `0 <= newPriority <= 10<sup>9</sup>`
/// * At most `2 * 10<sup>5</sup>` calls will be made in **total** to `add`, `edit`, `rmv`, and `execTop` methods.
/// * The input is generated such that `taskId` will be valid.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/design-task-manager/
// discuss: https://leetcode.com/problems/design-task-manager/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::BinaryHeap;

#[allow(unused_imports)]
use itertools::Itertools;

struct TaskManager {
    tasks_pq: BinaryHeap<(i32, usize)>,
    latest_priority: Vec<i32>,
    task_user: Vec<i32>,
}

impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut tasks_pq = BinaryHeap::new();
        let mut latest_priority = vec![0; 100001];
        let mut task_user = vec![0; 100001];

        for task in tasks {
            let (user_id, task_id, priority) = (task[0], task[1] as usize, task[2]);
            tasks_pq.push((priority, task_id));
            latest_priority[task_id] = priority;
            task_user[task_id] = user_id;
        }

        Self {
            tasks_pq,
            latest_priority,
            task_user,
        }
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.tasks_pq.push((priority, task_id as usize));
        self.latest_priority[task_id as usize] = priority;
        self.task_user[task_id as usize] = user_id;
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        self.tasks_pq.push((new_priority, task_id as usize));
        self.latest_priority[task_id as usize] = new_priority;
    }

    fn rmv(&mut self, task_id: i32) {
        self.latest_priority[task_id as usize] = -1;
    }

    fn exec_top(&mut self) -> i32 {
        while let Some((priority, task_id)) = self.tasks_pq.pop() {
            if self.latest_priority[task_id] == priority {
                self.latest_priority[task_id] = -1;
                return self.task_user[task_id];
            }
        }

        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3408() {
        let mut obj = TaskManager::new(nd_vec![[1, 101, 10], [2, 102, 20], [3, 103, 15]]);
        obj.add(4, 104, 5);
        obj.edit(102, 8);
        assert_eq!(obj.exec_top(), 3);
        obj.rmv(101);
        obj.add(5, 105, 15);
        assert_eq!(obj.exec_top(), 5);
    }
}
