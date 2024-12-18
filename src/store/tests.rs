use super::*;

#[tokio::test]
async fn test_create_task() {
    let store = Store::new();
    let task = CreateTask {
        title: "Test Task".to_string(),
        description: "Test Description".to_string(),
    };

    let id = store.create_task(task).await;
    assert!(id > 0);

    let created_task = store.get_task(id).await.unwrap();
    assert_eq!(created_task.title, "Test Task");
    assert_eq!(created_task.description, "Test Description");
    assert_eq!(created_task.completed, false);
}

#[tokio::test]
async fn test_get_task() {
    let store = Store::new();
    let task = CreateTask {
        title: "Test Task".to_string(),
        description: "Test Description".to_string(),
    };

    let id = store.create_task(task).await;
    let retrieved_task = store.get_task(id).await;

    assert!(retrieved_task.is_some());
    let task = retrieved_task.unwrap();
    assert_eq!(task.title, "Test Task");
    assert_eq!(task.description, "Test Description");
    assert_eq!(task.completed, false);
}

#[tokio::test]
async fn test_update_task() {
    let store = Store::new();
    let task = CreateTask {
        title: "Test Task".to_string(),
        description: "Test Description".to_string(),
    };

    let id = store.create_task(task).await;
    
    let update = UpdateTask {
        title: Some("Updated Task".to_string()),
        description: Some("Updated Description".to_string()),
        completed: Some(true),
    };

    let updated = store.update_task(id, update).await;
    assert!(updated.is_some());

    let updated_task = updated.unwrap();
    assert_eq!(updated_task.title, "Updated Task");
    assert_eq!(updated_task.description, "Updated Description");
    assert_eq!(updated_task.completed, true);
}

#[tokio::test]
async fn test_delete_task() {
    let store = Store::new();
    let task = CreateTask {
        title: "Test Task".to_string(),
        description: "Test Description".to_string(),
    };

    let id = store.create_task(task).await;
    assert!(store.delete_task(id).await.is_some());
    assert!(store.get_task(id).await.is_none());
}

#[tokio::test]
async fn test_list_tasks() {
    let store = Store::new();
    let task1 = CreateTask {
        title: "Test Task 1".to_string(),
        description: "Test Description 1".to_string(),
    };
    let task2 = CreateTask {
        title: "Test Task 2".to_string(),
        description: "Test Description 2".to_string(),
    };

    store.create_task(task1).await;
    store.create_task(task2).await;

    let tasks = store.list_tasks().await;
    assert_eq!(tasks.len(), 2);
}

#[tokio::test]
async fn test_task_not_found() {
    let store = Store::new();
    assert!(store.get_task(1).await.is_none());
    assert!(store.update_task(1, UpdateTask { 
        title: None, 
        description: None,
        completed: None,
    }).await.is_none());
    assert!(store.delete_task(1).await.is_none());
}
