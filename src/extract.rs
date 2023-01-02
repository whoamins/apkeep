use std::fs;
use std::path::Path;
use std::thread::sleep;
use std::process::{Command, Output};

pub fn extract_apk(path: &str) {
    let mut all_apk_files = Vec::new();

    for file in fs::read_dir(path).unwrap() {
        let filename = file.expect("").path().file_name().unwrap().to_string_lossy().into_owned();

        if filename.ends_with("apk") {
            all_apk_files.push(path.to_owned() + "/" + &filename)
        }
    }

    for apk_file in all_apk_files {
        // println!("\n{}\n", apk_file);
        let mut file = Command::new("apktool")
            .arg("d")
            .arg(apk_file)
            .spawn()
            .expect("apktool command failed to start",);

        file.wait().expect("TODO: panic message");
    }
}