use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

pub mod controller;

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    let app_sate = web::Data::new(AppState {
        app_name: String::from("PEDU"),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_sate.clone())
            .service(controller::getstart::index)
            .service(controller::getstart::echo)
            .route("/hey", web::get().to(controller::getstart::manual_hello))
            .service(
                web::scope("/v1")
                    .route("/index.html", web::get().to(controller::getstart::v1_index)),
            )
            .service(web::resource("exam").route(web::get().to(controller::exam::all)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

pub struct AppState {
    pub app_name: String,
    pub counter: Mutex<i32>,
}
