mod util;
mod routes;
mod middleware;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(routes::hello)
            .service(routes::echo)
            .service(routes::timetable)
            .service(routes::timetable_arg)
            // .route("/hey", web::get().to(routes::manual_hello))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}