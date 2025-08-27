use tauri_plugin_task_queue_lib::{Task, TaskOrigin, TaskPriority, TaskStatus, AppState};
use tauri_plugin_task_queue_lib::task::TaskQueue;
use serde_json::Value;
use tauri::{Manager, Emitter};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_task_queue_lib::init())
        .setup(|app| {
            // 获取插件提供的任务队列
            let state = app.state::<AppState>();
            let queue = state.queue.lock().unwrap().clone().unwrap();

            // 注册示例任务处理器
            queue.register_handler("download", move |mut task, app_handle| {
                // 实现下载任务逻辑
                tokio::task::spawn(async move {
                    task.status = TaskStatus::Running;
                    // 发送任务状态更新事件
                    let _ = app_handle.emit("task_event", &task);

                    // 模拟下载过程
                    let url = task.params["url"].as_str().unwrap_or_default();
                    println!("开始下载: {}", url);

                    // 模拟下载进度
                    for i in 0..=100 {
                        task.progress = i;
                        // 发送进度更新事件
                        let _ = app_handle.emit("task_event", &task);
                        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    }

                    task.status = TaskStatus::Completed;
                    // 发送完成事件
                    let _ = app_handle.emit("task_event", &task);
                    println!("下载完成: {}", url);
                })
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![enqueue_task, list_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// 从前端添加任务的命令
#[tauri::command]
async fn enqueue_task(
    app: tauri::AppHandle,
    task_type: String,
    params: Option<Value>,
    priority: Option<u8>,
) -> Result<String, String> {
    // 获取任务队列
    let state = app.state::<AppState>();
    let queue = state.queue.lock().unwrap().clone().unwrap();
    
    // 创建任务
    let task = queue.enqueue(
        &task_type,
        TaskOrigin::Frontend,
        params.unwrap_or(Value::Null),
        match priority.unwrap_or(1) {
            0 => TaskPriority::High,
            2 => TaskPriority::Low,
            _ => TaskPriority::Medium,
        }
    );
    
    Ok(task.id)
}

/// 列出所有任务的命令
#[tauri::command]
async fn list_tasks(app: tauri::AppHandle) -> Result<Vec<Task>, String> {
    // 获取任务队列
    let state = app.state::<AppState>();
    let queue = state.queue.lock().unwrap().clone().unwrap();
    
    // 获取任务列表
    let tasks = queue.list();
    Ok(tasks)
}