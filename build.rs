extern crate winres;

fn main() {
  #[cfg(target_os = "windows")]
  {
    let mut res = winres::WindowsResource::new();
    res.set_icon("astolfo.ico");
    res.set_language(winapi::um::winnt::MAKELANGID(
      winapi::um::winnt::LANG_ENGLISH,
      winapi::um::winnt::SUBLANG_ENGLISH_US,
    ));
    res.compile().unwrap();
  }
}
