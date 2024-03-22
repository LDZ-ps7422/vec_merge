use std::cmp::Ordering;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Record { // 单个记录-3字段
	pub id: String,
	pub name: String,
	pub total: u32,
}
impl Ord for Record {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id).reverse()
    }
}
impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}