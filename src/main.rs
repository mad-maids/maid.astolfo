mod dungeon;
mod errors;
mod health;
mod initializer;
mod model;
mod routes;
mod timetable;

extern crate core;

use crate::initializer::{initialize, target};
use actix_web::{middleware, App, HttpServer};
use log::info;

#[cfg(unix)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  initialize();

  // Logging the outlet
  info!("Running server on http://{}", target());

  // Creating the server
  HttpServer::new(|| {
    App::new()
      .wrap(middleware::Logger::default())
      .service(routes::index)
      .service(routes::timetable)
      .service(routes::timetable_index)
      .service(routes::timetable_list)
  })
  .bind(target())?
  .bind_uds("/tmp/astolfo.socket")?
  .run()
  .await
}

#[cfg(not(unix))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  initialize().await;

  // Logging the outlet
  info!("Running server on http://{}", target());

  // Creating the server
  HttpServer::new(|| {
    App::new()
      .wrap(middleware::Logger::default())
      .service(routes::index)
      .service(routes::timetable)
      .service(routes::timetable_index)
      .service(routes::timetable_list)
  })
  .bind(target())?
  .run()
  .await
}
