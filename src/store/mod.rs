use std::sync::RwLock;
use std::collections::HashMap;
use crate::models::{Task, CreateTask, UpdateTask};

#[derive(Default)]
pub struct Store {
    tasks: RwLock<HashMap<u64, Task>>,
    next_id: RwLock<u64>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            tasks: RwLock::new(HashMap::new()),
            next_id: RwLock::new(1),
        }
    }

    pub async fn create_task(&self, create_task: CreateTask) -> u64 {
        let mut next_id = self.next_id.write().unwrap();
        let id = *next_id;
        *next_id += 1;

        let task = Task {
            id,
            title: create_task.title,
            description: create_task.description,
            completed: false,
        };

        self.tasks.write().unwrap().insert(id, task);
        id
    }


    /// Retrieves a task with the given ID.
    ///
    /// Returns `None` if the task with the given `id` does not exist.
    pub async fn get_task(&self, id: u64) -> Option<Task> {
        self.tasks.read().unwrap().get(&id).cloned()
    }
    /// Updates a task with the given details.
    ///
    /// This function updates a task with the given title, description, and
    /// completed status. If the value for any of these fields is `None`, the
    /// corresponding field on the task will not be updated.
    ///
    /// Returns `None` if the task with the given `id` does not exist.

    pub async fn update_task(&self, id: u64, update: UpdateTask) -> Option<Task> {
        let mut tasks = self.tasks.write().unwrap();
        if let Some(task) = tasks.get_mut(&id) {
            if let Some(title) = update.title {
                task.title = title;
            }
            if let Some(description) = update.description {
                task.description = description;
            }
            if let Some(completed) = update.completed {
                task.completed = completed;
            }
            Some(task.clone())
        } else {
            None
        }
    }

    pub async fn delete_task(&self, id: u64) -> Option<Task> {
        self.tasks.write().unwrap().remove(&id)
    }

    pub async fn list_tasks(&self) -> Vec<Task> {
        self.tasks.read().unwrap().values().cloned().collect()
    }
}

#[cfg(test)]
mod tests;
