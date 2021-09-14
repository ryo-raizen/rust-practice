extern crate path_absolutize;
use std::path::Path;
use path_absolutize::*;
use webbrowser;

fn main() {
    let path = format!("{}{}", "file://", Path::new("./static/index.html").absolutize().unwrap().to_str().unwrap());
    if webbrowser::open(&path).is_ok() {
        println!("local html open!");
    }
}
