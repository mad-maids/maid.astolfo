use crate::health;
use dotenv::dotenv;
use std::env;

pub async fn initialize() {
  // Initialize dotenv configurations
  dotenv().ok();

  let is_dev = match env::var("ISDEV") {
    Ok(host) => host,
    Err(_) => "no".to_owned(),
  };

  if is_dev == "yes" {
    println!("Server has been started in Debug mode!");
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();
  } else {
    println!("Server has been started in Production mode!");
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
  }

  // Check for database things
  health::health().await
}

pub fn target() -> String {
  // Define the target of host
  let target = format!(
    "{}:{}",
    match env::var("HOST") {
      Ok(host) => host,
      Err(_) => "127.0.0.1".to_owned(),
    },
    match env::var("PORT") {
      Ok(port) => port,
      Err(_) => 9000.to_string(),
    }
  );
  target
}
