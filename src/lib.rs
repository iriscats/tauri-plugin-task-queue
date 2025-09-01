pub mod core;
mod commands;

use tauri::{Manager, plugin::TauriPlugin, Wry};
use core::manager::TaskManager;
use crate::core::task::Task;

pub type TaskHandler = dyn Fn(Task, tauri::AppHandle<Wry>) -> tokio::task::JoinHandle<()> + Send + Sync;

pub struct Builder {
    handlers: std::collections::HashMap<String, Box<TaskHandler>>,
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

    pub fn build(self) -> TauriPlugin<Wry> {
        tauri::plugin::Builder::new("task-queue")
            .setup(move |app, _api| {
                let app_handle = app.app_handle().clone();
                let manager = TaskManager::new(app_handle.clone());

                let handlers = self.handlers;
                app.manage(manager);

                tauri::async_runtime::spawn(async move {
                    let manager = app_handle.state::<TaskManager>();
                    for (task_type, handler) in handlers {
                        manager.register_handler(&task_type, handler).await;
                    }
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

pub fn init() -> Builder {
    Builder::new()
}