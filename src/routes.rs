use crate::middleware;

use actix_web::{get, post, HttpResponse, Responder, web};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/timetable")]
pub async fn timetable() -> impl Responder {
    let tt = middleware::timetable("".to_owned());
    HttpResponse::Ok().body(tt)
}

#[get("/timetable/{path}")]
pub async fn timetable_arg(path: web::Path<String>) -> impl Responder {
    println!("Path is: {}", path);
    let tt = middleware::timetable(path.to_string());
    HttpResponse::Ok().body(tt)
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}