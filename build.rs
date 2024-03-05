fn main() {
  if cfg!(target_os = "windows") {
    extern crate winresource;
    let mut res = winresource::WindowsResource::new();
    res.set_icon("icon.ico");
    res.compile().unwrap();
  }
}
