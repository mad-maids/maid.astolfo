use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::format;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

fn check(path: &str) -> bool {
  Path::new(path).exists()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Asset {
  name: String,
  browser_download_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GitHub {
  id: u32,
  tag_name: String,
  assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
  id: u32,
  version: String,
  modules: Vec<String>,
}

fn module_extract(modules: Vec<Asset>) -> Vec<String> {
  return modules
    .iter()
    .map(|module| module.name.replace(".zip", ""))
    .collect();
}

static APP_USER_AGENT: &str =
  concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub async fn health() {
  // Get info about latest builds
  let client = reqwest::Client::builder()
    .user_agent(APP_USER_AGENT)
    .build()
    .expect("Couldn't build the client");

  let target: GitHub = client
    .get("https://api.github.com/repos/mad-maids/maid.data/releases/latest")
    .send()
    .await
    .expect("Problems with Internet connectivity!")
    .json()
    .await
    .expect("Can't convert source into json!");

  if check("./config.json") {
    let cfg: String = fs::read_to_string("./config.json").unwrap();

    let parsed: Config = serde_json::from_str(cfg.as_str())
      .expect("Couldn't parse the config file");

    if parsed.id == target.id {
      info!("Assets are up to date, let's check health of modules");

      let raw = Config {
        id: target.id,
        version: target.tag_name,
        modules: module_extract(target.assets.clone()),
      };

      let config = serde_json::to_string_pretty(&raw).unwrap();
      let mut file = File::create("config.json").unwrap();
      file.write_all(config.as_bytes()).unwrap();

      match check_modules(module_extract(target.assets)).await {
        Ok(..) => println!("Yes"),
        Err(e) => println!("{}", e),
      }
    }

    if parsed.id != target.id {
      warn!("Assets are outdated! Need quick update...");
      // update(target.assets).await;
    }
  } else {
    error!("No config detected!");
    // update(target.assets).await
  }
}

async fn check_modules(modules: Vec<String>) -> Result<(), &'static str> {
  let task = for module in modules {
    if check(format!("./{}", module).as_str()) {
      info!("Module [{}] is ok", module);
    } else {
      warn!("The module [{}] is missing...", module);
      return Err("");
    }
  };
  Ok(task)
}

// async fn update(modules: Vec<Asset>) {
//
//   for file in target.assets {
//     let mut dumpfile = tempfile::tempfile().unwrap();
//
//     let resp = reqwest::get(file.browser_download_url.to_string())
//       .await
//       .expect("Problems with Internet connectivity!")
//       .bytes()
//       .await
//       .expect("Can't convert source into bytes!");
//     io::copy(&mut resp.as_ref(), &mut dumpfile)
//       .expect("Failed to copy content");
//     let mut zip = zip::ZipArchive::new(dumpfile).unwrap();
//
//     // Extract files inside folder
//     zip
//       .extract(file.name.replace(".zip", ""))
//       .expect("Couldn't extract...");
//   }
// }
