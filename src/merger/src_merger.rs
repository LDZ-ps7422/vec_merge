use crate::merger::Record;
use crate::source::Source;

use super::Merger;

struct SourceMerger {
    sources: Vec<Box<dyn Source>>,
}
impl Merger for SourceMerger {
    fn merge_read(&mut self) -> Option<Record> {
        let mut min_record: Option<Record> = None;
        let mut min_record_source_index: Option<usize> = None;
    
        let mut empty = true;
    
        for (idx, source) in self.sources.iter_mut().enumerate() {
            if let Some(record) = source.read() {
                match min_record {
                    Some(min) if record.id < min.id => {
                        min_record = Some(record.clone());
                        min_record_source_index = Some(idx);
                    }
                    None => {
                        min_record = Some(record.clone());
                        min_record_source_index = Some(idx);
                    }
                    _ => {}
                }
                empty = false;
            }
        }
    
        if empty {
            return None;
        }
        if let Some(index) = min_record_source_index {
            if let Some(source) = self.sources.get_mut(index) {
                source.remove_one();
            }
        }
    
        min_record
    }
}
