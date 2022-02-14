mod health;
mod model;
mod routes;
mod timetable;

extern crate core;
extern crate dotenv;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use log::info;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // Initialize dotenv configurations
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
      Err(_) => 9000.to_string(),
    }
  )
  .to_owned();

  // Logging the outlet
  info!("Running server on http://{}", &target);

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
