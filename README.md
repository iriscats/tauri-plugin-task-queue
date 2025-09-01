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
tauri-plugin-task-queue = { path = "." }
```

## 使用方法

### 基本设置

在 `main.rs` 中初始化任务队列：

```rust
use tauri_plugin_task_queue::{init, core::task::{Task, TaskStatus}};
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            init()
                .add_handler("download", |mut task, app_handle| {
                    tokio::spawn(async move {
                        println!("Handling download task: {}", task.id);
                        // 模拟任务执行
                        for i in 0..=100 {
                            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                            task.progress = i;
                            app_handle.emit_all("task_event", task.clone()).unwrap();
                        }
                        task.status = TaskStatus::Completed;
                        app_handle.emit_all("task_event", task).unwrap();
                    })
                })
                .build()
                .setup(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 前端调用

在前端可以通过 Tauri 命令调用任务队列：

```javascript
import { invoke } from '@tauri-apps/api';

// 添加任务到队列
await invoke('add_task', {
  taskType: 'my_task_type',
  params: { /* 任务参数 */ },
  priority: 1  // 0: 高, 1: 中, 2: 低
});

// 获取所有任务
await invoke('get_all_tasks');
```

## API 参考

### `init()`

初始化任务队列插件。

### `Builder.add_handler(task_type, handler)`

注册任务处理器。

**参数：**
- `task_type`: 任务类型字符串。
- `handler`: 任务处理函数，接受 `Task` 和 `AppHandle` 作为参数，返回 `tokio::task::JoinHandle<()>`。

### `Builder.build()`

构建 Tauri 插件。

### Tauri 命令

- `add_task(taskType: string, params: object, priority?: number)`: 添加任务到队列。
- `get_all_tasks()`: 获取所有任务列表。

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

## 许可证

MIT