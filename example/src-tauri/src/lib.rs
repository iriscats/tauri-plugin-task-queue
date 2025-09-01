use tauri_plugin_task_queue::{self};
use tauri::{Emitter, Manager};
use tauri_plugin_task_queue::core::task::{Task, TaskStatus};

pub fn run() {
    let plugin = tauri_plugin_task_queue::init().add_handler(
        "download",
        move |mut task: Task, app_handle| {
            tokio::task::spawn(async move {
                task.status = TaskStatus::Running;
                let _ = app_handle.emit("task_event", &task);

                let url = task.params["url"].as_str().unwrap_or_default();
                println!("开始下载: {}", url);

                for i in 0..=100 {
                    task.progress = i;
                    let _ = app_handle.emit("task_event", &task);
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                }

                task.status = TaskStatus::Completed;
                let _ = app_handle.emit("task_event", &task);
                println!("下载完成: {}", url);
            })
        },
    );

    tauri::Builder::default()
        .plugin(plugin.build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}