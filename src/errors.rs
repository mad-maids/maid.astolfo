use actix_web::http::StatusCode;
use actix_web::HttpResponse;

pub async fn not_found() -> HttpResponse {
  HttpResponse::build(StatusCode::FOUND)
    .append_header(("Location", "https://api.maid.uz/404"))
    .finish()
}
