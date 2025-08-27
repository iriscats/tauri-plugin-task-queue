pub mod task;
pub use task::{Task, TaskStatus, TaskOrigin, TaskPriority, TaskHandler};
mod commands;

use std::sync::Mutex;
use tauri::{Manager, plugin::TauriPlugin};

pub struct AppState {
    pub queue: Mutex<Option<task::TaskQueue>>,
}

/// 注册前端回调方法到任务处理器
/// 
/// # 参数
/// * `queue` - 任务队列实例
/// * `task_type` - 任务类型
/// * `handler` - 前端回调方法
pub fn register_frontend_handler<F>(app: &mut tauri::App<tauri::Wry>, task_type: &str, handler: F)
where
    F: Fn(Task, tauri::AppHandle<tauri::Wry>) -> tokio::task::JoinHandle<()> + Send + Sync + 'static,
{
    let state = app.state::<AppState>();
    let queue_option = state.queue.lock().unwrap();
    if let Some(queue) = queue_option.as_ref() {
        queue.register_handler(task_type, handler);
    }
}

/// 插件初始化入口
pub fn init() -> TauriPlugin<tauri::Wry> {
    tauri::plugin::Builder::new("tauri-plugin-task-queue")
        .setup(|app, _api| {
            let queue = task::TaskQueue::new(app.app_handle().clone());

            // 将队列存储在应用程序状态中
            app.manage(AppState {
                queue: Mutex::new(Some(queue)),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::add_backend_task,
            commands::enqueue_task,
            commands::get_all_tasks
        ])
        .build()
}