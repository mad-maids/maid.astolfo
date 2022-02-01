use std::fs;
use std::path::Path;

fn check(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn health() -> () {
    if check("./timetable") {
        println!("{}", "Timetable is ok")
    } else {
        panic!("Timetable doesn't exist")
    }
}
