use log::{error, info};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;

fn check(path: &str) -> bool {
  Path::new(path).exists()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Asset {
  browser_download_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GitHub {
  assets: Vec<Asset>,
}

static APP_USER_AGENT: &str =
  concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub async fn health() {
  if check("./timetable") {
    info!("Timetable is ok!")
  } else {
    error!("Timetable doesn't exist, downloading from database...");

    // Download and store timetables
    let client = reqwest::Client::builder()
      .user_agent(APP_USER_AGENT)
      .build()
      .expect("Couldn't build the client");
    let target: GitHub = client
      .get("https://api.github.com/repos/mad-maids/maid.table/releases/latest")
      .send()
      .await
      .expect("Problems with Internet connectivity!")
      .json()
      .await
      .expect("Can't convert source into json!");

    let mut dumpfile = tempfile::tempfile().unwrap();
    let resp = reqwest::get(target.assets[0].browser_download_url.to_string())
      .await
      .expect("Problems with Internet connectivity!")
      .bytes()
      .await
      .expect("Can't convert source into bytes!");
    io::copy(&mut resp.as_ref(), &mut dumpfile)
      .expect("Failed to copy content");
    let mut zip = zip::ZipArchive::new(dumpfile).unwrap();

    // Extract files inside folder
    zip.extract("./timetable").expect("Couldn't extract...");
  }
}
