use crate::timetable as tt;

use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
  HttpResponse::Found()
    .header("Location", "https://api.maid.uz")
    .finish()
}

#[get("/favicon")]
async fn favicon() -> actix_web::Result<actix_files::NamedFile> {
  Ok(actix_files::NamedFile::open("favicon.ico")?)
}

#[get("/timetable")]
pub async fn timetable() -> impl Responder {
  HttpResponse::Found()
    .header("Location", "https://api.maid.uz/docs/deno/soon")
    .finish()
}

#[get("/timetable/")]
pub async fn timetable_index() -> impl Responder {
  let tt = tt::timetable_list("".to_owned());
  HttpResponse::Ok().json(tt)
}

#[get("/timetable/{path}")]
pub async fn timetable_list(path: web::Path<String>) -> impl Responder {
  let tt = tt::timetable_view(path.to_string());
  HttpResponse::Ok().json(tt)
}
