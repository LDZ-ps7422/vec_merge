use crate::record::Record;

pub mod heap_merger;
pub mod src_merger;


pub trait Merger {
    fn merge_read(&mut self) -> Option<Record>;
}