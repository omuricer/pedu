use actix_web::{HttpResponse, Responder};

use in_mysql::repository::exam_repository::ExamRepositoryImpl;
use pedu::service::r#impl::exam_service::ExamServiceImpl;
use pedu::usecase::exam_usecase::ExamUsecase;

pub async fn all() -> impl Responder {
    let exam_repository = ExamRepositoryImpl {};
    let exam_service = ExamServiceImpl {
        exam_repository: exam_repository,
    };
    let exam_usecase = ExamUsecase::new(exam_service);
    let exams = exam_usecase.all();
    HttpResponse::Ok().body(exams)
}
