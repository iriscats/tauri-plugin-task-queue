use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cmp::Ordering;

/// 任务状态枚举，表示任务可能处于的不同状态
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
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
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskOrigin {
    /// 任务来自前端
    Frontend,
    /// 任务来自后端
    Backend,
}

/// 任务优先级枚举，用于确定任务执行的优先顺序
/// 数字越小优先级越高
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub enum TaskPriority {
    /// 高优先级
    High = 0,
    /// 中优先级
    Medium = 1,
    /// 低优先级
    Low = 2,
}

/// 任务结构体，包含任务的所有相关信息
#[derive(Clone, Debug, Serialize, Deserialize, Eq)]
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