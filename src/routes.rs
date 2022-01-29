use crate::middleware;

use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web::http::StatusCode;

const API: &str = "https://api.maid.uz";

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Found().header("Location", API).finish()
}

#[get("/timetable")]
pub async fn timetable_index() -> impl Responder {
    let tt = middleware::timetable_list("".to_owned());
    HttpResponse::Ok().json(tt)
}

#[get("/timetable/{path}")]
pub async fn timetable_list(path: web::Path<String>) -> impl Responder {
    let tt = middleware::timetable_list(path.to_string());
    HttpResponse::Ok().json(tt)
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
