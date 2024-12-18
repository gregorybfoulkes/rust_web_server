use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

impl Task {
/// Creates a new `Task` instance from a `CreateTask` object.
/// 
/// This function assigns a unique ID to the task using an atomic counter and
/// initializes the task with the provided title and description. The task is
/// marked as not completed by default.
/// 
/// # Arguments
/// 
/// * `create_task` - A `CreateTask` object containing the title and description
///   for the new task.
/// 
/// # Returns
/// 
/// A new `Task` instance with a unique ID and the specified title and description.

    pub fn new(create_task: CreateTask) -> Self {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        Task {
            id,
            title: create_task.title,
            description: create_task.description,
            completed: false,
        }
    }

    /// Updates a task with the given details.
    ///
    /// This function updates a task with the given title, description, and
    /// completed status. If the value for any of these fields is `None`, the
    /// corresponding field on the task will not be updated.
    ///
    /// # Arguments
    ///
    /// * `update` - An `UpdateTask` object containing the new title, description,
    ///   and completed status for the task.
    pub fn update(&mut self, update: UpdateTask) {
        if let Some(title) = update.title {
            self.title = title;
        }
        if let Some(description) = update.description {
            self.description = description;
        }
        if let Some(completed) = update.completed {
            self.completed = completed;
        }
    }
}
