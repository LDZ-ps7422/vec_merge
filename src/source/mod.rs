use crate::record::*;

pub mod csv;
pub mod memory;

pub trait Source {  // 读取数据函数trait
    fn read(&mut self) -> Option<Record>;
    fn remove_one(&mut self);
}