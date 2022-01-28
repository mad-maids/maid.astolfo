use crate::util;
use serde_json::Value;

pub fn timetable(location: String) -> Value {
    let some = util::read_dir(location);
    let json = util::json(some);
    return json;
}

// pub fn timetable_index() {}
