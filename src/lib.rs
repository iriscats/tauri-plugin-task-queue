pub mod task;
pub use task::{Task, TaskStatus, TaskOrigin, TaskPriority, TaskHandler};
mod commands;

use std::sync::Mutex;
use tauri::{Manager, plugin::TauriPlugin};

pub struct AppState {
    pub queue: Mutex<Option<task::TaskQueue>>,
}

pub struct Builder {
    handlers: std::collections::HashMap<String, Box<task::TaskHandler>>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            handlers: std::collections::HashMap::new(),
        }
    }

    pub fn add_handler<F>(mut self, task_type: &str, handler: F) -> Self
    where
        F: Fn(Task, tauri::AppHandle<tauri::Wry>) -> tokio::task::JoinHandle<()> + Send + Sync + 'static,
    {
        self.handlers.insert(task_type.to_string(), Box::new(handler));
        self
    }

    pub fn build(self) -> TauriPlugin<tauri::Wry> {
        tauri::plugin::Builder::new("task-queue")
            .setup(move |app, _api| {
                let mut queue = task::TaskQueue::new(app.app_handle().clone());

                for (task_type, handler) in self.handlers {
                    queue.register_handler(&task_type, handler);
                }

                app.manage(AppState {
                    queue: Mutex::new(Some(queue)),
                });

                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                commands::add_task,
                commands::get_all_tasks
            ])
            .build()
    }
}


/// 插件初始化入口
pub fn init() -> Builder {
    Builder::new()
}