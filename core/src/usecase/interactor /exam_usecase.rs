use crate::service::exam_service::ExamService;

/// # 全ての試験を取得する
pub fn all() -> &'static str {
    let exam_service = ExamService::new(exam_repository);
    return ExamService::all();
}
