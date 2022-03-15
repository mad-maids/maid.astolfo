use crate::middlewares as tt;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
  HttpResponse::Found()
    .append_header(("Location", "https://api.maid.uz"))
    .finish()
}

#[get("/{module}")]
pub async fn module_api(path: web::Path<(String)>) -> impl Responder {
  let (module) = path.into_inner();
  HttpResponse::Found()
    .append_header((
      "Location",
      format!("https://api.maid.uz/docs/node/{}", module),
    ))
    .finish()
}

#[get("/{module}/")]
pub async fn module_index(path: web::Path<(String)>) -> impl Responder {
  let (module) = path.into_inner();
  let tt = tt::json_list(module, "".to_owned());
  HttpResponse::Ok().json(tt)
}

#[get("/{module}/{path}")]
pub async fn module_list(path: web::Path<(String, String)>) -> impl Responder {
  let (module, path) = path.into_inner();
  let tt = tt::json_view(module, path);
  HttpResponse::Ok().json(tt)
}
