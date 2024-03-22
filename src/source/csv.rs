use std::fs::File;
use std::io::{BufReader, BufRead};

use crate::record::Record;
use super::Source;

struct MemorySource { // 数据源 包含一个记录vector
	records: Vec<Record>,
}
impl Source for MemorySource {   // 数据源的方法
	fn read(&mut self) -> Option<Record> {
		self.records.first().cloned()
	}
    fn remove_one(&mut self) {
        self.records.remove(0);
    }
}

pub struct CsvSource {
    file_cursor: BufReader<File>,
}
impl CsvSource {
    pub fn new(file_path: String) -> Self {
        let file_name = file_path;
        let file_open = File::open(& file_name);
        let file = match file_open {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: \"{}\"\nError Info: {:?}", file_name, error),
        };
        let file_cursor = BufReader::new(file);
        let csvsource = CsvSource {
            file_cursor,
        };
        csvsource
    }
    pub fn load_one_record(cursor: &mut BufReader<File>) -> Option<Record> {
        // 从当前cursor所在位置向后读取一个record并返回
        let mut line = String::new();
        let mut len = cursor.read_line(&mut line);
        while let Ok(2) = len {
            len = cursor.read_line(&mut line);
        }
        let top = match len {
            Ok(0) => None,
            Ok(_) => Some(CsvSource::csv_line_parser(&line)),
            Err(_e) => None,
        };
        top
    }
    fn extract_numbers(s: &str) -> u32 {
        // 从str中提取出所有数字字符，为空填0
        let mut result = 0;
        for c in s.chars() {
            if c.is_digit(10) {
                if let Some(digit) = c.to_digit(10) {
                    result = result * 10 + digit;
                }
            }
        }
        result
    }
    fn csv_line_parser(line: &str) -> Record {
        let parts: Vec<&str> = line.split(',').collect();
    
        let id: String = parts[0].trim().to_string();
    
        let name: String = if parts.len() >= 2 {
            parts[1].trim().to_string()
        } else { "".to_string() };
    
        let total: u32 = if parts.len() >= 3 {
            CsvSource::extract_numbers(parts[2].trim())
        } else { 0 };
    
        Record {id, name, total} // 返回解析后的 Record 结构体
    }
}
impl Source for CsvSource {
    fn read(&mut self) -> Option<Record> {
        CsvSource::load_one_record(&mut self.file_cursor)
    }
    fn remove_one(&mut self) {
        panic!("··ERROR: REMOVE_ONE is not a legal func for csvMerger, check code..");
    }
}
