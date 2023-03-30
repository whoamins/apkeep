use std::{fs};
use std::process::Command;


pub fn extract_apk(path: &str) {
    let mut all_apk_files = Vec::new();

    for file in fs::read_dir(path).unwrap() {
        let filename = file.expect("").path().file_name().unwrap().to_string_lossy().into_owned();

        if filename.ends_with("apk") {
            all_apk_files.push(path.to_owned() + "/" + &filename)
        }
    }

    for apk_file in all_apk_files {
        let mut folder_name: &str = ".";

        if apk_file.ends_with("xapk") {
            folder_name = apk_file.split(".xapk").collect::<Vec<_>>()[0];
            let name = folder_name.split("/").collect::<Vec<_>>().last().unwrap().clone();
            let apk_name = name.split("@").collect::<Vec<_>>().first().unwrap().clone();
            if let xapk_filename = (folder_name.to_owned() + "/unknown/" + apk_name).as_str() {
                let _ = Command::new("apktool")
                    .arg("d")
                    .arg(apk_file.clone())
                    .arg("-o")
                    .arg(folder_name)
                    .spawn()
                    .expect("apktool command failed to start",).wait();

                let _ = Command::new("apktool")
                    .arg("d")
                    .arg(xapk_filename.to_owned() + ".apk")
                    .arg("-o")
                    .arg(folder_name.to_owned() + "/out/")
                    .spawn()
                    .expect("apktool command failed to start",);
            }
        } else if apk_file.ends_with("apk") {
            folder_name = apk_file.split(".apk").collect::<Vec<_>>()[0];
        }

        let _ = Command::new("apktool")
            .arg("d")
            .arg(apk_file.clone())
            .arg("-o")
            .arg(folder_name)
            .spawn()
            .expect("apktool command failed to start",);
    }
}