// Handler for basic endpoints (root and health)
pub mod basic;

// Handler for task-related endpoints
pub mod tasks;

// Re-export handlers
pub use basic::*;
pub use tasks::*;

#[cfg(test)]
mod tests {
    mod basic_tests;
    mod tasks_tests;
}
