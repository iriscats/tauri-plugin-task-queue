# Tauri Plugin Task Queue

A task queue plugin for Tauri applications that supports priority-based task scheduling and processing for both frontend and backend tasks.

## Features

- Task queue management
- Priority-based task scheduling
- Frontend and backend task handling
- Task status tracking
- Task progress reporting

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
tauri-plugin-task-queue = { git = "https://github.com/iriscats/tauri-plugin-task-queue" }
```

## Usage

### Basic Setup

Initialize the task queue in your `main.rs`:

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
                        // Simulate task execution
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

### Frontend Usage

You can interact with the task queue from the frontend using Tauri commands:

```javascript
import { invoke } from '@tauri-apps/api';

// Add a task to the queue
await invoke('add_task', {
  taskType: 'my_task_type',
  params: { /* task parameters */ },
  priority: 1  // 0: High, 1: Medium, 2: Low
});

// Get all tasks
await invoke('get_all_tasks');
```

## API Reference

### `init()`

Initializes the task queue plugin.

### `Builder.add_handler(task_type, handler)`

Registers a task handler.

**Arguments:**
- `task_type`: A string representing the task type.
- `handler`: The task handler function, which takes a `Task` and an `AppHandle` as arguments and returns a `tokio::task::JoinHandle<()>`.

### `Builder.build()`

Builds the Tauri plugin.

### Tauri Commands

- `add_task(taskType: string, params: object, priority?: number)`: Adds a task to the queue.
- `get_all_tasks()`: Retrieves a list of all tasks.

## Frontend Example

The project includes a complete frontend example in the `examples/frontend` directory.

### Example Features

- Add tasks to the queue
- View task list and statuses
- Listen for task progress updates
- Support for different task priorities

### Running the Example

1. Ensure you have the Tauri development environment set up.
2. Run `cargo tauri dev` in the project root.
3. Once the application starts, you can add and manage tasks.

## License

MIT