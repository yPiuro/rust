fn main() {
  if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("icon.ico");
    res.compile().unwrap();
  }
}
