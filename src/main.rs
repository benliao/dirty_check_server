use rustrict::CensorStr;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub result: String,
}

#[get("/dirtycheck/{content}")]
pub async fn dirty_check_handler(path: web::Path<String>) -> impl Responder {

    let content = path.into_inner();

    let inappropriate: bool = content.is_inappropriate();
    println!("{} {}", content, inappropriate);

    let response_json = &GenericResponse {
        status: "success".to_string(),
        result: inappropriate.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("api")
            .service(dirty_check_handler)
        );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .configure(config)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8333))?
    .run()
    .await
    

}
