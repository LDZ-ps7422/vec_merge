use crate::record::Record;
use std::cmp::Ordering;

pub mod heap_merger;
pub mod src_merger;


pub trait Merger {
    fn merge_read(&mut self) -> Option<Record>;
}


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