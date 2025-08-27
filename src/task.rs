use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;

/// 任务状态枚举，表示任务可能处于的不同状态
#[derive(Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    /// 任务待处理
    Pending,
    /// 任务正在运行
    Running,
    /// 任务已完成
    Completed,
    /// 任务失败，并附带错误信息
    Failed(String),
    /// 任务已取消
    Canceled,
}

/// 任务来源枚举，标识任务是由前端还是后端发起
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskOrigin {
    /// 任务来自前端
    Frontend,
    /// 任务来自后端
    Backend,
}

/// 任务优先级枚举，用于确定任务执行的优先顺序
/// 数字越小优先级越高
#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub enum TaskPriority {
    /// 高优先级
    High = 0,
    /// 中优先级
    Medium = 1,
    /// 低优先级
    Low = 2,
}

/// 任务结构体，包含任务的所有相关信息
#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    /// 任务唯一标识符
    pub id: String,
    /// 任务类型，用于匹配相应的处理器
    pub task_type: String,
    /// 任务来源（前端或后端）
    pub origin: TaskOrigin,
    /// 任务当前状态
    pub status: TaskStatus,
    /// 任务进度百分比 (0-100)
    pub progress: u8,
    /// 任务参数，使用JSON格式存储
    pub params: Value,
    /// 任务优先级
    pub priority: TaskPriority,
}

impl Task {
    /// 创建一个新的任务实例
    /// 
    /// # 参数
    /// * `task_type` - 任务类型字符串
    /// * `origin` - 任务来源
    /// * `params` - 任务参数
    /// * `priority` - 任务优先级
    /// 
    /// # 返回值
    /// 返回新创建的Task实例
    pub fn new(task_type: &str, origin: TaskOrigin, params: Value, priority: TaskPriority) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            task_type: task_type.to_string(),
            origin,
            status: TaskStatus::Pending,
            progress: 0,
            params,
            priority,
        }
    }
}

/// 为Task实现Ord trait，使其可以按优先级排序
/// 优先级数字越小，优先级越高
impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

/// 为Task实现PartialOrd trait
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// 为Task实现PartialEq trait，通过比较任务ID来判断任务是否相等
impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// 为Task实现Eq trait
impl Eq for Task {}

/// 任务处理器类型定义
/// 接受一个Task和AppHandle，返回一个tokio任务句柄
pub type TaskHandler = dyn Fn(Task, tauri::AppHandle<tauri::Wry>) -> tokio::task::JoinHandle<()> + Send + Sync;

/// 任务队列结构体，用于管理任务的入队、执行和处理
#[derive(Clone)]
pub struct TaskQueue {
    /// 用于发送任务的通道发送端
    sender: mpsc::Sender<Task>,
    /// 存储任务的优先队列
    tasks: Arc<Mutex<BinaryHeap<Task>>>,
    /// 存储任务类型与处理器映射关系的哈希表
    handlers: Arc<Mutex<HashMap<String, Arc<TaskHandler>>>>,
}

impl TaskQueue {
    /// 创建一个新的任务队列实例
    /// 
    /// # 参数
    /// * `app` - Tauri应用句柄
    /// 
    /// # 返回值
    /// 返回新创建的TaskQueue实例
    pub fn new(app: tauri::AppHandle<tauri::Wry>) -> Self {
        let (tx, mut rx) = mpsc::channel::<Task>(100);
        let tasks = Arc::new(Mutex::new(BinaryHeap::new()));
        let handlers = Arc::new(Mutex::new(HashMap::<String, Arc<TaskHandler>>::new()));

        let tasks_clone = tasks.clone();
        let handlers_clone = handlers.clone();
        let app_handle = app.clone();

        tauri::async_runtime::spawn(async move {
            while let Some(task) = rx.recv().await {
                {
                    let mut locked = tasks_clone.lock().unwrap();
                    locked.push(task.clone());
                }

                // Execute the task
                if let Some(handler) = handlers_clone.lock().unwrap().get(&task.task_type) {
                    let handler = handler.clone();
                    let app_handle_clone = app_handle.clone();
                    handler(task, app_handle_clone);
                }

                let maybe_task = {
                    let mut locked = tasks_clone.lock().unwrap();
                    locked.pop()
                };

                if let Some(task) = maybe_task {
                    let handler_opt = {
                        let map = handlers_clone.lock().unwrap();
                        map.get(&task.task_type).cloned()
                    };

                    if let Some(handler) = handler_opt {
                        handler(task, app.clone());
                    } else {
                        eprintln!("No handler registered for task {}", task.task_type);
                    }
                }
            }
        });

        Self { sender: tx, tasks, handlers }
    }

    /// 将任务加入队列
    /// 
    /// # 参数
    /// * `task_type` - 任务类型
    /// * `origin` - 任务来源
    /// * `params` - 任务参数
    /// * `priority` - 任务优先级
    /// 
    /// # 返回值
    /// 返回创建的任务实例
    pub fn enqueue(&self, task_type: &str, origin: TaskOrigin, params: Value, priority: TaskPriority) -> Task {
        let task = Task::new(task_type, origin, params, priority);
        let _ = self.sender.try_send(task.clone());
        task
    }

    /// 注册任务处理器
    /// 
    /// # 参数
    /// * `task_type` - 任务类型
    /// * `handler` - 任务处理器函数
    pub fn register_handler<F>(&self, task_type: &str, handler: F)
    where
        F: Fn(Task, tauri::AppHandle<tauri::Wry>) -> tokio::task::JoinHandle<()> + Send + Sync + 'static,
    {
        self.handlers
            .lock()
            .unwrap()
            .insert(task_type.to_string(), Arc::new(handler));
    }

    /// 获取所有任务列表
    /// 
    /// # 返回值
    /// 返回当前所有任务的向量
    pub fn list(&self) -> Vec<Task> {
        let locked = self.tasks.lock().unwrap();
        locked.clone().into_sorted_vec()
    }
}