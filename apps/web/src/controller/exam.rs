use actix_web::{HttpResponse, Responder};
use pedu::usecase;

pub async fn all() -> impl Responder {
    let exams = usecase::exam::all();
    HttpResponse::Ok().body(exams)
}
