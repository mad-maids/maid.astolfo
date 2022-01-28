use serde_json::Value;
use std::io::Error;
use std::{fs, io};

pub fn read_dir(location: String) -> Result<Vec<String>, Error> {
    let inner_location: String = format!("./timetable/{}", location);
    let clear: String = format!("./timetable/{}", location);
    let entries = fs::read_dir(&inner_location)?
        .map(|res| {
            res.map(|e| {
                e.path()
                    .display()
                    .to_string()
                    .replace("\\", "/")
                    .replace(&clear, "")
                    .replace("/", "")
            })
        })
        .collect::<Result<Vec<_>, io::Error>>();

    return entries;
}

pub fn json(ctx: Result<Vec<String>, Error>) -> Value {
    let text = match ctx {
        Ok(content) => serde_json::json!(content),
        Err(error) => serde_json::json!({
            "error": error.to_string(),
        }),
    };
    return text;
}
