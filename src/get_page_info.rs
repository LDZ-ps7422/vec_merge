use crate::merger::Merger;



pub struct PageInfo {   // 分页信息
	pub start_key: String,
	pub end_key: String,
	pub total: u32,
	pub count: u32,
}


pub fn get_page_info(page_size: u32, mut merger: Box<dyn Merger>) -> Vec<PageInfo> {
    let mut result_pages: Vec<PageInfo> = Vec::new();
    let mut cur_index = 0;
    let mut cur_total = 0;
    let mut count = 0;
    let mut start_key: String = String::new();
    let mut end_key: String = String::new();

    while let Some(opr_record) = merger.merge_read() {

        // println!("{}\t{}\t {}", opr_record.id, opr_record.name, opr_record.total);

        end_key = opr_record.id.clone();
        count += 1;
        cur_total += opr_record.total;
        if (cur_index + 1) % page_size == 1 {
            start_key = opr_record.id.clone();
        } else if (cur_index + 1) % page_size == 0 {
            let page_info = PageInfo {
                start_key: start_key.clone(),
                end_key: end_key.clone(),
                total: cur_total,
                count,
            };
            result_pages.push(page_info);
            count = 0;
            cur_total = 0;
        }
        cur_index += 1;


    }

    if count > 0 { // 添加最后一个 PageInfo 对象
        let page_info = PageInfo {
            start_key: start_key.clone(),
            end_key: end_key.clone(),
            total: cur_total,
            count,
        };
        result_pages.push(page_info);
    }

    result_pages
}
