#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use kernel::sleep;

#[tauri::command]
fn basic() {
  println!("I was invoked from JS!");
}

#[tauri::command]
fn hello_world(name: String) -> Result<String, String> {
  println!("I was invoked from JS! Hello: {}", name);
  Ok("Hello from Rust!".into())
}

#[tauri::command]
async fn async_command() -> Result<String, String> {
  sleep(1000).await;
  println!("hello");
  Ok("Finished".into())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![basic, hello_world, async_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
