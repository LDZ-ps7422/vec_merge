use vec_merge::{get_page_info::get_page_info, merger::{heap_merger::HeapMerger, Merger}, source::{csv::CsvSource, thread_src::ThreadSource}};
use std::env;
use std::time::Instant;


use vec_merge::source::Source;


fn main() {

    // let file_names: Vec<String> = env::args().skip(1).collect();
    let file_num: u32 = env::args().nth(1).unwrap().parse().expect("Please provide a valid number");
    let mut sources: Vec<Box<dyn Source>> = Vec::new();

    for i in 1..=file_num {
        let file_path = format!("./file20x100w/data{}.csv", i);
        // sources.push(Box::new(CsvSource::new(file_path)) as Box<dyn Source>);
        sources.push(Box::new(ThreadSource::new(file_path)) as Box<dyn Source>);
    }

    // println!("Code is moving outside ..");
    let page_size = 1000000;

    // 开始计时
    let start_time = Instant::now();

    let result_pages = get_page_info(page_size, Box::new(HeapMerger::new(sources)) as Box<dyn Merger>);

    // 结束计时
    let end_time = Instant::now();

    // 计算代码块的运行时间
    let duration = end_time.duration_since(start_time);

    for (idx, page) in result_pages.iter().enumerate() {
        println!("Page {}: Start Key: {}, End Key: {}, Total: {}, Count: {}",
                 idx + 1, page.start_key, page.end_key, page.total, page.count);
    }
    
    println!("Running time: {:?}", duration);
}
