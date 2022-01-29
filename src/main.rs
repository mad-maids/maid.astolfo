mod middleware;
mod routes;

// use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running server on http://{}", "127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(routes::hello)
            .service(routes::echo)
            .service(routes::timetable_list)
            .service(routes::timetable_file)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
