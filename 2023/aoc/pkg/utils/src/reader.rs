use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub struct FileReader {
    reader: BufReader<File>,
}

// implementation of file reader
impl FileReader {
    // create a new file reader
    pub fn new(file_name: &str) -> FileReader {
        let file = File::open(file_name).unwrap();
        FileReader {
            reader: BufReader::new(file),
        }
    }

    pub fn next(&mut self) -> Option<String> {
        let mut line = String::new();
        let bytes = self.reader.read_line(&mut line);
        if bytes.is_ok_and(|x| x > 1) {
            Some(line.trim().to_string())
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        let _ = self.reader.seek(std::io::SeekFrom::Start(0));
    }
}