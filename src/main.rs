mod middleware;
mod routes;
mod util;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::hello)
            .service(routes::echo)
            .service(routes::timetable)
            .service(routes::timetable_arg)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
