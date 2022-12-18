use crate::service;

/// # 全ての試験を取得する
pub fn all() -> &'static str {
    return service::exam::all();
}
