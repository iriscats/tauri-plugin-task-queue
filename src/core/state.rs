use crate::core::task::Task;
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct TaskQueueState {
    pub tasks: Mutex<HashMap<String, Task>>,
}