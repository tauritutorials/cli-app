// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::Value;
use tauri::{api::cli::Matches, App, Manager as _};

struct Name(Option<String>);

#[tauri::command]
fn get_name(state: tauri::State<'_, Name>) -> String {
    state.0.clone().unwrap_or("World!".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_name])
        .setup(|app| {
            match app.get_cli_matches() {
                Ok(matches) => {
                    handle_cli_matches(matches, app);
                }
                Err(_) => {}
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn handle_cli_matches(matches: Matches, app: &mut App) {
    if let Some(subcommand) = matches.subcommand {
        match subcommand.name.as_str() {
            "greet" => {
                let name = subcommand
                    .matches
                    .args
                    .get("name")
                    .and_then(|arg| match &arg.value {
                        Value::String(name) => Some(name.clone()),
                        _ => None,
                    });

                app.manage(Name(name));
            }
            _ => {
                std::process::exit(0);
            }
        }
    } else {
        std::process::exit(0);
    }
}
