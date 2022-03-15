use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::{fs, io};

pub fn json_list(module: String, location: String) -> Value {
  let inner_location: String = format!("./{}/{}", module, location);
  let entries = fs::read_dir(&inner_location)
    .unwrap()
    .map(|res| {
      res.map(|e| {
        e.path()
          .file_name()
          .unwrap()
          .to_str()
          .unwrap()
          .to_owned()
          .replace(".json", "")
      })
    })
    .collect::<Result<Vec<_>, io::Error>>();

  match entries {
    Ok(content) => serde_json::json!(content),
    Err(_) => serde_json::json!({
        "error": "Database is pretty empty... Maybe some sort of error occurred?",
    }),
  }
}

pub fn json_view(module: String, location: String) -> Value {
  let inner_location: String = format!("./{}/{}.json", module, location);
  let file = File::open(inner_location);
  let mut contents = String::new();

  match file {
    Ok(mut content) => {
      content.read_to_string(&mut contents).unwrap();
      serde_json::from_str(&contents).unwrap()
    }
    Err(error) => serde_json::json!({
        "error": error.to_string(),
    }),
  }
}
