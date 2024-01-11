// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod structures;

use coursehku::course::CourseMap;
use std::{path::PathBuf, sync::Mutex};
use structures::{CourseList, Data};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn search(
    conflict: bool,
    semester: i8,
    contains: String,
    data: tauri::State<DataPointer>,
) -> CourseList {
    let mut lazy = data.0.lock().unwrap().table.to_lazy();
    if conflict {
        lazy = lazy.no_conflict_with(data.0.lock().unwrap().map.clone());
    }
    if semester != 0 {
        lazy = lazy.semester(semester);
    }
    if !contains.is_empty() {
        lazy = lazy.contains(&[&contains]);
    }
    let map: CourseMap = lazy.collect().unwrap().into();
    map.into()
}

#[tauri::command]
fn add(code: String, section: String, data: tauri::State<DataPointer>) {
    if section == "all" {
        data.0.lock().unwrap().map.add(
            code.clone(),
            data.0
                .lock()
                .unwrap()
                .table
                .get_course(&code)
                .unwrap()
                .clone(),
        );
    } else {
        data.0.lock().unwrap().map.add(
            code.clone(),
            data.0
                .lock()
                .unwrap()
                .table
                .get_section(&code, &section)
                .unwrap()
                .clone(),
        );
    }
}

struct DataPointer(Mutex<Data>);

fn main() {
    tauri::Builder::default()
        .manage(DataPointer(Mutex::new(Data::new(PathBuf::from(
            "data.csv",
        )))))
        .invoke_handler(tauri::generate_handler![greet, search, add])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
