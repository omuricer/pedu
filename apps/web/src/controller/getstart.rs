use crate::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/")]
pub async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Hello {app_name}");
    format!("Request number: {counter}")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn v1_index() -> impl Responder {
    "Hello world!"
}
