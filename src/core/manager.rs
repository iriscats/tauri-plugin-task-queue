use crate::core::error::Result;
use crate::core::task::{Task, TaskStatus};
use std::collections::{BinaryHeap, HashMap};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Wry};
use tokio::sync::{mpsc, Mutex};
use crate::TaskHandler;

pub struct TaskManager {
    tasks: Arc<Mutex<HashMap<String, Task>>>,
    #[allow(dead_code)]
    pending_queue: Arc<Mutex<BinaryHeap<Task>>>,
    app_handle: AppHandle<Wry>,
    sender: mpsc::Sender<Task>,
    handlers: Arc<Mutex<HashMap<String, Arc<TaskHandler>>>>,
}

impl TaskManager {
    pub fn new(app_handle: AppHandle<Wry>) -> Self {
        let (tx, mut rx) = mpsc::channel::<Task>(100);
        let tasks = Arc::new(Mutex::new(HashMap::new()));
        let pending_queue = Arc::new(Mutex::new(BinaryHeap::new()));
        let handlers = Arc::new(Mutex::new(HashMap::<String, Arc<TaskHandler>>::new()));

        let tasks_receiver_clone = tasks.clone();
        let pending_queue_receiver_clone = pending_queue.clone();

        // Task receiver loop
        tauri::async_runtime::spawn(async move {
            while let Some(task) = rx.recv().await {
                tasks_receiver_clone
                    .lock()
                    .await
                    .insert(task.id.clone(), task.clone());
                pending_queue_receiver_clone.lock().await.push(task);
            }
        });

        let tasks_processor_clone = tasks.clone();
        let pending_queue_processor_clone = pending_queue.clone();
        let handlers_clone = handlers.clone();
        let app_handle_clone = app_handle.clone();

        // Task processor loop
        tauri::async_runtime::spawn(async move {
            loop {
                let task_to_run = {
                    let mut pending = pending_queue_processor_clone.lock().await;
                    pending.pop()
                };

                if let Some(mut task_to_run) = task_to_run {
                    if let Some(handler) = handlers_clone.lock().await.get(&task_to_run.task_type) {
                        task_to_run.status = TaskStatus::Running;
                        tasks_processor_clone
                            .lock()
                            .await
                            .insert(task_to_run.id.clone(), task_to_run.clone());

                        let task_id = task_to_run.id.clone();
                        let handler_clone = handler.clone();
                        let app_handle_for_handler = app_handle_clone.clone();
                        let tasks_for_removal = tasks_processor_clone.clone();

                        tauri::async_runtime::spawn(async move {
                            let handle = handler_clone(task_to_run, app_handle_for_handler);
                            let _ = handle.await;
                            tasks_for_removal.lock().await.remove(&task_id);
                        });
                    } else {
                        eprintln!("No handler registered for task {}", task_to_run.task_type);
                    }
                } else {
                    // Wait for a bit if the queue is empty
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                }
            }
        });

        Self {
            tasks,
            pending_queue,
            app_handle,
            sender: tx,
            handlers,
        }
    }

    pub async fn add_task(&self, task: Task) -> Result<()> {
        let _ = self.sender.try_send(task.clone());
        Ok(())
    }

    pub async fn get_task(&self, id: &str) -> Option<Task> {
        let tasks = self.tasks.lock().await;
        tasks.get(id).cloned()
    }

    pub async fn get_all_tasks(&self) -> Vec<Task> {
        let tasks = self.tasks.lock().await;
        tasks.values().cloned().collect()
    }

    pub async fn update_task_status(&self, id: &str, status: TaskStatus) -> Result<()> {
        let mut tasks = self.tasks.lock().await;
        if let Some(task) = tasks.get_mut(id) {
            task.status = status;
            self.app_handle
                .emit("task_event", task.clone())
                .unwrap();
            Ok(())
        } else {
            Err(crate::core::error::Error::TaskNotFound)
        }
    }

    pub async fn register_handler<F>(&self, task_type: &str, handler: F)
    where
        F: Fn(Task, tauri::AppHandle<Wry>) -> tokio::task::JoinHandle<()>
            + Send
            + Sync
            + 'static,
    {
        self.handlers
            .lock()
            .await
            .insert(task_type.to_string(), Arc::new(handler));
    }
}