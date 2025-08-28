const COMMANDS: &[&str] = &["get_all_tasks", "enqueue_task"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}