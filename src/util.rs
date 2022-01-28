use serde_json::Value;
use std::fs::Metadata;
use std::io::{Error, Read};
use std::{fs, io};

pub fn read_dir(location: String) -> Result<Vec<String>, Error> {
    let inner_location: String = format!("./timetable/{}", location);
    let entries = fs::read_dir(&inner_location)?
        .map(|res| res.map(|e| e.path().file_name().unwrap().to_str().unwrap().to_owned()))
        .collect::<Result<Vec<_>, io::Error>>();

    return entries;
}

pub fn read_file(location: String) -> String {
    let mut s = String::new();
    std::fs::File::open(location);
    return s;
}

pub enum ReadContents {
    File(String),
    Folder(Vec<String>),
}

pub fn read(is_folder: bool, location: String) -> ReadContents {
    return if is_folder {
        ReadContents::Folder(read_dir(location).unwrap())
    } else {
        ReadContents::File(read_file(location))
    };
}

pub fn json(ctx: ReadContents) -> Value {
    if std::any::type_name::<ctx>() == ReadContents::File {

    }
    return serde_json::json!(ctx);
}
