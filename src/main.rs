mod health;
mod model;
mod routes;
mod supabase;
mod timetable;

extern crate core;
extern crate dotenv;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialiuze dotenv configurations
    dotenv().ok();

    // Check for database things
    health::health();

    // Define the target of host
    let target = format!(
        "{}:{}",
        match env::var("HOST") {
            Ok(host) => host,
            Err(_) => "127.0.0.1".to_owned(),
        },
        match env::var("PORT") {
            Ok(port) => port.to_string(),
            Err(_) => 8080.to_string(),
        }
    )
    .to_owned();

    // Logging the outlet
    println!("Running server on http://{}", &target);

    // Creating the server
    HttpServer::new(|| {
        App::new()
            .service(routes::index)
            .service(routes::timetable)
            .service(routes::timetable_index)
            .service(routes::timetable_list)
    })
    .bind(&target)?
    .run()
    .await
}
