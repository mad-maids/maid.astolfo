use serde_json::Value;
use crate::util;

pub fn timetable(location: String) -> Value {
    let some = util::read_dir(location);
    let json = util::json(some);
    return json;
}