extern crate directories;
extern crate serde;

use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    library_directory: PathBuf,
}

const SETTINGS_FILE: &str = "settings.json";

fn create_settings() -> Box<UserData> {
    let user_data = UserData {
        library_directory: PathBuf::new()
    };

    Box::new(user_data)
}

pub fn load_settings() -> Box<UserData> {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "wford", "Ryuuji") {
        let dir = proj_dirs.config_dir();
        let complete_file_path = dir.join(Path::new(SETTINGS_FILE));
        let display = complete_file_path.display();

        let mut file = match File::open(&complete_file_path) {
            Err(_why) => return create_settings(),
            Ok(file) => file,
        };

        let mut file_contents = String::new();
        match file.read_to_string(&mut file_contents) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("{} contains:\n{}", display, file_contents),
        }

        let settings: UserData = serde_json::from_str(&file_contents).unwrap();
        Box::new(settings)
    } else {
        panic!("No valid configuration directory")
    }
}

pub fn save_settings(user_data: &Box<UserData>) {
    let serialized = serde_json::to_string(user_data).unwrap();

    if let Some(proj_dirs) = ProjectDirs::from("dev", "wford", "Ryuuji") {
        let dir = proj_dirs.config_dir();
        let complete_file_path = dir.join(Path::new(SETTINGS_FILE));
        let display = complete_file_path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(serialized.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => {}
        }
    } else {
        panic!("No valid configuration directory")
    }
}