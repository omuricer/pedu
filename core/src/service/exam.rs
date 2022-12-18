use crate::domain::repository;

/// # 全ての試験を取得する
pub fn all() -> &'static str {
    return repository::exam::all();
}
