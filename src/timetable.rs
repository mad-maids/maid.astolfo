use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::{fs, io};

pub fn timetable_list(location: String) -> Value {
  let inner_location: String = format!("./timetable/{}", location);
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

  let text = match entries {
    Ok(content) => serde_json::json!(content),
    Err(_) => serde_json::json!({
        "error": "Database is pretty empty... Maybe some sort of error occurred?",
    }),
  };
  return text;
}

pub fn timetable_view(location: String) -> Value {
  let inner_location: String = format!("./timetable/{}.json", location);
  let file = File::open(inner_location);
  let mut contents = String::new();

  let text = match file {
    Ok(mut content) => {
      content.read_to_string(&mut contents).unwrap();
      serde_json::from_str(&contents).unwrap()
    }
    Err(error) => serde_json::json!({
        "error": error.to_string(),
    }),
  };
  return text;
}
