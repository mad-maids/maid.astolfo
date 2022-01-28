use crate::util;
use serde_json::Value;

pub fn timetable_list(location: String) -> Value {
    let args: Vec<&str> = location.split("/").collect();
    let ctx = util::read(args.len() < 2, location);
    let json = util::json(ctx);
    return json;
}

// pub fn timetable_index() {}
