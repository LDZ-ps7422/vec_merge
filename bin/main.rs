use vec_merge::{funcs::get_page_info, merger::{heap_merger::HeapMerger, Merger}, source::csv::CsvSource};
use std::env;

use vec_merge::source::Source;


fn main() {

    let file_names: Vec<String> = env::args().skip(1).collect();
    let mut sources: Vec<Box<dyn Source>> = Vec::new();

    for file_name in &file_names {
        let file_path = format!("{}.csv", file_name);
        sources.push(Box::new(CsvSource::new(file_path)) as Box<dyn Source>);
    }
    // let merger = SourceMerger{sources};
    // let merger = HeapMerger::new(sources);

    let page_size = 7;
    let result_pages = get_page_info(page_size, Box::new(HeapMerger::new(sources)) as Box<dyn Merger>);

    for (idx, page) in result_pages.iter().enumerate() {
        println!("Page {}: Start Key: {}, End Key: {}, Total: {}, Count: {}",
                 idx + 1, page.start_key, page.end_key, page.total, page.count);
    }
}
