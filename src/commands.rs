use crate::{Task, TaskOrigin, TaskPriority};
use crate::AppState;
use serde_json::Value;
use tauri::State;

#[tauri::command]
pub fn add_backend_task(
    state: State<'_, AppState>,
    task_type: String,
    params: Value,
    priority: Option<u8>,
) -> Result<String, String> {
    let queue = state.queue.lock().unwrap();
    let _queue_ref = queue.as_ref().ok_or("Queue not initialized")?;

    let prio = match priority.unwrap_or(1) {
        0 => TaskPriority::High,
        2 => TaskPriority::Low,
        _ => TaskPriority::Medium,
    };

    // For now, return a placeholder since we can't use async here
    let task = Task::new(&task_type, TaskOrigin::Backend, params, prio);
    Ok(task.id)
}

/// 前端添加任务的命令
#[tauri::command]
pub fn enqueue_task(
    state: State<'_, AppState>,
    task_type: String,
    params: Value,
    priority: Option<u8>,
) -> Result<String, String> {
    let queue = state.queue.lock().unwrap();
    let _queue_ref = queue.as_ref().ok_or("Queue not initialized")?;

    let prio = match priority.unwrap_or(1) {
        0 => TaskPriority::High,
        2 => TaskPriority::Low,
        _ => TaskPriority::Medium,
    };

    // For now, return a placeholder since we can't use async here
    let task = Task::new(&task_type, TaskOrigin::Frontend, params, prio);
    Ok(task.id)
}

/// 列出所有任务的命令
#[tauri::command]
pub fn get_all_tasks(state: State<'_, AppState>) -> Result<Vec<Task>, String> {
    let queue = state.queue.lock().unwrap();
    let queue_ref = queue.as_ref().ok_or("Queue not initialized")?;
    
    Ok(queue_ref.list())
}