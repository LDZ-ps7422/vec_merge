use crate::merger::Record;
use crate::source::Source;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

use super::Merger;

#[derive(Debug, PartialEq, Eq)]
struct HeapData {
    // value: String,
    value: Record,
    index: usize,
}
impl Ord for HeapData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.id.cmp(&other.value.id).reverse()
    }
}
impl PartialOrd for HeapData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


pub struct HeapMerger {
    sources: Vec<Box<dyn Source>>,
    heap_tree: BinaryHeap<HeapData>,
}
impl HeapMerger {
    pub fn new(mut sources: Vec<Box<dyn Source>>) -> Self {
        let mut buf_heap: BinaryHeap<HeapData> = BinaryHeap::new();
        for (idx, source) in sources.iter_mut().enumerate() {
            if let Some(record) = source.read() {
                buf_heap.push(HeapData{
                    value: record,
                    index: idx,
                })
            }
        }
        let heapmerger = HeapMerger{
            sources,
            heap_tree: buf_heap,
        };
        heapmerger
    }
}
impl Merger for HeapMerger {
    fn merge_read(&mut self) -> Option<Record> {
        let mut _min_record: Option<Record> = None;

        if let Some(min_data) = self.heap_tree.pop() {
            _min_record = Some(min_data.value);

            if let Some(value) = self.sources[min_data.index].read() {
                let index = min_data.index;
                self.heap_tree.push(HeapData{value, index});
            }
            // self.sources[min_data.index].remove_one();
            _min_record
        } else {
            None
        }
    }
}
