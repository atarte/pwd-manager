use std::{fs, fs::File, path::Path};

use serde::{Serialize, de::DeserializeOwned};

pub mod user_selection;
pub mod user_creation;
pub mod login;

pub trait Init {
    fn init() -> Self;
}

// trait Json {
//     fn export_json();
//     fn import_json() -> Self;
// }

pub fn export_json<T: Serialize>(path: &str, data: &T) {
    let file: File = File::create(path).unwrap();

    serde_json::to_writer_pretty(&file, data).unwrap();
}

pub fn import_json<T: Init + Serialize>(path: &str) -> T where T: DeserializeOwned {
    if Path::new(path).exists() {
        let data = T::init();

        export_json(path, &data);

        return data
    }

    let file_string: String = fs::read_to_string(path)
        .expect("ok");

    println!("{}", file_string);

    serde_json::from_str(&file_string)
        .expect("pas ok")
}

