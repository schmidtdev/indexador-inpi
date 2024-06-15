// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use lopdf::Document;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn handle_pdf(window: tauri::Window) -> Result<String, String> {
    let file = "marcas.pdf";
    let doc = Document::load(file);

    match doc {
        Ok(document) => {
            let pages = document.get_pages();
            
            for (i, _) in pages.iter().enumerate() {
                let page_number = (i + 1) as u32;

                let progress = format!("Página {} de {}", i + 1, pages.len());
                window.emit("progress", &progress).expect("Failed to emit event");

                let text = document.extract_text(&[page_number]);

                for page in text.unwrap_or_default().split("\r\n") {
                    for keyword in fs::read_to_string("keywords.txt").unwrap().lines() {
                        for paragraph in page.split("\n") {
                            if paragraph.to_lowercase().contains(keyword) {
                                let match_info = format!("Página {}:<br>{}<br>", i + 1, paragraph);

                                window.emit("match_found", &match_info).expect("Failed to emit event");
                            }
                        }
                    }
                }
            }
        }
        Err(_) => return Err("Failed to load PDF document.".to_string())
    }
    
    Ok("Busca finalizada.".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![handle_pdf])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
