use std::{fs, io};
use std::io::Error;
use std::path::{PathBuf};
use serde_json::Value;

fn read(location: String) -> Result<Vec<PathBuf>, Error> {
    let inner_location: String = format!("./timetable/{}", location);
    let entries = fs::read_dir(inner_location)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>();

    return entries;
}

fn json(locations: Result<Vec<PathBuf>, Error>) -> Value {
    let text = serde_json::json!(locations.unwrap());
    return text;
}

pub fn timetable(location: String) -> Value {
    let some = read(location);
    let json = json(some);
    return json;
}