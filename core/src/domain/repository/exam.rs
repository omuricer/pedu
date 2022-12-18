use in_mysql::repository::exam as exam_impl;

/// # 全ての試験を取得する
pub fn all() -> &'static str {
    return exam_impl::all();
}
