// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use winreg::RegKey;
use winreg::enums::*;

#[derive(Clone, serde::Serialize)]
struct Payload {
  args: Vec<String>,
  cwd: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> Result<String, String> {
    use std::path::Path;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let path = Path::new("Software").join("WOW6432Node").join("Halo Wing").join("test");
    let (key, _) = hklm.create_subkey(&path).map_err(|e| format!("Failed to create subkey: {:?}", e))?;
    
    key.set_value("Test", &name).map_err(|e| format!("Failed to set value: {:?}", e))?;

    let reg_name: String = key.get_value("test").map_err(|e| format!("Failed to get value: {:?}", e))?;



    Ok(format!("Hello, {}! You've been greeted from Rust!", reg_name))
}

use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent};
use tauri_plugin_autostart::MacosLauncher;

fn main() {
    let tray_menu = SystemTrayMenu::new()
    .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
    .add_item(CustomMenuItem::new("hide".to_string(), "Hide"));

    tauri::Builder::default()
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
            event.window().hide().unwrap();
            api.prevent_close();
            }
            _ => {}
        })
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
              } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
              }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .setup(|app| {
        #[cfg(debug_assertions)] // only include this code on debug builds
        {
          let window = app.get_window("main").unwrap();
          window.open_devtools();
          window.close_devtools();
        }
        Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);

            app.emit_all("single-instance", Payload { args: argv, cwd }).unwrap();
        }))
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--flag1", "--flag2"]) /* arbitrary number of args to pass to your app */))
        .run(tauri::generate_context!())
        .expect("error while running tauri application") 
}
