use crate::domain::repository::exam_repository::ExamRepository;
use crate::service::exam_service::ExamService;

pub struct ExamServiceImpl<T: ExamRepository> {
    pub exam_repository: T,
}

impl ExamService for ExamServiceImpl<Box<dyn ExamRepository>> {
    /// # 全ての試験を取得する
    fn all(&self) -> String {
        return self.exam_repository.all();
    }
}
