# Tauri Plugin Task Queue

这是一个用于 Tauri 应用程序的任务队列插件，支持优先级任务调度和前后端任务处理。

## 功能特性

- 任务队列管理
- 优先级任务调度
- 前后端任务处理
- 任务状态跟踪
- 任务进度报告

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
tauri-plugin-task-queue = { path = "./tauri-plugin-task-queue" }
```

## 使用方法

### 基本设置

在 `main.rs` 中初始化任务队列：

```rust
use tauri_plugin_task_queue::{TaskQueue, register_frontend_handler};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let queue = TaskQueue::new(app.handle().clone());
            
            // 注册任务处理器
            register_frontend_handler(&queue, "download", move |mut task, app| {
                // 实现任务逻辑
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 注册前端回调方法

使用 `register_frontend_handler` 函数可以将前端的回调方法注册到任务处理器中：

```rust
use tauri_plugin_task_queue::{TaskQueue, register_frontend_handler};

// 创建任务队列
let queue = TaskQueue::new(app_handle);

// 注册前端回调方法
register_frontend_handler(&queue, "my_task_type", move |mut task, app| {
    // 在这里实现任务逻辑
    tokio::task::spawn(async move {
        // 任务执行代码
        // 可以通过 app.emit() 发送事件到前端
    })
});
```

### 前端调用

在前端可以通过 Tauri 命令调用任务队列：

```javascript
import { invoke } from '@tauri-apps/api';

// 添加任务到队列
await invoke('enqueue_task', {
  taskType: 'my_task_type',
  params: { /* 任务参数 */ },
  priority: 1  // 0: 高, 1: 中, 2: 低
});
```

## API 参考

### `register_frontend_handler`

注册前端回调方法到任务处理器。

**参数：**
- `queue`: 任务队列实例
- `task_type`: 任务类型
- `handler`: 前端回调方法

### `TaskQueue::new`

创建一个新的任务队列实例。

**参数：**
- `app`: Tauri 应用句柄

### `TaskQueue::enqueue`

将任务加入队列。

**参数：**
- `task_type`: 任务类型
- `origin`: 任务来源
- `params`: 任务参数
- `priority`: 任务优先级

## 前端示例

项目包含一个完整的前端使用示例，位于 `examples/frontend` 目录中。

### 示例功能

- 添加任务到队列
- 查看任务列表和状态
- 监听任务进度更新
- 支持不同优先级的任务

### 运行示例

1. 确保已安装Tauri开发环境
2. 在项目根目录运行：`cargo tauri dev`
3. 应用启动后，可以添加和管理任务

### 前端集成说明

在Tauri应用的主进程中初始化任务队列并注册处理器：

```rust
use tauri_plugin_task_queue::{TaskQueue, register_frontend_handler};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let queue = TaskQueue::new(app.handle().clone());
            
            // 注册前端任务处理器
            register_frontend_handler(&queue, "download", move |mut task, app| {
                // 实现下载任务逻辑
            });
            
            // 将队列存储在应用程序状态中
            app.manage(queue);
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 注册前端可调用的命令
            enqueue_task,
            list_tasks,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

从前端添加任务：

```javascript
// 添加任务
const addTask = async (taskType, params, priority) => {
  try {
    const taskId = await window.__TAURI__.invoke('enqueue_task', {
      taskType,
      params,
      priority
    });
    console.log('任务已添加，ID:', taskId);
    return taskId;
  } catch (error) {
    console.error('添加任务失败:', error);
  }
};

// 获取任务列表
const listTasks = async () => {
  try {
    const tasks = await window.__TAURI__.invoke('list_tasks');
    console.log('当前任务列表:', tasks);
    return tasks;
  } catch (error) {
    console.error('获取任务列表失败:', error);
  }
};
```

监听任务事件：

```javascript
// 监听任务事件
window.__TAURI__.event.listen('task_event', (event) => {
  const task = event.payload;
  console.log('任务更新:', task);
  
  // 更新UI显示任务进度
  updateTaskProgress(task.id, task.progress, task.status);
});
```

## 许可证

MIT