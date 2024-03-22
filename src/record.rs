#[derive(Debug, Default, Clone)]
pub struct Record { // 单个记录-3字段
	pub id: String,
	pub name: String,
	pub total: u32,
}