use crate::domain::repository::exam_repository::ExamRepository;
use crate::service::exam_service::ExamService;

pub trait ExamUsecase<T> {
    fn new(exam_service: T);
    fn all() -> String;
}
