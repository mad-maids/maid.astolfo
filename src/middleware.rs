use std::{fs, io};
use serde_json::Value;
use actix_files::Files;

pub fn timetable_list(location: String) -> Value {
    let inner_location: String = format!("./timetable/{}", location);
    let entries = fs::read_dir(&inner_location)
        .unwrap()
        .map(|res| res.map(|e| e.path().file_name().unwrap().to_str().unwrap().to_owned()))
        .collect::<Result<Vec<_>, io::Error>>();

    let text = match entries {
        Ok(content) => serde_json::json!(content),
        Err(error) => serde_json::json!({
            "error": error.to_string(),
        }),
    };
    return text;
}

pub fn timetable_view(location: String) {

}
