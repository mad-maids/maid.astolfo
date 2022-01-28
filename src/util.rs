use serde_json::Value;
use std::io::Error;
use std::path::PathBuf;
use std::{fs, io};

pub fn read_dir(location: String) -> Result<Vec<PathBuf>, Error> {
    let inner_location: String = format!("./timetable/{}", location);
    let entries = fs::read_dir(inner_location)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>();

    return entries;
}

pub fn json(locations: Result<Vec<PathBuf>, Error>) -> Value {
    let text = match locations {
        Ok(content) => serde_json::json!(content),
        Err(error) => serde_json::json!({
            "error": error.to_string(),
        }),
    };
    return text;
}
