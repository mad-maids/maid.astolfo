use log::{error, info};
use std::path::Path;
use std::{fs, io};

fn check(path: &str) -> bool {
  Path::new(path).exists()
}

pub async fn health() {
  let target =
    "https://codeload.github.com/mad-maids/maid.table/zip/refs/heads/main";
  if check("./timetable") {
    info!("Timetable is ok!")
  } else {
    error!("Timetable doesn't exist");
    let mut dumpfile = tempfile::tempfile().unwrap();
    let resp = reqwest::get(target)
      .await
      .expect("Problems with Internet connectivity!")
      .bytes()
      .await
      .expect("Can't convert source into bytes!");
    io::copy(&mut resp.as_ref(), &mut dumpfile)
      .expect("Failed to copy content");
    let mut zip = zip::ZipArchive::new(dumpfile).unwrap();

    for i in 0..zip.len() {
      let mut file = zip.by_index(i).unwrap();
      let out_path = match file.enclosed_name() {
        Some(path) => path.to_owned(),
        None => continue,
      };

      if file.name().ends_with(".json") && file.name().contains("data") {
        println!("{}", file.name());
        let mut outfile =
          fs::File::create(format!("./timetable/{}", out_path.to_owned()))
            .unwrap();
        io::copy(&mut file, &mut outfile).unwrap();
      }
    }

    // Extract files inside folder
    // zip.extract("./timetable").expect("Couldn't extract...");

    // Works only with a file
    // copy("./timetable/maid.table-main/data", "./timetable.bak")
    //   .expect("Can't move files");

    // let mut out = File::create("timetable.zip").expect("Failed to create file");
  }
}
