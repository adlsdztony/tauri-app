// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod structures;

use coursehku::course::CourseMap;
use coursehku::serilize::CourseList;

use std::{path::PathBuf, sync::Mutex};
use structures::Data;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn search(
    conflict: bool,
    semester: String,
    contains: String,
    data: tauri::State<DataPointer>,
) -> CourseList {
    let mut lazy = data.0.lock().unwrap().table.to_lazy();
    if conflict {
        lazy = lazy.no_conflict_with(data.0.lock().unwrap().map.clone());
    }
    if semester != "0" {
        let semester: i8 = semester.parse().unwrap();
        lazy = lazy.semester(semester);
    }
    if !contains.is_empty() {
        lazy = lazy.contains(&[&contains]);
    }
    let map: CourseMap = lazy.collect().unwrap().into();
    map.into()
}

#[tauri::command]
fn courses(data: tauri::State<DataPointer>) -> CourseList {
    data.0.lock().unwrap().map.clone().into()
}

#[tauri::command]
fn clear(data: tauri::State<DataPointer>) -> CourseList {
    data.0.lock().unwrap().map.clear();
    data.0.lock().unwrap().map.clone().into()
}

#[tauri::command]
fn add(code: String, section: String, data: tauri::State<DataPointer>) -> CourseList {
    if section == "all" {
        // get course first to avoid deadlock
        let course = data.0
            .lock()
            .unwrap()
            .table
            .get_course(&code)
            .unwrap()
            .clone();

        data.0.lock().unwrap().map.add(
            code.clone(),
            course,
        );
    } else {
        let course = data.0
            .lock()
            .unwrap()
            .table
            .get_section(&code, &section)
            .unwrap()
            .clone();

        data.0.lock().unwrap().map.add(
            code.clone(),
            course,
        );
    }
    data.0.lock().unwrap().map.clone().into()
}

struct DataPointer(Mutex<Data>);

fn main() {
    tauri::Builder::default()
        .manage(DataPointer(Mutex::new(Data::new(PathBuf::from(
            "data.csv",
        )))))
        .invoke_handler(tauri::generate_handler![greet, search, add, courses, clear])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_add() {
        let data = Data::new(PathBuf::from("data.csv"));
        let data = DataPointer(Mutex::new(data));
        let section = "all";
        let code = "COMP1117";

        if section == "all" {
            let course = data.0
                .lock()
                .unwrap()
                .table
                .get_course(&code)
                .unwrap()
                .clone();
            data.0.lock().unwrap().map.add(
                code.to_string(),
                course,
            );
        } else {
            let course = data.0
                .lock()
                .unwrap()
                .table
                .get_section(&code, &section)
                .unwrap()
                .clone();

            data.0.lock().unwrap().map.add(
                code.to_string(),
                course,
            );
        }
        println!("end {} {}", code, section);
        let course_list = data.0.lock().unwrap().map.clone();
        println!("{:?}", course_list);

    }
}