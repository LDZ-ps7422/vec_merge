use std::fs::File;
use std::io::{BufReader, BufRead};
// use std::sync::{mpsc, mpsc::SyncSender};
use std::thread::{self, JoinHandle};
use crossbeam_channel::{bounded, Sender, Receiver};
use std::time::{Duration, Instant};

use crate::record::Record;
use super::Source;

const QUEUE_SIZE: usize = 700000;

pub struct ThreadSource {
    r: Receiver<Option<Record>>,
}
impl ThreadSource {
    pub fn new(file_path: String) -> Self {
        let (s, r) = bounded::<Option<Record>>(QUEUE_SIZE);
        // let (s, r) = mpsc::sync_channel(QUEUE_SIZE);

        let file_name = file_path;
        let file_open = File::open(& file_name);
        let file = match file_open {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: \"{}\"\nError Info: {:?}", file_name, error),
        };
        let file_cursor = BufReader::new(file);
        ThreadSource::keep_reading(s, file_cursor);
        let csvsource = ThreadSource {
            r,
        };
        csvsource
    }
    fn keep_reading(s: Sender<Option<Record>>, mut cursor: BufReader<File>) -> JoinHandle<()> {
        let producer_thread = thread::spawn(move || {
            let mut load_total: Duration = Duration::new(0, 0);
            let mut send_total: Duration = Duration::new(0, 0);
            let start = Instant::now();
            loop {

                let load_start = Instant::now();
                let data = ThreadSource::load_one_record(&mut cursor);
                let load_end = Instant::now();
                let load_duration = load_end.duration_since(load_start);
                load_total += load_duration;

                if data == None {
                    s.send(None).unwrap();
                    break;
                }
                let send_start = Instant::now();

                s.send(data).unwrap();
                
                let send_end = Instant::now();
                let send_duration = send_end.duration_since(send_start);
                send_total += send_duration;

            }
            let end = Instant::now();
            let duration = end.duration_since(start);
            println!("merge read time : {:?}", duration);
            println!("load time : {:?}", load_total);
            println!("send time : {:?}", send_total);

        });
        producer_thread
    }
    fn load_one_record(cursor: &mut BufReader<File>) -> Option<Record> {
        // 从当前cursor所在位置向后读取一个record并返回
        let mut line = String::new();
        let mut len = cursor.read_line(&mut line);
        while let Ok(2) = len {
            len = cursor.read_line(&mut line);
        }
        let top = match len {
            Ok(0) => None,
            Ok(_) => Some(ThreadSource::csv_line_parser(&line)),
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
            ThreadSource::extract_numbers(parts[2].trim())
        } else { 0 };
    
        Record {id, name, total} // 返回解析后的 Record 结构体
    }
}
impl Source for ThreadSource {
    fn read(&mut self) -> Option<Record> {
        self.r.recv().unwrap()
    }
    fn remove_one(&mut self) {
        panic!("··ERROR: REMOVE_ONE is not a legal func for csvMerger, check code..");
    }
}
