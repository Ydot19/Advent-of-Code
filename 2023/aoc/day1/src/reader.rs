#![allow(dead_code)]

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use crate::calibration;

// struct file reader with a buffer reader
pub(crate) struct FileReader {
    reader: BufReader<File>,
}

// implementation of file reader
impl FileReader {
    // create a new file reader
    pub(crate) fn new(file_name: &str) -> FileReader {
        let file = File::open(file_name).unwrap();
        FileReader {
            reader: BufReader::new(file),
        }
    }

    // read a line from the file
    pub(crate) fn read_line_calibration_value(&mut self) -> Option<i32> {
        let mut line = String::new();
        let bytes = self.reader.read_line(&mut line);
        if bytes.is_ok() {
            calibration::calibration_value(line.as_str())
        } else {
            None
        }
    }
}

