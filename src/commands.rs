use crate::core::task::{Task, TaskOrigin, TaskPriority};
use crate::core::manager::TaskManager;
use serde_json::Value;
use tauri::State;

#[tauri::command]
pub async fn add_task(
    manager: State<'_, TaskManager>,
    task_type: String,
    params: Value,
    priority: Option<u8>,
) -> Result<String, String> {
    let prio = match priority.unwrap_or(1) {
        0 => TaskPriority::High,
        2 => TaskPriority::Low,
        _ => TaskPriority::Medium,
    };

    let task = Task::new(&task_type, TaskOrigin::Backend, params, prio);
    let task_id = task.id.clone();
    manager.add_task(task).await.map_err(|e| e.to_string())?;
    
    println!("Task added with ID: {}", task_id);
    
    Ok(task_id)
}

/// 列出所有任务的命令
#[tauri::command]
pub async fn get_all_tasks(manager: State<'_, TaskManager>) -> Result<Vec<Task>, String> {
    Ok(manager.get_all_tasks().await)
}