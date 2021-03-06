mod errors;
mod health;
mod initializer;
mod middlewares;
mod model;
mod routes;

extern crate core;

use crate::initializer::{initialize, target};
use actix_web::{middleware, web, App, HttpServer};

#[cfg(unix)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  initialize().await;

  // Logging the outlet
  println!("Running server on http://{}", target());

  // Creating the server
  HttpServer::new(|| {
    App::new()
      .wrap(middleware::Logger::default())
      .service(routes::index)
      .service(routes::module_api)
      .service(routes::module_index)
      .service(routes::module_list)
      .default_service(web::route().to(errors::not_found))
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
  println!("Running server on http://{}", target());

  // Creating the server
  HttpServer::new(|| {
    App::new()
      .wrap(middleware::Logger::default())
      .service(routes::index)
      .service(routes::module_api)
      .service(routes::module_index)
      .service(routes::module_list)
      .default_service(web::route().to(errors::not_found))
  })
  .bind(target())?
  .run()
  .await
}
