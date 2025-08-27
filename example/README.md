# Tauri Task Queue 前端使用示例

这个示例展示了如何在Tauri应用的前端使用任务队列插件。

## 功能特性

- 添加任务到队列
- 查看任务列表和状态
- 监听任务进度更新
- 支持不同优先级的任务
- 响应式UI设计
- 实时任务状态更新

## 项目结构

```
frontend/
├── README.md
├── index.html
├── style.css
├── script.js
├── main.rs
├── tauri.conf.json
├── package.json
└── vite.config.js
```

## 前端集成说明

### 1. 初始化任务队列

在Tauri应用的主进程中初始化任务队列：

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

### 2. 前端调用示例

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

### 3. 监听任务事件

监听任务进度和状态更新：

```javascript
// 监听任务事件
window.__TAURI__.event.listen('task_event', (event) => {
  const task = event.payload;
  console.log('任务更新:', task);
  
  // 更新UI显示任务进度
  updateTaskProgress(task.id, task.progress, task.status);
});
```

## 运行示例

### 方法一：使用Cargo运行（推荐）

1. 确保已安装Rust和Tauri开发环境
2. 在项目根目录运行：
   ```bash
   cargo run --example frontend_demo
   ```

### 方法二：使用Tauri CLI运行

1. 安装Tauri CLI：
   ```bash
   cargo install tauri-cli
   ```
2. 在frontend目录下运行：
   ```bash
   cargo tauri dev
   ```

### 方法三：使用Vite进行Web开发

1. 安装依赖：
   ```bash
   npm install
   ```
2. 启动开发服务器：
   ```bash
   npm run dev
   ```

## 构建应用

要构建生产版本的应用：

```bash
# 使用Cargo构建
cargo build --example frontend_demo --release

# 或使用Tauri CLI构建
npm run build
```

## 自定义任务处理器

你可以根据需要注册不同的任务处理器：

```rust
// 注册下载任务处理器
register_frontend_handler(&queue, "download", move |mut task, app| {
    tokio::task::spawn(async move {
        // 实现下载逻辑
    })
});

// 注册处理任务处理器
register_frontend_handler(&queue, "process", move |mut task, app| {
    tokio::task::spawn(async move {
        // 实现处理逻辑
    })
});
```

## 前端API参考

### `enqueue_task`

添加任务到队列。

**参数：**
- `taskType`: 任务类型字符串
- `params`: 任务参数（JSON对象）
- `priority`: 任务优先级（0=高，1=中，2=低）

### `list_tasks`

获取当前所有任务的列表。

**返回值：**
- 任务数组，包含所有任务的详细信息

### `task_event`

任务状态更新事件。

**事件数据：**
- 任务对象，包含更新后的任务信息