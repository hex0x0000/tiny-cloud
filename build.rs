use minifier::{css, js};
use std::env;
use std::fs;
use std::path::PathBuf;

fn handle_file(file: PathBuf, out_dir: PathBuf) {
    let path_str = file.to_str().expect("Invalid path UTF-8");

    // If it is an image or binary data it is copied
    if path_str.ends_with("256.png") || path_str.ends_with(".ico") {
        let mut new_file_path = out_dir.clone();
        new_file_path.push(&file);
        let new_path_str = new_file_path.to_str().expect("Invalid path UTF-8");
        fs::copy(&file, &new_file_path)
            .unwrap_or_else(|_| panic!("Failed to copy file from {path_str} to {new_path_str}"));
        return;
    }

    // If it is a Web File it is minified and then written
    // If it's none of them, file is ignored
    let minified: String = if let Ok(file_content) = fs::read_to_string(&file) {
        if path_str.ends_with(".css") {
            css::minify(&file_content)
                .unwrap_or_else(|_| panic!("Failed to minify CSS file at {path_str}"))
                .to_string()
        } else if path_str.ends_with(".js") {
            js::minify(&file_content).to_string()
        } else {
            return;
        }
    } else {
        return;
    };
    let mut new_file_path = out_dir.clone();
    new_file_path.push(&file);
    fs::write(&new_file_path, minified).unwrap_or_else(|_| {
        panic!(
            "Failed to write minified file {}",
            new_file_path.to_str().unwrap_or(path_str)
        )
    });
}

fn handle_directory(directory: PathBuf, out_dir: PathBuf) {
    let mut new_dir = out_dir.clone();
    let path_str = directory.to_str().unwrap_or("directory");
    new_dir.push(&directory);
    fs::create_dir_all(&new_dir).unwrap_or_else(|_| panic!("Failed to create {path_str}"));
    for direntry in fs::read_dir(&directory)
        .unwrap_or_else(|_| panic!("Failed to read files of {path_str}"))
        .flatten()
    {
        if let Ok(file_type) = direntry.file_type() {
            if file_type.is_dir() {
                handle_directory(direntry.path(), out_dir.clone());
            } else if file_type.is_file() {
                handle_file(direntry.path(), out_dir.clone());
            }
        }
    }
}

fn main() {
    let out_dir =
        PathBuf::from(env::var_os("OUT_DIR").expect("Failed to get OUT_DIR env variable"));
    handle_directory(PathBuf::from("assets"), out_dir);

    println!("cargo:rerun-if-changed=assets");
}
