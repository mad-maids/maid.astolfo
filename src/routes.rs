use crate::middleware;

use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/timetable/")]
pub async fn timetable() -> impl Responder {
    let tt = middleware::timetable("".to_owned());
    HttpResponse::Ok().content_type("application/json").body(tt)
}

#[get("/timetable/{path}")]
pub async fn timetable_arg(path: web::Path<String>) -> impl Responder {
    let tt = middleware::timetable(path.to_string());
    HttpResponse::Ok().content_type("application/json").body(tt)
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
