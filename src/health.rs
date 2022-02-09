use log::{error, info};
use std::path::Path; // trace, warn

fn check(path: &str) -> bool {
  Path::new(path).exists()
}

pub fn health() -> () {
  if check("./timetable") {
    info!("Timetable is ok!")
  } else {
    error!("Timetable doesn't exist")
  }
}
