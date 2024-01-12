use std::{collections::HashMap, path::PathBuf};

use coursehku::course::{CourseMap, CourseTable};

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


