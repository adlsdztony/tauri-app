use std::{collections::HashMap, path::PathBuf};

use coursehku::course::{Course, CourseMap, CourseTable};

pub struct Data {
    pub table: CourseTable,
    pub map: CourseMap,
}

impl Data {
    pub fn new(file_path: PathBuf) -> Data {
        Data {
            table: CourseTable::load(file_path),
            map: CourseMap::new(HashMap::new()),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct Query {
    pub conflict: bool,
    pub semester: i8,
    pub contains: String,
}

#[derive(serde::Serialize)]
pub struct CourseList {
    courses: Vec<Course>,
}

impl From<CourseMap> for CourseList {
    fn from(map: CourseMap) -> Self {
        let mut courses = Vec::new();
        for course in map.values() {
            courses.push(course.clone());
        }

        CourseList { courses }
    }
}
